use std::{thread, time};
use std::collections::HashMap;

use clap::{App, Arg};
use reqwest::Client;


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
            .help("How often do you want to check (in millisecond)")
            .takes_value(true))
        .arg(Arg::with_name("webhook")
            .required(true)
            .long("webhook")
            .help("The webhook URL that will call for you when it didn't return 200")
            .takes_value(true))
        .get_matches();

    let url = matches.value_of("url").unwrap();
    let every = matches.value_of("every").unwrap();
    let web_hook = matches.value_of("webhook").unwrap();

    println!();
    println!("HealthCheck by @armariya");
    println!("...................................................................................");
    println!("Call: {}", url);
    println!("Every {} milliseconds", every);
    println!("If failed will call: {}", web_hook);
    println!("...................................................................................");

    let every_millisecond: u64 = every.parse::<u64>().unwrap();

    call_url_every_x_time_if_failed_then_call_web_hook(url, every_millisecond, web_hook)
}

fn call_url_every_x_time_if_failed_then_call_web_hook(url: &str, every: u64, web_hook: &str) {
    let client = Client::new();
    loop {
        let status = match client.get(url).send() {
            Ok(response) => response.status().as_u16(),
            Err(_) => 404
        };

        if status != 200 {
            let mut map = HashMap::new();
            map.insert("username", "healthcheck");
            map.insert("content", "the server is down");
            client
                .post(web_hook)
                .header("Content-Type", "application/json")
                .json(&map)
                .send()
                .unwrap();
        }

        thread::sleep(time::Duration::from_millis(every));
    }
}