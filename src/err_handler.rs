extern crate actix_web;
use actix_web::{App, http::Method, HttpRequest, HttpResponse, dev::Resource};
use serde::{Serialize};

#[derive(Serialize)]
struct Body {
    ok: bool,
    status: String,
}

fn not_found(_r: &HttpRequest) -> HttpResponse {
    HttpResponse::NotFound()
        .json(Body{ok: false, status: "not found".to_string()})
}

fn bad_request(_r: &HttpRequest) -> HttpResponse {
    HttpResponse::BadRequest()
        .json(Body{ok: false, status: "bad request".to_string()})
}

pub fn handle(r: &mut Resource) {
    r.method(Method::GET).f(not_found);
    r.method(Method::POST).f(bad_request);
}

pub fn serve() -> App {
    App::new().default_resource(handle)
}
