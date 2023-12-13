// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lazy_static::lazy_static;
use levenshtein::levenshtein;
use std::{collections::HashMap, sync::Mutex};

mod db;

lazy_static! {
    static ref KEY_VALUE_PAIRS: Mutex<HashMap<&'static str, &'static str>> = {
        let mut map = HashMap::new();

        let key_value_pairs = db::load_key_value_pairs();
        for (key, value) in key_value_pairs {
            let key_static: &'static str = Box::leak(key.into_boxed_str());
            let value_static: &'static str = Box::leak(value.into_boxed_str());
            map.insert(key_static, value_static);
        }

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
            // println!("Matched values: {:?}", matched_values);
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

    // println!("Matches: {:?}", matches);
    // if !matches.is_empty() {
    //     println!("Match found! for Key {} Value: {:?}", key, matches);
    // } else {
    //     println!("No match found for key: {}", key);
    // }

    result
}

fn insert_key_value(key: String, value: String) {
    if let Ok(mut map) = KEY_VALUE_PAIRS.lock() {
        let key_static: &'static str = Box::leak(key.into_boxed_str());
        let value_static: &'static str = Box::leak(value.into_boxed_str());
        map.insert(key_static, value_static);
        db::insert_key_value(key_static, value_static);
    }
}

fn delete_key_value(key: &str) -> Option<String> {
    if let Ok(mut map) = KEY_VALUE_PAIRS.lock() {
        if let Some(value) = map.remove(key) {
            db::delete_key_value(key);
            return Some(value.to_string());
        }
    }
    None
}

#[tauri::command]
fn addrecord(key: &str, value: &str) -> String {
    insert_key_value(key.to_string(), value.to_string());
    // println!("Added record: Key: {}, Value: {}", key, value);
    format!("Added record: Key: {}, Value: {}", key, value)
}

#[tauri::command]
fn deleterecord(key: &str) -> String {
    format!("Key: {}", key);

    if let Some(value) = delete_key_value(key) {
        // println!("Deleted record: Key: {}, Value: {}", key, value);
        format!("Deleted record: Key: {}, Value: {}", key, value)
    } else {
        // println!("No record found for key: {}", key.to_string());
        format!("No record found for key: {}", key)
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            matcher,
            addrecord,
            deleterecord,
            // db::getappdatapath
        ])
        .setup(|_app| {
            db::init();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
