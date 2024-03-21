use clap::{arg, command, value_parser, ArgGroup, ArgMatches};
use waybar_hypr_fullscreen_status::{query_monitor_status_by_id, query_monitor_status_by_name};

fn main() {
    let args = arg_matches();

    let res: Option<bool> = args.get_one::<u8>("monitor-id").map_or_else(
        || {
            args.get_one::<String>("monitor-name")
                .and_then(query_monitor_status_by_name)
        },
        |id| query_monitor_status_by_id(*id),
    );

    // Convert the boolean return to a message
    let message = match res {
        Some(true) => args
            .get_one::<String>("fullscreen-text")
            .expect("has default value"),
        Some(false) => args
            .get_one::<String>("normal-text")
            .expect("has default value"),
        None => "",
    };

    println!("{message}");
}

#[allow(clippy::cognitive_complexity)]
fn arg_matches() -> ArgMatches {
    command!()
        .arg(arg!(--"monitor-id" <ID> "Monitor ID").value_parser(value_parser!(u8)))
        .arg(arg!(--"monitor-name" <NAME> "Monitor name"))
        .group(
            ArgGroup::new("monitor")
                .required(true)
                .args(["monitor-id", "monitor-name"]),
        )
        .arg(
            arg!(--"fullscreen-text" <MESSAGE> "Full screen mode text")
                .default_value("Full Screen"),
        )
        .arg(arg!(--"normal-text" <MESSAGE> "Normal mode text").default_value(""))
        .get_matches()
}
