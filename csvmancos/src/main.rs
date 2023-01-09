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
fn examine_entity_type(entity_type: Option<EntityType>) -> bool {
    match entity_type {
        /* short circuit some entity types that are definitely irrelevant */
        Some(EntityType::Plc) => false,
        Some(EntityType::CIC) => false,
        Some(EntityType::CIO) => false,
        Some(EntityType::RegSoc) => false,
        Some(EntityType::Recognised) => false,
        /* otherwise, it's worth investigating further */
        _ => true
    }
}

fn is_rmc(c: &legal_entity::LegalEntity,
          excluded_names: &Vec<String>,
          included_names: &Vec<String>)
          -> bool {
    let t = entity_type_of_str(&c.company_type);
    if !examine_entity_type(t) {
        return false
    }

    let name = &c.name;

    if matches_any_substring(name, excluded_names) {
        return false
    }

    let sics = sics_of_one_record(&c);
    if !(sics.contains(&"68320".to_string()) ||
         sics.contains(&"98000".to_string())) {
        return false
    }

    if matches_any_substring(name, included_names) {
        return true
    }
    if name.contains(" HOUSE ") && name.contains("MANAGEMENT") {
        return true
    }
    false
}

fn find_rmcs() -> Result<(), Box<dyn Error>> {
    let excluded_text = include_str!("exclude_names.txt");
    let excluded_names = string_column_to_vec(excluded_text);
    let included_text = include_str!("include_names.txt");
    let included_names = string_column_to_vec(included_text);

    let interval = 100000;
    let mut counter = 0;
    let mut rmcs = 0;

    let mut reader = csv::Reader::from_reader(io::stdin());
    let mut writer = csv::Writer::from_writer(io::stdout());

    for result in reader.deserialize::<LegalEntity>() {
        let record: LegalEntity = result?;
        counter += 1;

        if 0 == counter % interval {
            eprintln!("{}", counter);
            writer.flush()?
        }

        if !is_rmc(&record, &excluded_names, &included_names) {
            continue;
        }

        let entity_type = entity_type_of_str(&record.company_type);
        match entity_type {
            Some(_) => {},
            None => {
                eprintln!("Unrecognised entity type: {} @ {:?}",
                          record.company_type, record);
                continue;
            }
        }
        writer.write_record(&[record.number,
                              entity_type.unwrap().to_string(),
                              record.name])?;
        rmcs += 1;
    }
    eprintln!("#RMCs: {}", rmcs);
    Ok(())
}

fn main() {
    if let Err(err) = find_rmcs() {
        println!("error: {}", err);
        process::exit(1);
    }
}
