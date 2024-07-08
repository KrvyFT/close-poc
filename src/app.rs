use clap::{Arg, Command};
use tokio::task::{JoinError, JoinSet};

fn cli() -> Command {
    Command::new("check-poc")
        .about("A proof-of-concept tool for checking system vulnerabilities")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("check")
                .about("Check vulnerabilities using a proof-of-concept file")
                .arg(
                    Arg::new("POC")
                        .short('p')
                        .long("poc")
                        .value_name("PATH")
                        .required(true),
                )
                .arg(
                    Arg::new("URLLIST")
                        .short('u')
                        .long("urllist")
                        .value_name("PATH")
                        .required(true),
                ),
        )
}

pub fn app() -> (String, String) {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("check", arg)) => {
            let poc_path = arg.get_one::<String>("POC").unwrap().clone();
            let urllist_path = arg.get_one::<String>("URLLIST").unwrap().clone();
            return (poc_path, urllist_path);
        }
        _ => panic!(),
    }
}

pub async fn run(path: (String, String)) -> JoinSet<Result<(), JoinError>> {
    let poc = crate::poc::Poc::from_json(&path.0).unwrap();
    let urls = crate::urls::read_from_file(&path.1).await.unwrap();

    poc.clone().check_all_vulnerabilities(urls)
}
