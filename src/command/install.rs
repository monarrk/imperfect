use hyper::Client;
use hyper::Uri;

const AUR_URL: &str = "http://aur.archlinux.org/";

pub async fn install(arg: &str) -> Result<(), hyper::error::Error> {
    let args: Vec<&str> = arg.split("::").collect();
    if args.len() < 2 {
        println!("Please enter a package name and repo like pkg::repo");
        std::process::exit(1);
    }

    match args[1] {
        "aur" => aur_download(args[0]).await,
        _ => {
            println!("Repo \"{}\" is not supported", args[1]);
            std::process::exit(1);
        }
    }.expect("Failed to get pkg");

    Ok(())
}

async fn aur_download(pkgname: &str) -> Result<(), hyper::error::Error> {
    println!("Downloading {} from the AUR", pkgname);

    let url = format!("{}{}.git", AUR_URL, pkgname).as_str().parse::<Uri>().unwrap();

    println!("\tMaking request to {}", &url);
    let client = Client::new();
    let res = client.get(url).await?;
    println!("\tResponse: {}", res.status());

    let mut body = res.into_body();

    while let Some(next) = body.next().await {
        let chunk = next?;
        println!("{:?}", &chunk);
    }

    Ok(())
}
