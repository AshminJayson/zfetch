// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use levenshtein::levenshtein;
use std::collections::HashMap;

fn fuzzy_match<'a>(input: &str, candidates: &'a [&'a str], threshold: usize) -> Option<Box<str>> {
    candidates.iter().find_map(|&candidate| {
        let distance = levenshtein(input, candidate);
        if distance <= threshold {
            Some(candidate.to_owned().into_boxed_str())
        } else {
            None
        }
    })
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
    let fuzzy_candidates: Vec<Box<str>> = key_value_pairs
        .keys()
        .filter_map(|&k| fuzzy_match(key, &[k], 10))
        .collect();

    if fuzzy_candidates.len() == 1 {
        let matched_key = fuzzy_candidates[0].clone();
        return Some(key_value_pairs[&*matched_key].to_owned().into_boxed_str());
    } else {
        None
    }
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
