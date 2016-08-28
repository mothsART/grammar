extern crate rustbox;

use lib::{BoxStruct, TextArea, Box};
mod lib;

use std::error::Error;
use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

struct Suggestion {
    selected: bool,
    highlight_start: usize,
    highlight_end: usize,
    text: &'static str,
}

struct SuggestionBox {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    suggestions: Vec<Suggestion>,
}

fn display_suggestionn_box(rustbox: &RustBox, suggestion_box: SuggestionBox) {
    // The 4th corners
    rustbox.print(suggestion_box.x,
                  suggestion_box.y,
                  rustbox::RB_NORMAL,
                  Color::White,
                  Color::Default,
                  "┌");
    rustbox.print(suggestion_box.x + suggestion_box.width,
                  suggestion_box.y,
                  rustbox::RB_NORMAL,
                  Color::White,
                  Color::Default,
                  "┐");
    rustbox.print(suggestion_box.x,
                  suggestion_box.y + suggestion_box.height,
                  rustbox::RB_NORMAL,
                  Color::White,
                  Color::Default,
                  "└");
    rustbox.print(suggestion_box.x + suggestion_box.width,
                  suggestion_box.y + suggestion_box.height,
                  rustbox::RB_NORMAL,
                  Color::White,
                  Color::Default,
                  "┘");

    // Horizontal lines
    for i in 1..suggestion_box.width {
        rustbox.print(suggestion_box.x + i,
                      suggestion_box.y,
                      rustbox::RB_NORMAL,
                      Color::White,
                      Color::Default,
                      "─");
        rustbox.print(suggestion_box.x + i,
                      suggestion_box.y + suggestion_box.height,
                      rustbox::RB_NORMAL,
                      Color::White,
                      Color::Default,
                      "─");
    }

    // Vertical lines
    for i in 1..suggestion_box.height {
        rustbox.print(suggestion_box.x,
                      suggestion_box.y + i,
                      rustbox::RB_NORMAL,
                      Color::White,
                      Color::Default,
                      "│");
        rustbox.print(suggestion_box.x + suggestion_box.width,
                      suggestion_box.y + i,
                      rustbox::RB_NORMAL,
                      Color::White,
                      Color::Default,
                      "│");
    }

    // inner text
    for i in 1..suggestion_box.width {
        for j in 1..suggestion_box.height {
           rustbox.print(suggestion_box.x + i,
                          suggestion_box.y + j,
                          rustbox::RB_NORMAL,
                          Color::White,
                          Color::Default,
                          " ");
        }
    }

    // Print suggestions
    let mut inc = 0;
    for suggestion in suggestion_box.suggestions {
        inc = inc + 1;
        rustbox.print(suggestion_box.x + inc + 1,
                      suggestion_box.y + 1,
                      rustbox::RB_NORMAL,
                      Color::White,
                      Color::Default,
                      suggestion.text);
        rustbox.print(suggestion_box.x + inc + 1 + suggestion.highlight_start,
                      suggestion_box.y + 1,
                      rustbox::RB_NORMAL,
                      Color::Blue,
                      Color::Default,
                      &suggestion.text[suggestion.highlight_start..suggestion.highlight_end]);
    }
}

fn draw_suggestion(rustbox: &RustBox, x: usize, y: usize) {
    let suggestion1 = Suggestion {
        selected: false,
        highlight_start: 2,
        highlight_end: 8,
        text: "suggestion1",
    };
    let mut suggestions = Vec::new();
    suggestions.push(suggestion1);

    let suggestion_box = SuggestionBox {
        x: x,
        y: y,
        width: 25,
        height: 10,
        suggestions: suggestions,
    };

    display_suggestionn_box(&rustbox, suggestion_box);
}

struct InterfaceStruct<'a> {
    r: &'a RustBox,
}

trait Interface<'a> {
    fn new(rustbox: &'a RustBox) -> InterfaceStruct;
    fn print(&self) -> TextArea;
    fn key_press(&self, &key: &Key, textarea: &mut TextArea);
}

impl<'a> Interface<'a> for InterfaceStruct<'a> {
    fn new(rustbox: &'a RustBox) -> InterfaceStruct {
        InterfaceStruct {
            r: rustbox
        }
    }

    fn print<'b>(&self) -> TextArea {
        let mut textarea = TextArea::new(
            &self.r,
            &BoxStruct { x: 2, y: 2, width: 20, height: 10, border: true }
        );

        textarea.display();

        self.r.print(40,
                      20,
                      rustbox::RB_NORMAL,
                      Color::Magenta,
                      Color::Default,
                      "(press CTRL+X and then CTRL+Q to exit)");

        textarea
    }

    fn key_press(&self, key: &Key, textarea: &mut TextArea) {
        match key {
            &Key::Char(c) => {
                textarea.push(c);
            }
            &Key::Backspace => {
                textarea.pop();
            }
            _ => {}
        }
        textarea.display();
        // self.r.print(1,
        //               1,
        //               rustbox::RB_NORMAL,
        //               Color::White,
        //               Color::Default,
        //               &textarea.text.to_string());
        // self.r.print(1, 1, rustbox::RB_NORMAL, Color::Default, Color::White, "s");

        draw_suggestion(self.r, textarea.x + textarea.cursor_position_x + 1, textarea.y + textarea.cursor_position_y + 2);
        //draw_key(&self.r, &key, &mut textarea);
    }
}

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let mut ctrl_xpressed: bool = false;

    let interface = InterfaceStruct::new(&rustbox);
    let mut textarea = interface.print();
    rustbox.present();

    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                if key == Key::Ctrl('q') && ctrl_xpressed {
                    break;
                }
                if key == Key::Ctrl('x') {
                    ctrl_xpressed = true;
                } else {
                    ctrl_xpressed = false;
                }
                rustbox.clear();
                interface.print();
                interface.key_press(&key, &mut textarea);
                rustbox.present();
            }
            Err(e) => panic!("{}", e.description()),
            _ => {}
        }
    }
}
