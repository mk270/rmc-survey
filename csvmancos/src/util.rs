/*
    rmc-survey, Tools for surveying the population of RMCs, by Martin Keegan
    
    To the extent (if any) permissible by law, Copyright (C) 2023  Martin Keegan
    
    This programme is free software; you may redistribute and/or modify it under
    the terms of the Apache Software Licence v2.0.
*/
use csv::ReaderBuilder;

pub fn matches_any_substring(haystack: &String, needles: &Vec<String>) -> bool {
    for needle in needles {
        if haystack.contains(needle) {
            return true
        }
    }
    false
}

// Grab the first column of a CSV file
pub fn string_column_to_vec(data : &'static str) -> Vec<String> {
    ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data.as_bytes())
        .records().map(|record| {
            record.unwrap()[0].to_string()
        }).collect()
}
