use clap::{arg, command, Command};

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("set")
                .about("Set the value of a string key to a string")
                .arg(arg!([KEY]))
                .arg(arg!([VALUE])),
        )
        .subcommand(
            Command::new("get")
                .about("Get the string value of a given string key")
                .arg(arg!([KEY])),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove a given key")
                .arg(arg!([KEY])),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("set", _)) => {
            panic!("unimplemented");
        }
        Some(("get", _)) => {
            panic!("unimplemented");
        }
        Some(("rm", _)) => {
            panic!("unimplemented");
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
