use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'x', long, default_value_t = 'X')]
    pub fill_char: char,

    #[arg(short = 'c', long, default_value_t = 'o')]
    pub cross_char: char,

    #[arg(short, long, default_value_t = '.')]
    pub blank: char,

    #[arg(short, long)]
    pub debug: bool,
}
