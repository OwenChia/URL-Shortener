#![allow(dead_code)]
#![allow(unused_imports)]

use actix_web::{Path, HttpRequest, HttpResponse};

pub fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

pub fn path(info: Path<(String,)>) -> String {
    format!("{}", info.0)
}

pub fn redirect(_req: HttpRequest) -> HttpResponse {
    let body = format!(r#"<html><meta http-equiv="refresh" content="0;url={uri}"></html>"#,
                       uri="https://www.sogou.com");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(body)
}
