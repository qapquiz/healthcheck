# healthcheck
A healthcheck tool written in Rust. It will fire a webhook for you when the server didn't return 200.

## USAGE:
    healthcheck --every <every> --url <url> --webhook <webhook>

## FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

## OPTIONS:
    --every <every>        How often do you want to check (in millisecond)
    --url <url>            The URL that you want to check
    --webhook <webhook>    The webhook URL that will call for you when it didn't return 200
