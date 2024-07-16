use clap::Parser;
use clap_verbosity_flag::Verbosity;
use w1_temperature_api::app::{run, Config};
use w1_temperature_api::log::init_logger;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgGroup {
    /// Set log level
    #[command(flatten)]
    verbosity: Verbosity,

    /// ID of the w1 device to supply temperature for
    #[arg()]
    device_id: String,

    /// Interface to run server on
    #[arg(short, long, default_value = "127.0.0.1")]
    interface: String,

    /// Port to run server on
    #[arg(short, long, default_value = "8080")]
    port: u16,
}

fn main() {
    let cli_args = CliArgGroup::parse();
    init_logger(cli_args.verbosity);

    let config = Config {
        device_id: cli_args.device_id,
        interface: cli_args.interface,
        port: cli_args.port,
    };

    run(config).unwrap();
}
