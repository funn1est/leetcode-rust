use clap::{App, AppSettings, Arg, SubCommand};
use leetcode_rust::{LcCli, Result};

fn main() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("get")
                .about("Get leetcode question")
                .arg(
                    Arg::with_name("QUESTION")
                        .help("The question name")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("get", Some(matches)) => {
            let question = matches
                .value_of("QUESTION")
                .expect("QUESTION argument missing");

            if let Ok(_) = LcCli::get(question.to_owned()) {
                println!("gen question {} successful", question);
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
