use actix_web::{Path, HttpRequest, HttpResponse};
use futures::future::Future;

use database::{GetUrl, StoreUrl, DelUrl};
use utils;
use State;

pub fn index(_req: HttpRequest<State>) -> &'static str {
    "Hello world!"
}

pub fn set(url: Path<(String,)>, req: HttpRequest<State>) -> String {
    let url = utils::decode_url(&url.0);
    let length = req.state().length;
    let message = StoreUrl {
        url: url,
        length: length,
    };

    match req.state().db.send(message).wait() {
        Ok(s) => format!("{}", s.unwrap()),
        Err(e) => format!("{:?}", e),
    }
}

pub fn get(info: Path<(String,)>, req: HttpRequest<State>) -> HttpResponse {
    let url = info.0.trim().to_owned();
    let message = GetUrl {
        hashed_url: url,
    };

    match req.state().db.send(message).wait() {
        Ok(s) => match s {
            Ok(url) => HttpResponse::Ok()
                .content_type("text/html")
                .body(utils::redirect(&url)),
            Err(e) => {
                println!("NOT FOUND: {:?}", e);
                HttpResponse::NotFound()
                    .finish()
            },
        },
        Err(e) => HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("GET ERROR: {:?}", e)),
    }
}

pub fn del(info: Path<(String,)>, req: HttpRequest<State>) -> String {
    let url = info.0.to_owned();
    let message = DelUrl {
        hashed_url: url,
    };

    match req.state().db.send(message).wait() {
        Ok(s) => format!("{}", s.unwrap()),
        Err(e) => format!("{:?}", e),
    }
}
