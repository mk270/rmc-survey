/*
    rmc-survey, Tools for surveying the population of RMCs, by Martin Keegan
    
    To the extent (if any) permissible by law, Copyright (C) 2023  Martin Keegan
    
    This programme is free software; you may redistribute and/or modify it under
    the terms of the Apache Software Licence v2.0.
*/
use std::error::Error;
use std::io;
use std::io::BufRead;
use serde_json::{Value};

#[derive(Debug)]
enum PSC {
    Individual,
    Corporate,
    Unidentified
}

fn process_lines() {
    let mut counter = 0;
    let mut outputs = 0;
    let mut writer = csv::Writer::from_writer(io::stdout());

    for line in io::stdin().lock().lines() {
        counter += 1;
        if 0 == (counter % 100000) {
            eprintln!("reader: {}", counter);
        }

        let v : Value = serde_json::from_str(&line.unwrap()).unwrap();
        let kind = v["data"]["kind"].as_str().unwrap();

        if kind == "totals#persons-of-significant-control-snapshot" {
            continue;
        }

        let num = &v["company_number"].as_str();
        let number = match num {
            None => {
                eprintln!("No company number: {}", v);
                continue
            },
            Some(n) => n
        };
    
        let number_prefix = &number[0..2];

        /* filter out entities whose ID starts with two digits, otherwise
           interrogate what the two initial characters are. The things
           beginning with digits are England/Wales registered companies */
        match number_prefix.parse::<u32>() {
            Err(_) => match number_prefix {
                /* entities that can't be RMCs, e.g., non-companies */
                "SO" | "OC" | "SL" | "NC" | "SE" | "SG" | "OE" => continue,

                /* entities that must be companies but outside E&W */
                "NI" | "SC" | "R0" => {},

                /* seriously? */
                "ZC" | "SZ" => {},

                /* unrecognised entities */
                _ => { panic!("unrecognised prefix: {}", number); }
            },
            Ok(_) => {}
        }

        let data_obj = v["data"].as_object().unwrap();

        /* discard PSC statements that have ceased to apply */
        if data_obj.contains_key("ceased_on") ||
           data_obj.contains_key("ceased") {
               continue;
        }

        let _payload = match kind {
            "super-secure-person-with-significant-control" => {
                PSC::Unidentified
            }
            "individual-person-with-significant-control" => {
                PSC::Individual
            },
            "legal-person-person-with-significant-control" |
            "corporate-entity-person-with-significant-control" => {
                PSC::Corporate
            },
            "persons-with-significant-control-statement" => {
                let stmt = data_obj["statement"].as_str().unwrap();
                match stmt {
                    "no-individual-or-entity-with-signficant-control" =>
                        continue,
                    "psc-details-not-confirmed" |
                    "psc-exists-but-not-identified" |
                    "psc-contacted-but-no-response" |
                    "restrictions-notice-issued-to-psc" |
                    "psc-has-failed-to-confirm-changed-details"
                        => PSC::Unidentified,
                    "steps-to-find-psc-not-yet-completed" =>
                        continue, /* tiresomely - this is actually unknown */
                    s => panic!("unrecognised statement: {}\n{}", s, v)
                }
            },
            "exemptions" => {
                continue
            },
            _ => {
                eprintln!("{}", v);
                panic!("wrong thing")
            }
        };

        writer.write_record(&[number]).unwrap();

        outputs += 1;
        if 0 == outputs % 100000 {
            eprintln!("dumper: {}", outputs);
            writer.flush().unwrap();
        }
    }
}

fn main() -> Result<(), Box<dyn Send + Error>> {
    process_lines();
    Ok(())
}
