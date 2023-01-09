/*
    rmc-survey, Tools for surveying the population of RMCs, by Martin Keegan
    
    To the extent (if any) permissible by law, Copyright (C) 2023  Martin Keegan
    
    This programme is free software; you may redistribute and/or modify it under
    the terms of the Apache Software Licence v2.0.
*/

use std::error::Error;
use std::io;
use std::process;

pub use self::legal_entity::*;
mod legal_entity;
pub use self::util::*;
mod util;

/* are entities of this type worth examining further?

   i.e., if the entity is, say, a CIO, should we bother investigating
   further?
*/
fn examine_entity_type(entity_type: EntityType) -> bool {
    match entity_type {
        /* short circuit some entity types that are definitely irrelevant */
        EntityType::Plc => false,
        EntityType::CIC => false,
        EntityType::CIO => false,
        EntityType::RegSoc => false,
        EntityType::Recognised => false,
        /* otherwise, it's worth investigating further */
        _ => true
    }
}

struct RMC {
    name: String,
    number: String,
    description: String
}

impl RMC {
    fn new(name: String, number: String, description: String) -> RMC {
        RMC {
            name, number, description
        }
    }

    fn to_vec(&self) -> Vec<String> {
        vec![
            self.number.clone(),
            self.description.clone(),
            self.name.clone()
        ]
    }
}

fn get_rmc(c: &legal_entity::LegalEntity,
          excluded_names: &Vec<String>,
          included_names: &Vec<String>)
          -> Option<RMC> {
    let t = entity_type_of_str(&c.company_type);
    match t {
        None => {
            eprintln!("Unrecognised entity type: {}", &c.company_type);
            return None;
        },
        Some(et) => {
            if !examine_entity_type(et) {
                return None;
            }
        }
    }

    let name = &c.name;

    if matches_any_substring(name, excluded_names) {
        return None;
    }

    let sics : Vec<String> = sics_of_one_record(&c);
    if !sics.contains(&"68320".to_string()) &&
        !sics.contains(&"98000".to_string()) {
        return None;
    }

    let rmc = RMC::new(c.name.clone(),
                       c.number.clone(),
                       t.unwrap().to_string());

    if matches_any_substring(name, included_names) {
        Some(rmc)
    } else if name.contains(" HOUSE ") && name.contains("MANAGEMENT") {
        Some(rmc)
    } else {
        None
    }
}

fn find_rmcs() -> Result<(), Box<dyn Error>> {
    let excluded_text = include_str!("exclude_names.txt");
    let excluded_names = string_column_to_vec(excluded_text);
    let included_text = include_str!("include_names.txt");
    let included_names = string_column_to_vec(included_text);

    let mut counter = 0;
    // this knob controls how often we flush/report
    const INTERVAL : usize = 100000;

    let mut reader = csv::Reader::from_reader(io::stdin());
    let mut writer = csv::Writer::from_writer(io::stdout());

    for result in reader.deserialize::<LegalEntity>() {
        let record: LegalEntity = result?;
        counter += 1;

        if 0 == counter % INTERVAL {
            eprintln!("{}", counter);
            writer.flush()?
        }

        match get_rmc(&record, &excluded_names, &included_names) {
            None => continue,
            Some(rmc) => writer.write_record(rmc.to_vec())?
        }
    }
    writer.flush()?; // otiose?
    Ok(())
}

fn main() {
    if let Err(err) = find_rmcs() {
        println!("error: {}", err);
        process::exit(1);
    }
}
