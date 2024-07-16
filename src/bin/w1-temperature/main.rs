use clap::Parser;
use clap_verbosity_flag::Verbosity;
use w1_temperature_api::log::init_logger;
use w1_temperature_api::temperature::read_temperature;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgGroup {
    /// Set log level
    #[command(flatten)]
    verbosity: Verbosity,

    /// ID of the w1 device to supply temperature for
    #[arg()]
    device_id: String,
}

fn main() {
    let cli_args = CliArgGroup::parse();
    init_logger(cli_args.verbosity);

    let device_id = cli_args.device_id;
    let temperature = read_temperature(&device_id).unwrap();

    println!("{}", temperature);
}
