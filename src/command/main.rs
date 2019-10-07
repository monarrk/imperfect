#![feature(termination_trait_lib)]
#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::env;

mod install;

const HELP_STR: &str = "IMPERFECT...\n\nSyntax: i [command] [arg]\n\nCommands:\n  install\tinstall a package\n  remove\tremove a package\n  search\tsearch for a package\n  update\tupgrade a package\n  repo\t\t[sync|add|remove] syncs, adds, or removes a package repository\n  sync\t\tsyncs all repos and updates all packages";

#[tokio::main]
async fn main() -> Result<(), hyper::error::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("{}", HELP_STR);
        std::process::exit(1);
    }

    let cmd: &str = args[1].as_str();
    let arg: &str = args[2].as_str();

    match cmd {
        "install" => install::install(arg),
        //"remove" => cmd,
        //"search" => cmd,
        //"update" => cmd,
        //"repo" => cmd,
        //"sync" => cmd,
        _ => {
            println!("Command {} does not exist...", cmd);
            std::process::exit(1);
        }
    }.await?;

    Ok(())
}
