pub fn install(arg: &str) {
    let args: Vec<&str> = arg.split("::").collect();
    if args.len() < 2 {
        println!("Please enter a package name and repo like pkg::repo");
        std::process::exit(1);
    }

    match args[1] {
        "aur" => println!("AUR"),
        _ => {
            println!("Repo \"{}\" is not supported", args[1]);
            std::process::exit(1);
        }
    };
}
