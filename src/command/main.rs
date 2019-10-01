use std::env;

const HELP_STR: &str = "IMPERFECT...\n\nSyntax: i [command] [arg]\n\nCommands:\n  install\tinstall a package\n  remove\tremove a package\n  search\tsearch for a package\n  update\tupgrade a package\n  repo\t\t[sync|add|remove] syncs, adds, or removes a package repository\n  sync\t\tsyncs all repos and updates all packages";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("{}", HELP_STR);
        std::process::exit(1);
    }

    let cmd: &str = args[1].as_str();
    let arg: &str = args[2].as_str();

    match cmd {
        "install" => return,
        "remove" => return,
        "search" => return,
        "update" => return,
        "repo" => return,
        "sync" => return,
        _ => {
            println!("Command {} does not exist...", cmd);
            std::process::exit(1);
        }
    };
}
