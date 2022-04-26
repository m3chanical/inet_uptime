use clap::{Arg, Command};
use inet_uptime;
use std::any::type_name;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {

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
            ),
    )
    .get_matches();
    // jump into lib.rs and do stuff. 
    // open up internet connection

    match matches.subcommand() {
        Some(("ping", ping_matches)) => {
            if let Some(address) = ping_matches.values_of("address") {
                let values = address.collect::<Vec<_>>();
                // check if port is used
                inet_uptime::ping(values);
            } else {
                let mut values: Vec<&str> = Vec::new(); 
                // if no address, but port is used, append port to default address
                values.push("8.8.8.8:53");
                inet_uptime::ping(values);
            }
        }
        _ => unreachable!(),
    }
}
