extern crate chrono;
extern crate clap;

mod server;
mod client;

use clap::{Arg, App, SubCommand};

fn main() -> std::io::Result<()> {

    let matches = App::new("Local speed testings tool")
        .version("0.0.1")
        .author("MapLegend")
        .about("Testing speed in local network")
        .subcommand(SubCommand::with_name("server")
            .alias("s")
            .about("runs server")
            .arg(Arg::with_name("ip")
                .short("i")
                .required(true)
                .value_name("IP ADDRESS")
                .takes_value(true)
                .index(1)
                .help("sets server ip address"))
            .arg(Arg::with_name("verbose")
                .short("v")
                .help("print debug information verbosely")))
        .subcommand(SubCommand::with_name("client")
            .alias("c")
            .about("runs client")
            .arg(Arg::with_name("ip")
                .short("i")
                .required(true)
                .index(1)
                .value_name("IP ADDRESS")
                .takes_value(true)
                .help("sets client ip address"))
            .arg(Arg::with_name("verbose")
                .short("v")
                .help("print debug information verbosely"))
            .arg(Arg::with_name("payload")
                .short("p")
                .value_name("PAYLOAD")
                .takes_value(true)
                .conflicts_with("size")
                .help("sets payload"))
            .arg(Arg::with_name("size")
                .short("s")
                .value_name("PAYLOAD SIZE")
                .takes_value(true)
                .default_value("10")
                .help("sets payload size"))
            .arg(Arg::with_name("tries")
                .short("t")
                .value_name("COUNT")
                .default_value("10")
                .takes_value(true)
                .help("sets tries count")))
        .get_matches();

    if let Some(args) = matches.subcommand_matches("server") {
        server::run(args)
    } else if let Some(args) = matches.subcommand_matches("client") {
        client::run(args)
    } else {
        Ok(())
    }

//    let args: Vec<String> = env::args().collect();
//    match &*args[1] {
//        "client" | "c" => client::run(args),
//        "server" | "s" => server::run(args),
//        _ => {
//            println!("unknown type");
//            Ok(())
//        }
//    }
}