// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lazy_static::lazy_static;
use levenshtein::levenshtein;
use std::{collections::HashMap, sync::Mutex};

lazy_static! {
    static ref KEY_VALUE_PAIRS: Mutex<HashMap<&'static str, &'static str>> = {
        let mut map = HashMap::new();
        map.insert("username", "Ashmin Jayson");
        map.insert(
            "Address Line 1",
            "Edappulavan House Kottappady PO Kothamanagalam",
        );
        map.insert("Mother's name", "Gracy Jayson");
        map.insert("Phone number", "+1234567890");
        map.insert("Email", "ashmin@example.com");
        map.insert("Occupation", "Software Developer");
        map.insert("Aadhar Number", "212980970069");
        Mutex::new(map)
    };
}

fn levenshtein_up_to_min_length(a: &str, b: &str) -> usize {
    let min_length = a.len().min(b.len());
    levenshtein(&a[..min_length], &b[..min_length])
}

fn match_key<'a>(key: &str) -> Vec<(Box<str>, Box<str>)> {
    if let Ok(map) = KEY_VALUE_PAIRS.lock() {
        if let Some(value) = map.get(key).copied() {
            return vec![(
                key.to_owned().into_boxed_str(),
                value.to_owned().into_boxed_str(),
            )];
        }

        // Fuzzy match with a threshold of 5 (adjust as needed)

        let keys: Vec<&str> = map.keys().map(|k| *k).collect();

        let distanced_candidates: Vec<(&str, usize)> = keys
            .iter()
            .filter_map(|&k| {
                let distance = levenshtein_up_to_min_length(key, k);
                if distance <= 3 {
                    Some((k, distance))
                } else {
                    None
                }
            })
            .collect();

        // println!("Distanced candidates: {:?}", distanced_candidates);

        // Sort fuzzy candidates by Levenshtein distance
        let mut fuzzy_candidates: Vec<&str> =
            distanced_candidates.iter().map(|&(s, _)| s).collect();

        fuzzy_candidates.sort_by(|&a, &b| {
            levenshtein_up_to_min_length(key, a).cmp(&levenshtein_up_to_min_length(key, b))
        });

        let matched_values: Vec<(Box<str>, Box<str>)> = fuzzy_candidates
            .iter()
            .map(|&k| {
                (
                    k.to_owned().into_boxed_str(),
                    map[k].to_owned().into_boxed_str(),
                )
            })
            .collect();

        if !matched_values.is_empty() {
            println!("Matched values: {:?}", matched_values);
            return matched_values;
        }
    }

    vec![]
}

#[tauri::command]
fn matcher(key: &str) -> Vec<(String, String)> {
    let matches: Vec<(Box<str>, Box<str>)> = match_key(key);
    let result: Vec<(String, String)> = matches
        .iter()
        .map(|&(ref k, ref v)| (k.to_string(), v.to_string()))
        .collect();

    println!("Matches: {:?}", matches);
    if !matches.is_empty() {
        println!("Match found! for Key {} Value: {:?}", key, matches);
    } else {
        println!("No match found for key: {}", key);
    }

    result
}

fn insert_key_value(key: String, value: String) {
    if let Ok(mut map) = KEY_VALUE_PAIRS.lock() {
        let key_static: &'static str = Box::leak(key.into_boxed_str());
        let value_static: &'static str = Box::leak(value.into_boxed_str());
        map.insert(key_static, value_static);
    }
}

#[tauri::command]
fn addrecord(key: &str, value: &str) -> String {
    println!("Key: {} Value: {}", key, value);
    format!("Key: {} Value: {}", key, value);

    // Add the key-value pair to the key_value_pairs HashMap
    insert_key_value(key.to_string(), value.to_string());
    // Return a success message
    format!("Added record: Key: {}, Value: {}", key, value)
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, matcher, addrecord])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
