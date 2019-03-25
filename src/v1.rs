extern crate actix_web;
use actix_web::{App, http::Method, HttpResponse};

use crate::err_handler;

pub fn serve() -> App {
    App::new().prefix("/v1")
        .resource("/users", |r| {
            r.method(Method::GET).f(|_r| HttpResponse::Ok());
        })
        .default_resource(err_handler::handle)
}
