extern crate rustbox;

use std::error::Error;
use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

struct FieldSet {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    title: &'static str
}

fn display_fieldset(rustbox: &RustBox, fieldset: FieldSet) {
    // The 4th corners
    rustbox.print(fieldset.x, fieldset.y, rustbox::RB_NORMAL, Color::White, Color::Black, "┌");
    rustbox.print(fieldset.x + fieldset.width, fieldset.y, rustbox::RB_NORMAL, Color::White, Color::Black, "┐");
    rustbox.print(fieldset.x, fieldset.y + fieldset.height, rustbox::RB_NORMAL, Color::White, Color::Black, "└");
    rustbox.print(fieldset.x + fieldset.width, fieldset.y + fieldset.height, rustbox::RB_NORMAL, Color::White, Color::Black, "┘");

    // Horizontal lines
    for i in 1..fieldset.width {
        rustbox.print(fieldset.x + i, fieldset.y, rustbox::RB_NORMAL, Color::White, Color::Black, "─");
        rustbox.print(fieldset.x + i, fieldset.y + fieldset.height, rustbox::RB_NORMAL, Color::White, Color::Black, "─");
    }

    // Vertical lines
    for i in 1..fieldset.height {
        rustbox.print(fieldset.x, fieldset.y + i, rustbox::RB_NORMAL, Color::White, Color::Black, "│");
        rustbox.print(fieldset.x + fieldset.width, fieldset.y + i, rustbox::RB_NORMAL, Color::White, Color::Black, "│");
    }

    // Title
    rustbox.print(fieldset.x + 2, fieldset.y, rustbox::RB_NORMAL, Color::Red, Color::Black, &*format!(" {} ", fieldset.title));
}

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(7, 0, rustbox::RB_BOLD, Color::Blue, Color::Black, "Grammar : Conjugeur");

    display_fieldset(&rustbox, FieldSet{ x: 4, y: 2, width: 25, height: 2, title: "Infinitif" });
    rustbox.print(7, 3, rustbox::RB_BOLD, Color::White, Color::Black, "être");

    display_fieldset(&rustbox, FieldSet{ x: 4, y: 5, width: 25, height: 2, title: "Participe présent" });
    rustbox.print(7, 6, rustbox::RB_BOLD, Color::White, Color::Black, "étant");

    display_fieldset(&rustbox, FieldSet{ x: 4, y: 8, width: 25, height: 2, title: "Participe passé" });
    rustbox.print(7, 9, rustbox::RB_BOLD, Color::White, Color::Black, "été");

    display_fieldset(&rustbox, FieldSet{ x: 4, y: 11, width: 25, height: 10, title: "Indicatif" });
    rustbox.print(7, 12, rustbox::RB_BOLD, Color::Blue, Color::Black, "Présent");
    rustbox.print(7, 13, rustbox::RB_BOLD, Color::White, Color::Black, "je suis");
    rustbox.present();
    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
    }
}