/*
    rmc-survey, Tools for surveying the population of RMCs, by Martin Keegan
    
    To the extent (if any) permissible by law, Copyright (C) 2023  Martin Keegan
    
    This programme is free software; you may redistribute and/or modify it under
    the terms of the Apache Software Licence v2.0.
*/
use std::fmt;
use serde::Deserialize;

/*
  Note that the fields in the actual CSV contain some infelicities:

    * some of the field names contain leading (or is it trailing?) spaces
    * some contain fullstops and other punctuation

  Accordingly we rename every single one of them manually.
*/

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct LegalEntityRecord {
    #[serde(rename="CompanyName")]
    pub name: String,
    #[serde(rename=" CompanyNumber")]
    pub number: String,
    #[serde(rename="RegAddress.CareOf")]
    unknown3: String,
    #[serde(rename="RegAddress.POBox")]
    unknown4: String,
    #[serde(rename="RegAddress.AddressLine1")]
    unknown5: String,
    #[serde(rename=" RegAddress.AddressLine2")]
    unknown6: String,
    #[serde(rename="RegAddress.PostTown")]
    unknown7: String,
    #[serde(rename="RegAddress.County")]
    unknown8: String,
    #[serde(rename="RegAddress.Country")]
    country: String,
    #[serde(rename="RegAddress.PostCode")]
    postcode: String,
    #[serde(rename="CompanyCategory")]
    pub company_type: String,
    #[serde(rename="CompanyStatus")]
    status: String,
    #[serde(rename="CountryOfOrigin")]
    unknown13: String,
    #[serde(rename="DissolutionDate")]
    unknown14: String,
    #[serde(rename="IncorporationDate")]
    incorporation_date: String,
    #[serde(rename="Accounts.AccountRefDay")]
    unknown16: String,
    #[serde(rename="Accounts.AccountRefMonth")]
    unknown17: String,
    #[serde(rename="Accounts.NextDueDate")]
    unknown18: String,
    #[serde(rename="Accounts.LastMadeUpDate")]
    unknown19: String,
    #[serde(rename="Accounts.AccountCategory")]
    last_accounts: String,
    #[serde(rename="Returns.NextDueDate")]
    unknown21: String,
    #[serde(rename="Returns.LastMadeUpDate")]
    unknown22: String,
    #[serde(rename="Mortgages.NumMortCharges")]
    unknown23: String,
    #[serde(rename="Mortgages.NumMortOutstanding")]
    unknown24: String,
    #[serde(rename="Mortgages.NumMortPartSatisfied")]
    unknown25: String,
    #[serde(rename="Mortgages.NumMortSatisfied")]
    unknown26: String,
    #[serde(rename="SICCode.SicText_1")]
    pub sic_code1: String,
    #[serde(rename="SICCode.SicText_2")]
    pub sic_code2: String,
    #[serde(rename="SICCode.SicText_3")]
    pub sic_code3: String,
    #[serde(rename="SICCode.SicText_4")]
    pub sic_code4: String,
    #[serde(rename="LimitedPartnerships.NumGenPartners")]
    unknown31: String,
    #[serde(rename="LimitedPartnerships.NumLimPartners")]
    unknown32: String,
    #[serde(rename="URI")]
    unknown33: String,
    #[serde(rename="PreviousName_1.CONDATE")]
    unknown34: String,
    #[serde(rename=" PreviousName_1.CompanyName")]
    unknown35: String,
    #[serde(rename=" PreviousName_2.CONDATE")]
    unknown36: String,
    #[serde(rename=" PreviousName_2.CompanyName")]
    unknown37: String,
    #[serde(rename="PreviousName_3.CONDATE")]
    unknown38: String,
    #[serde(rename=" PreviousName_3.CompanyName")]
    unknown39: String,
    #[serde(rename="PreviousName_4.CONDATE")]
    unknown40: String,
    #[serde(rename=" PreviousName_4.CompanyName")]
    unknown41: String,
    #[serde(rename="PreviousName_5.CONDATE")]
    unknown42: String,
    #[serde(rename=" PreviousName_5.CompanyName")]
    unknown43: String,
    #[serde(rename="PreviousName_6.CONDATE")]
    unknown44: String,
    #[serde(rename=" PreviousName_6.CompanyName")]
    unknown45: String,
    #[serde(rename="PreviousName_7.CONDATE")]
    unknown46: String,
    #[serde(rename=" PreviousName_7.CompanyName")]
    unknown47: String,
    #[serde(rename="PreviousName_8.CONDATE")]
    unknown48: String,
    #[serde(rename=" PreviousName_8.CompanyName")]
    unknown49: String,
    #[serde(rename="PreviousName_9.CONDATE")]
    unknown50: String,
    #[serde(rename=" PreviousName_9.CompanyName")]
    unknown51: String,
    #[serde(rename="PreviousName_10.CONDATE")]
    unknown52: String,
    #[serde(rename=" PreviousName_10.CompanyName")]
    unknown53: String,
    #[serde(rename="ConfStmtNextDueDate")]
    unknown54: String,
    #[serde(rename=" ConfStmtLastMadeUpDate")]
    unknown55: String,
}

