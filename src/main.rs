#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(use_extern_macros)]
#![feature(proc_macro_path_invoc)] 
#![feature(custom_attribute)] 
#![feature(decl_macro)] 
#![feature(attr_literals)]
#![feature(unicode)]  
#![feature(try_from)] 
extern crate rocket;
use rocket::http::Status;
use rocket::http::RawStr;
use rocket::request::{self, Form, FromForm, FromRequest};
use rocket::response::{content, NamedFile};
use rocket::{Outcome, Request, State};
use rocket::config::{self, Config, Environment, RocketConfig};
extern crate rocket_contrib;
use rocket_contrib::Template;


//extern crate env_logger;
#[macro_use(log)]
extern crate log;
extern crate log4rs;

pub mod test_mod;

#[get("/")]
fn home() -> String {
    ::log::debug!("debug");
    ::log::info!("info");
    ::log::warn!("warn");
    ::log::error!("error");
    test_mod::t();
    "home".to_owned()
}

//#[catch(404)]    //rocket 4.0 dev
#[error(404)]
fn not_found(req: &rocket::Request) -> content::Html<&'static str> {
    content::Html("Page Not Found.")
}


fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    rocket::ignite()
        .mount("/", routes![home])
        .catch(errors![not_found])
        .launch();
}
