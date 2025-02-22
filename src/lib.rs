pub use clap::Parser;
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
#[command(name = "ping_shutdown")]
#[command(author = "H-Chris233")]
#[command(version = "0.0.2")]
pub struct ArgsIn{
    ///the ip address or website you want to check [Default: bing.com]
    //#[arg(short, long, default_values_t = vec!["bing.com".to_string(), "apple.com".to_string()])]
    #[arg(short, long, default_value = "bing.com")]
    pub ip: String,
    ///use -o to active when any connection losts
    #[arg(short, long, default_value = "and")]
    pub or: String,
    ///time between two normal check [Default: 60]
    #[arg(short = 'n', long, default_value = "60")]
    pub secs_for_loop: String,
    ///time between two emegency check [Default: 20]
    #[arg(short = 'e', long, default_value = "20")]
    pub secs_for_emergency_loop: String,
    ///times for emergency lopp [Default: 3]
    #[arg(short, long, default_value = "3")]
    pub times_for_emergency_loop: String,
}












