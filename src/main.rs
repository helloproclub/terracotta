extern crate actix_web;
use actix_web::{server};

mod v1;
mod err_handler;

fn main() {
    server::new(|| {
        vec![
            v1::serve(),
            err_handler::serve(),
        ]
    }).bind("127.0.0.1:8088").unwrap().run();
}
