// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use levenshtein::levenshtein;
use std::collections::HashMap;

fn levenshtein_up_to_min_length(a: &str, b: &str) -> usize {
    let min_length = a.len().min(b.len());
    levenshtein(&a[..min_length], &b[..min_length])
}

fn match_key<'a>(key: &str) -> Option<Box<str>> {
    let mut key_value_pairs = HashMap::new();
    key_value_pairs.insert("username", "Ashmin Jayson");
    key_value_pairs.insert(
        "Address Line 1",
        "Edappulavan House Kottappady PO Kothamanagalam",
    );
    key_value_pairs.insert("Mother's name", "Gracy Jayson");

    if let Some(value) = key_value_pairs.get(key).copied() {
        return Some(value.to_owned().into_boxed_str());
    }

    // Fuzzy match with a threshold of 5 (adjust as needed)

    let keys: Vec<&str> = key_value_pairs.keys().map(|k| *k).collect();

    let mut fuzzy_candidates: Vec<&str> = keys
        .iter()
        .filter(|&k| levenshtein_up_to_min_length(key, k) <= 5)
        .cloned()
        .collect();

    // Sort fuzzy candidates by Levenshtein distance
    fuzzy_candidates.sort_by(|&a, &b| {
        levenshtein_up_to_min_length(key, a).cmp(&levenshtein_up_to_min_length(key, b))
    });

    if !fuzzy_candidates.is_empty() {
        let matched_key = fuzzy_candidates[0];
        return Some(key_value_pairs[matched_key].to_owned().into_boxed_str());
    }

    None
}

#[tauri::command]
fn matcher(key: &str) -> String {
    if let Some(value) = match_key(key) {
        println!("Match found! Value: {}", value);
        value.to_string()
    } else {
        println!("No match found for key: {}", key);
        format!("No match found for key: {}", key)
    }
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
