#[macro_use]
extern crate bencher;
extern crate grammar;

use grammar::textformater::fr::*;
use bencher::Bencher;

fn loop_formater() {
    let mut formater = TextFormater::new();
    for (key, val) in &formater.groups {
        for (k, v) in &val.lines {
            format(k.as_test(), &v.rules);
        }
    }
}

fn all(b: &mut Bencher) {
    b.iter(|| loop_formater());
}

fn bench_summary_space(b: &mut Bencher) {
    /*let mut formater = TextFormater::new();
    let t = formater.get(
        TextFormaterGroups::SurnumerarySpaces,
        TextFormaterLine::BetweenWords
    );*/
    let t = &TextFormat {
        short:   "Entre les mots",
        rules:   vec![["  +", " "]]
    };
    b.iter(|| format("    un texte avec trop d'espaces au début", &t.rules));
}

benchmark_group!(benches, all);
benchmark_main!(benches);