extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate rand;
extern crate redis;
#[macro_use] extern crate serde_derive;
extern crate toml;
extern crate url;

use std::path::Path;

use actix::prelude::*;
use actix_web::middleware::Logger;
use actix_web::{server, http, App};

mod config;
mod database;
mod handler;
mod utils;

use database::DbExecutor;
use config::Config;

pub struct State {
    db: Addr<Syn, DbExecutor>,
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("shorturl");

    let path = Path::new("config.toml");
    let config = Config::from_file(path);

    let redis_connection = {
        let client = redis::Client::open(&config.redis_connection.unwrap()[..]);
        client.unwrap().get_connection().unwrap()
    };

    let addr: Addr<Syn, DbExecutor> = DbExecutor(redis_connection).start();

    server::new(
        move || App::with_state(State{ db: addr.clone() })
            .middleware(Logger::default())
            .resource("/", |r| r.f(handler::index))
            .resource("/set/{url}", |r| r.method(http::Method::GET).with2(handler::set))
            .resource("/del/{hashed_url}", |r| r.method(http::Method::GET).with2(handler::del))
            .resource("/{hashed_url}", |r| r.method(http::Method::GET).with2(handler::get)))
        .bind("127.0.0.1:8088").expect("Can not bind to 127.0.0.1:8088")
        .start();

    let _ = sys.run();
}
