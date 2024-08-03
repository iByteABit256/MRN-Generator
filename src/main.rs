mod parser;

use std::error::Error;

use clap::Parser;
use mrn_generator::*;
use parser::Args;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let declaration_office = args.declaration_office.as_deref();
    let combined = args.combined.as_deref();
    let procedure = args
        .procedure_category
        .map(|proctg| match_procedure(&proctg, combined));

    for _ in 0..args.number_of_mrns {
        let mrn: &str = &generate_random_mrn(&args.country_code, procedure, declaration_office);
        println!("{mrn}");
    }

    Ok(())
}
