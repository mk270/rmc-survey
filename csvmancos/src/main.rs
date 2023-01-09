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

fn exclude_by_name(c: &LegalEntity,
                   excluded_names: &Vec<String>) -> bool
{
    !matches_any_substring(&c.name, excluded_names)
}

// TODO: check we've got the test in the right sense
fn relevant_sic_codes(c: &LegalEntity) -> bool
{
    if c.sic_codes.iter().all(|i| i != &"68320" && i != &"98000") {
        return false;
    }
    true
}

fn include_by_name(c: &LegalEntity,
          included_names: &Vec<String>)
          -> bool {
    if matches_any_substring(&c.name, included_names) {
        true
    } else if c.name.contains(" HOUSE ") && c.name.contains("MANAGEMENT") {
        true
    } else {
        false
    }
}

fn find_rmcs() -> Result<(), Box<dyn Error>> {
    let excluded_text = include_str!("exclude_names.txt");
    let excluded_names = string_column_to_vec(excluded_text);
    let included_text = include_str!("include_names.txt");
    let included_names = string_column_to_vec(included_text);

    let mut reader = csv::Reader::from_reader(io::stdin());
    let mut writer = csv::Writer::from_writer(io::stdout());

    for rmc in reader.deserialize::<LegalEntityRecord>()
        .map(Result::unwrap)
        .map(|ler| LegalEntity::new(ler))
        .filter_map(|opt_le| opt_le)
        .filter(|le| examine_entity_type(le.category))
        .filter(|le| exclude_by_name(le, &excluded_names))
        .filter(|le| relevant_sic_codes(le))
        .filter(|le| include_by_name(le, &included_names))
    {
        writer.write_record(rmc.to_vec())?;
        writer.flush()?
    }

    Ok(())
}

fn main() {
    if let Err(err) = find_rmcs() {
        println!("error: {}", err);
        process::exit(1);
    }
}