/*
  The Companies House bulk data conflate a number of different types of legal
  entity with UK registered and unregistered companies. We want to be able to recognise,
  and *ignore* these extraneous entries.

  Even more egregiously, where a company is a Community Interest Company, this fact is
  mentioned *instead* of whether the company happens to be limited by shares or by
  guarantee(!)

  In any case, CICs are unlikely to be bona fide RMCs.
*/
#[derive(Copy, Clone, Debug)]
pub enum EntityType {
    Ltd,
    CLG,
    Unltd,
    CIC,
    RegSoc,
    CIO,
    Plc,
    Recognised
}

impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            EntityType::Ltd => "private limited company",
            EntityType::CLG => "private company limited by guarantee",
            EntityType::Unltd => "private unlimited company",
            EntityType::CIO => "charitable incorporated organisation",
            EntityType::CIC => "community interest company",
            EntityType::RegSoc => "registered society",
            EntityType::Plc => "public limited company",
            EntityType::Recognised => "recognised"
        };
        write!(f, "{}", s)
    }
}

pub fn entity_type_of_str(s: &str) -> Option<EntityType> {
    match s {
        "Private Limited Company"
            => Some(EntityType::Ltd),

        "PRI/LTD BY GUAR/NSC (Private, limited by guarantee, no share capital)"
            => Some(EntityType::CLG),

        "PRI/LBG/NSC (Private, Limited by guarantee, no share capital, use of 'Limited' exemption)"
            => Some(EntityType::CLG),

        "Private Unlimited"
            => Some(EntityType::Unltd),

        "Private Unlimited Company"
            => Some(EntityType::Unltd),

        "Community Interest Company"
            => Some(EntityType::CIC),

        "Charitable Incorporated Organisation"
            => Some(EntityType::CIO),

        "Scottish Charitable Incorporated Organisation"
            => Some(EntityType::CIO),

        "Registered Society"
            => Some(EntityType::RegSoc),

        "Limited Partnership"
            => Some(EntityType::Recognised),

        "Limited Liability Partnership"
            => Some(EntityType::Recognised),

        "Other company type"
            => Some(EntityType::Recognised),

        "Industrial and Provident Society"
            => Some(EntityType::Recognised),

        "Investment Company with Variable Capital"
            => Some(EntityType::Recognised),

        "Investment Company with Variable Capital(Umbrella)"
            => Some(EntityType::Recognised),

        "Royal Charter Company"
            => Some(EntityType::Recognised),

        "Scottish Partnership"
            => Some(EntityType::Recognised),

        "United Kingdom Economic Interest Grouping"
            => Some(EntityType::Recognised),

        "United Kingdom Societas"
            => Some(EntityType::Recognised),

        "Investment Company with Variable Capital (Securities)"
            => Some(EntityType::Recognised),

        "Old Public Company"
            => Some(EntityType::Recognised),

        "Other Company Type"
            => Some(EntityType::Recognised),

        "PRIV LTD SECT. 30 (Private limited company, section 30 of the Companies Act)"
            => Some(EntityType::Recognised),

        "Protected Cell Company"
            => Some(EntityType::Recognised),

        "Converted/Closed"
            => Some(EntityType::Recognised),

        "Further Education and Sixth Form College Corps"
            => Some(EntityType::Recognised),

        "Overseas Entity"
            => Some(EntityType::Recognised),

        "Public Limited Company"
            => Some(EntityType::Plc),

        _ => None
    }
}

/*
  The data has four columns for SIC codes; all the valid SIC codes are five digits or fewer,
  and, in the data, are presented with a textual annotation, which we remove. The separator
  for the text annotation is what we use to work out if the code is valid.
*/
pub fn sics_of_one_record(record: &LegalEntityRecord) -> Vec<String> {
    let mut sics = vec![];
    for sic in vec![
        &record.sic_code1,
        &record.sic_code2,
        &record.sic_code3,
        &record.sic_code4
    ] {
        if !sic.contains(" - ") {
            continue;
        }
        sics.push(sic.split(" ").next().unwrap().to_string()); // sic
    }
    sics
}

pub struct LegalEntity {
    pub name: String,
    pub number: String,
    pub category: EntityType,
    pub sic_codes: Vec<String>
}

impl LegalEntity {
    pub fn new(ler: LegalEntityRecord) -> Option<LegalEntity> {
        let sic_codes = sics_of_one_record(&ler);
        let category = entity_type_of_str(&ler.company_type);
        match category {
            None => {
                eprintln!("Unrecognised entity type");
                None
            },
            Some(cat) =>
                Some(LegalEntity {
                    name: ler.name,
                    number: ler.number,
                    category: cat,
                    sic_codes: sic_codes
                })
        }
    }

    pub fn to_vec(self) -> Vec<String> {
        let category = self.category;
        let description = format!("{category}");
        vec![
            self.number,
            description,
            self.name
        ]
    }
}

impl fmt::Display for LegalEntity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LegalEntity: {}", self.number)
    }
}
