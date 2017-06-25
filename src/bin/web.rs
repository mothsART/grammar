#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
extern crate rocket_contrib;
extern crate rocket;
extern crate grammar;

use grammar::textformater::fr::*;
use rocket_contrib::{MsgPack};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use rocket::response::NamedFile;
use rocket_contrib::Template;


#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/")]
fn index() -> Template {
    let mut map = HashMap::new();
    map.insert("path", "cool");
    //let context = HashMap::new();
    Template::render("index", &map)
}

#[post("/", data = "<data>", format = "application/msgpack")]
fn f(data:  MsgPack<String>) -> MsgPack<String> {
    let mut formater = TextFormater::new();
    let rules = formater.all_rules();
    MsgPack(
        format(data.0, &rules).to_string()
    )
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index]).attach(Template::fairing())
    .mount("/static", routes![files])
    .mount("/format", routes![f])
    .launch();
}