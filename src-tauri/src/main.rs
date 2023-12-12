// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use levenshtein::levenshtein;
use std::collections::HashMap;

fn levenshtein_up_to_min_length(a: &str, b: &str) -> usize {
    let min_length = a.len().min(b.len());
    levenshtein(&a[..min_length], &b[..min_length])
}

fn match_key<'a>(key: &str) -> Vec<Box<str>> {
    let mut key_value_pairs = HashMap::new();
    key_value_pairs.insert("username", "Ashmin Jayson");
    key_value_pairs.insert(
        "Address Line 1",
        "Edappulavan House Kottappady PO Kothamanagalam",
    );
    key_value_pairs.insert("Mother's name", "Gracy Jayson");
    key_value_pairs.insert("Phone number", "+1234567890");
    key_value_pairs.insert("Email", "ashmin@example.com");
    key_value_pairs.insert("Occupation", "Software Developer");

    if let Some(value) = key_value_pairs.get(key).copied() {
        return vec![value.to_owned().into_boxed_str()];
    }

    // Fuzzy match with a threshold of 5 (adjust as needed)

    let keys: Vec<&str> = key_value_pairs.keys().map(|k| *k).collect();

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
    let mut fuzzy_candidates: Vec<&str> = distanced_candidates.iter().map(|&(s, _)| s).collect();

    fuzzy_candidates.sort_by(|&a, &b| {
        levenshtein_up_to_min_length(key, a).cmp(&levenshtein_up_to_min_length(key, b))
    });

    let matched_values: Vec<Box<str>> = fuzzy_candidates
        .iter()
        .map(|&k| key_value_pairs[k].to_owned().into_boxed_str())
        .collect();

    if matched_values.len() > 0 {
        println!("Matched values: {:?}", matched_values);
        return matched_values;
    }

    vec![]
}

#[tauri::command]
fn matcher(key: &str) -> Vec<String> {
    let matches: Vec<Box<str>> = match_key(key);
    let result: Vec<String> = matches.iter().map(|s| s.to_string()).collect();

    println!("Matches: {:?}", matches);
    if matches.len() > 0 {
        println!("Match found! Key {} Value: {:?}", key, matches);
    } else {
        println!("No match found for key: {}", key);
    }

    result
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![matcher])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
