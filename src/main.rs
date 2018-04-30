extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate redis;
extern crate rand;
extern crate env_logger;
extern crate futures;
extern crate url;

use std::path::Path;
use std::error::Error;

use actix::prelude::*;
use actix_web::{server, http, App};
use actix_web::middleware::Logger;

mod config;
mod handler;
mod database;
mod utils;

use database::DbExecutor;

pub struct State {
    db: Addr<Syn, DbExecutor>,
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("shorturl");

    let path = Path::new("config.toml");
    let config = config::Config::from_file(path);
    let redis_client = database::RedisClient::new(&config.redis_connection.unwrap()[..]);

    let addr: Addr<Syn, DbExecutor> = DbExecutor(redis_client.connection).start();


    //let uri = String::from("http://sogou.com");

    //let suri = redis_client.set(&uri).unwrap();
    //match redis_client.get(&suri) {
        //Ok(uri) => println!("{} -> {}", uri, suri),
        //Err(why) => panic!("reason: {}", why.description()),
    //};

    server::new(
        move || App::with_state(State{ db: addr.clone() })
            .middleware(Logger::default())
            .resource("/", |r| r.f(handler::index))
            .resource("/set/{url}", |r| r.method(http::Method::GET).with2(handler::set))
            .resource("/redirect", |r| r.f(handler::redirect))
            .resource("/{hashed_uri}", |r| r.method(http::Method::GET).with(handler::path)))
        .bind("127.0.0.1:8088").expect("Can not bind to 127.0.0.1:8088")
        .start();

    let _ = sys.run();

    //redis_client.del(&suri).unwrap();
}
