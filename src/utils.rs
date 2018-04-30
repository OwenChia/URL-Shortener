use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use url::form_urlencoded;

pub fn decode_url(url: &str) -> String {
    form_urlencoded::parse(url.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect()
}

fn generate_shorturl(length: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(length).collect()
}

pub fn generate_shorturl_8() -> String {
    generate_shorturl(8)
}
