use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    baselink: String,

    #[clap(value_parser)]
    secret: String,

    #[clap(short, long, default_value_t = 30)]
    period: i64,
}


use base64::{encode_config, URL_SAFE_NO_PAD};
use chrono::{Duration, Utc};
use url::Url;

fn secure_link(baselink: &str, secret: &str, period: i64) -> String {
    let url = Url::parse(baselink).expect("Invalid URL");

    let expires = (Utc::now() + Duration::days(period))
        .timestamp();

    let hashstring = format!("{}{} {}", expires, url.path(), secret);

    let result = md5::compute(hashstring.as_bytes());

    let protection_string = encode_config(&result[..], URL_SAFE_NO_PAD);

    format!("{}?md5={}&expires={}", baselink, protection_string, expires)
}

fn main() {
    let args = Args::parse();

    let link = secure_link(&args.baselink, &args.secret, args.period);

    println!("{}", link);
}
