extern crate actix_web;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate redis;
extern crate rand;

use std::path::Path;
use std::error::Error;

use actix_web::{server, http, App};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

mod config;
mod handler;
mod database;

fn generate_shorturl(length: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(length).collect()
}

fn generate_shorturl_8() -> String {
    generate_shorturl(8)
}

fn main() {
    let path = Path::new("config.toml");
    let config = config::Config::from_file(path);
    let mut redis_client = database::RedisClient::new(&config.redis_connection.unwrap()[..]);


    let uri = String::from("http://sogou.com");

    let suri = redis_client.set(&uri).unwrap();
    match redis_client.get(&suri) {
        Ok(uri) => println!("{} -> {}", uri, suri),
        Err(why) => panic!("reason: {}", why.description()),
    };

    server::new(
        || App::new()
            .resource("/", |r| r.f(handler::index))
            .resource("/redirect", |r| r.f(handler::redirect))
            .resource("/{hashed_uri}", |r| r.method(http::Method::GET).with(handler::path)))
        .bind("127.0.0.1:8088").expect("Can not bind to 127.0.0.1:8088")
        .run();

    redis_client.del(&suri).unwrap();
}
