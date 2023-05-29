use clap::Parser;

/// Command line utility to generate valid MRNs
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Country code of MRN
    #[arg(short, long)]
    pub country_code: String,

    /// Number of MRNs to generate
    #[arg(short, long, default_value_t = 1)]
    pub number_of_mrns: usize,

    /// Regime
    #[arg(short, long)]
    pub regime: Option<String>,
}
