extern crate moniker_rs;

use moniker_rs::{Animal, Moby, Moniker};

enum Command {
    Default(MonikerType),
    License,
    Help,
}

enum MonikerType {
    Animal,
    Moby,
    Unknown,
}

fn parse_type(arg: &str) -> MonikerType {
    match arg {
        "moby" => MonikerType::Moby,
        "animal" => MonikerType::Animal,
        _ => MonikerType::Unknown,
    }
}

fn parse_command(arg: Option<&String>) -> Command {
    match arg {
        None => print_help(),
        Some(cmd) => match cmd.as_str() {
            "--help" | "-h" => Command::Help,
            "--license" => Command::License,
            _ => Command::Default(parse_type(cmd)),
        },
    }
}

fn print_monikers_help() -> ! {
    println!(r#"Available moniker types: "moby", "animal""#);

    std::process::exit(1);
}

fn print_license() -> ! {
    println!(include_str!("../../LICENSE"));

    std::process::exit(0);
}

fn print_help() -> ! {
    println!(
        r"Usage:
    moniker-cli (animal | moby)
    moniker-cli --license
    moniker-cli -h | --help"
    );

    std::process::exit(0);
}

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = parse_command(args.get(1));

    let moniker_type = match cmd {
        Command::License => print_license(),
        Command::Help => print_help(),
        Command::Default(moniker_type) => moniker_type,
    };

    let moniker = match moniker_type {
        MonikerType::Unknown => print_monikers_help(),
        MonikerType::Moby => Moby::get_random(),
        MonikerType::Animal => Animal::get_random(),
    };

    println!("{}", moniker);
}
