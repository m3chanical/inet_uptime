use clap::{Arg, Command};
use inet_uptime;

#[tokio::main]
async fn main() {

    let matches = clap::Command::new("Internet Uptime")
    .version("0.0.1")
    .about("My internet constantly disconnects so I wanna fucking see it")
    .author("Sinbeard the 'Human'")
    .subcommand_required(true)
    .subcommand(
        Command::new("ping")
            .short_flag('P')
            .long_flag("ping")
            .about("Ping an address")
            .arg(
                Arg::new("address")
                    .short('a')
                    .long("address")
                    .help("Internet address to ping, IP or URL")
                    .takes_value(true)
                    .multiple_values(true),
            )
            .arg(
                Arg::new("port")
                    .short('p')
                    .long("port")
                    .help("Port on the server to ping")
                    .takes_value(true)
                    .multiple_values(true)                
            )
            .arg(
                Arg::new("loop")
                    .short('l')
                    .long("loop")
                    .help("Continuously Ping servers")
                    .takes_value(false)
                    .multiple_values(false)
            )
            .arg(
                Arg::new("save")
                    .short('s')
                    .long("save")
                    .help("Write results to sqlite db")
                    .takes_value(false)
                    .multiple_occurrences(false)
            ),
    )
    .get_matches();

    match matches.subcommand() {
        Some(("ping", ping_matches)) => {
            ping_command(&ping_matches).await;
        }
        _ => unreachable!(),
    }
}

async fn ping_command(ping_matches: &clap::ArgMatches) 
{
    if let Some(address) = ping_matches.values_of("address") {
        let values = address.collect::<Vec<_>>();
        // check if port is used
        inet_uptime::connect(values).await;
    } else {
        let mut values: Vec<&str> = Vec::new(); 
        // if no address, but port is used, append port to default address
        values.push("8.8.8.8:53");
        inet_uptime::connect(values).await;
    }
}