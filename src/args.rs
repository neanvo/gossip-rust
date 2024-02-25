use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long, short = 'p', required = true)]
    pub period: u16,

    #[arg(long, short = 'o', required = true)]
    pub port: u16,

    #[arg(long, short = 'c')]
    pub connect: String,
}
