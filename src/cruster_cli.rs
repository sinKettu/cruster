mod cruster;

use newer_clap as clap;


fn cli() -> clap::Command {
    clap::Command::new("cruster-cli")
        .about("Cruster Command Line Interface")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            clap::Command::new("http")
                .about("Work with dumped HTTP data")
                .subcommand_required(true)
                .subcommand(
                    clap::Command::new("show")
                        .about("Filter/Sort/Find HTTP data and print it")
                        .alias("s")
                        .arg_required_else_help(true)
                        .arg(
                            clap::arg!(<INDEX> "range or index in storage to print HTTP data: n -- first n pairs, n-m -- pairs from n to m, -m -- last m pairs, n! -- only Nth pair")
                                .required(true)
                        )
                        .arg(
                            clap::Arg::new("urls")
                                .short('u')
                                .long("urls")
                                .action(clap::ArgAction::SetTrue)
                                .help("Print only indexes and full URLs")
                        )
                )
        )
        .arg(
            clap::arg!(-c <CONFIG> "Path to cruster config")
                .default_value("~/.cruster/config.yaml")
        )
        .arg(
            clap::arg!(-p <PROJECT> "Path to project dir to work with (by default will try to get it from config)")
        )
}

fn main() {
    let command = cli().get_matches();

    let config_path = command.get_one::<String>("CONFIG").unwrap();
    let proj_path = command.get_one::<String>("PROJECT");

    match command.subcommand() {
        Some(("http", subcommands)) => {
            match subcommands.subcommand() {
                Some(("show", args)) => {
                    let range = args.get_one::<String>("INDEX").unwrap();
                    let urls = args.get_flag("urls");

                    println!("INDEX:{:?} URLS:{:?}", range, urls);
                },
                _ => {}
            }
        },
        _ => {
            unreachable!()
        }
    }
}
