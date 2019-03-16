use clap::{App, Arg};

fn main() {
    let matches = App::new("healthcheck")
        .version("0.1.0")
        .author("@armariya")
        .about("A healthcheck tool written in Rust. It will fire a webhook for you when the server didn't return 200.")
        .arg(Arg::with_name("url")
            .required(true)
            .long("url")
            .help("The URL that you want to check")
            .takes_value(true))
        .arg(Arg::with_name("every")
            .required(true)
            .long("every")
            .help("How often do you want to check")
            .takes_value(true))
        .arg(Arg::with_name("webhook")
            .required(true)
            .long("webhook")
            .help("The webhook URL that will call for you when it didn't return 200")
            .takes_value(true))
        .get_matches();

    let config = matches.value_of("config").unwrap();
    println!("Value of config: {}", config);
}
