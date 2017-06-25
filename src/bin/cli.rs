#[macro_use]
extern crate lazy_static;
extern crate grammar;

use grammar::textformater::fr::*;

fn loop_formater() {
    let mut formater = TextFormater::new();
    let rules = formater.all_rules();
    for x in 0..100000 {
        format("un texte   avec   trop d'espaces".to_string(), &rules);
    }
}

fn main() {
    loop_formater();
}