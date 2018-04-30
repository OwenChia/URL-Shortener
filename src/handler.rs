#![allow(dead_code)]
#![allow(unused_imports)]

use futures::future::Future;
use actix_web::{Path, HttpRequest, HttpResponse};

use database::StoreUrl;
use utils;

pub fn index(_req: HttpRequest<::State>) -> &'static str {
    "Hello world!"
}

pub fn set(url: Path<(String,)>, req: HttpRequest<::State>) -> String {
    let url = utils::decode_url(&url.0);

    match req.state().db.send(StoreUrl{url: url}).wait() {
        Ok(s) => format!("{}", s.unwrap()),
        Err(e) => format!("{:?}", e),
    }
}

pub fn path(info: Path<(String,)>) -> String {
    format!("{}", info.0)
}

pub fn redirect(_req: HttpRequest<::State>) -> HttpResponse {
    let body = format!(r#"<html><meta http-equiv="refresh" content="0;url={uri}"></html>"#,
                       uri="https://www.sogou.com");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(body)
}
