extern crate rustbox;

use lib::{BoxStruct, TextArea};
mod lib;

use std::cmp;
use std::error::Error;
use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

#[derive(Clone, Copy)]
#[derive(Debug)]
struct Suggestion {
    selected: bool,
    highlight_start: usize,
    highlight_end: usize,
    text: &'static str,
}

struct SuggestionBox {
    x: usize,
    y: usize,
    pos: usize,
    width: usize,
    height: usize,
    suggestions: Vec<Suggestion>,
}

struct InterfaceStruct<'a> {
    r: &'a RustBox,
    textarea: &'a mut TextArea<'a>,
    SuggestionBox: &'a mut SuggestionBox
}

impl<'a> InterfaceStruct<'a> {
    fn print(&mut self) {
        self.textarea.display();
        self.r.print(
            40, 20,
            rustbox::RB_NORMAL,
            Color::Magenta,
            Color::Default,
            "(press CTRL+X and then CTRL+Q to exit)"
        );
    }

    fn key_press(&mut self, key: &Key) {
        match key {
            &Key::Char(c) => {
                self.textarea.push(c);
            },
            &Key::Backspace => {
                self.textarea.pop();
            },
            &Key::Left => {
                self.textarea.move_cursor(-1, 0);
            },
            &Key::Right => {
                self.textarea.move_cursor(1, 0);
            },
            &Key::Up => {
                self.textarea.move_cursor(0, -1);
            },
            &Key::Down => {
                self.textarea.move_cursor(0, 1);
            },
            _ => {}
        }
        self.textarea.display();
        let x = self.textarea.x + self.textarea.cursor_position_x + 1;
        let y = self.textarea.y + self.textarea.cursor_position_y + 2;
        self.draw_suggestion(&key, x, y);
    }

    fn draw_suggestion(&mut self, key: &Key, x: usize, y: usize) {
        let suggestion1 = Suggestion {
            selected: true,
            highlight_start: 2,
            highlight_end: 8,
            text: "suggestion1",
        };
        let suggestion2 = Suggestion {
            selected: false,
            highlight_start: 0,
            highlight_end: 3,
            text: "suggestion2",
        };
        let suggestion3 = Suggestion {
            selected: false,
            highlight_start: 0,
            highlight_end: 5,
            text: "suggestion3",
        };
        let suggestion4 = Suggestion {
            selected: false,
            highlight_start: 2,
            highlight_end: 4,
            text: "suggestion4",
        };
        self.SuggestionBox.suggestions = Vec::new();
        self.SuggestionBox.suggestions.push(suggestion1);
        self.SuggestionBox.suggestions.push(suggestion2);
        self.SuggestionBox.suggestions.push(suggestion3);
        self.SuggestionBox.suggestions.push(suggestion4);

        let len = self.SuggestionBox.suggestions.len();
        let mut max_value = 0;
        for s in  self.SuggestionBox.suggestions.iter().cloned() {
          max_value = cmp::max(max_value, s.text.len());
        }
        self.SuggestionBox.x = x;
        self.SuggestionBox.y = y;
        self.SuggestionBox.width = max_value + 3;
        self.SuggestionBox.height = len + 1;

        let mut pos = self.SuggestionBox.pos;
        match key {
            &Key::Up => {
                pos = self.SuggestionBox.pos - 1;
                if (pos == 0) {
                    pos = len;
                }
            },
            &Key::Down => {
                pos = self.SuggestionBox.pos + 1;
                if (len + 1 == pos) {
                    pos = 1;
                }
            },
            _ => {}
        }
        self.SuggestionBox.pos = pos;
        self.display_suggestionn_box(key);
    }

    fn display_suggestionn_box(&self, key: &Key) {
        // Print suggestions
        let mut inc = 0;
        for suggestion in self.SuggestionBox.suggestions.iter() {
            let mut style = Color::Default;
            let mut color_text = Color::White;
            if (self.SuggestionBox.pos == inc + 1) {
                style =  Color::White;
                color_text = Color::Black;
            }
            self.r.print(
                self.SuggestionBox.x + 2,
                self.SuggestionBox.y + inc + 1,
                rustbox::RB_NORMAL,
                color_text,
                style,
                suggestion.text
            );
            self.r.print(self.SuggestionBox.x + 2 + suggestion.highlight_start,
                          self.SuggestionBox.y + inc + 1,
                          rustbox::RB_BOLD,
                          Color::Cyan,
                          style,
                          &suggestion.text[suggestion.highlight_start..suggestion.highlight_end]);
            inc = inc + 1;
        }
        let ref suggestion_box = self.SuggestionBox;
        // The 4th corners
        self.r.print(suggestion_box.x,
                      suggestion_box.y,
                      rustbox::RB_NORMAL,
                      Color::White,
                      Color::Default,
                      "┌");
        self.r.print(suggestion_box.x + suggestion_box.width,
                      suggestion_box.y,
                      rustbox::RB_NORMAL,
                      Color::White,
                      Color::Default,
                      "┐");
        self.r.print(suggestion_box.x,
                      suggestion_box.y + suggestion_box.height,
                      rustbox::RB_NORMAL,
                      Color::White,
                      Color::Default,
                      "└");
        self.r.print(suggestion_box.x + suggestion_box.width,
                      suggestion_box.y + suggestion_box.height,
                      rustbox::RB_NORMAL,
                      Color::White,
                      Color::Default,
                      "┘");

        // Horizontal lines
        for i in 1..suggestion_box.width {
            self.r.print(suggestion_box.x + i,
                          suggestion_box.y,
                          rustbox::RB_NORMAL,
                          Color::White,
                          Color::Default,
                          "─");
            self.r.print(suggestion_box.x + i,
                          suggestion_box.y + suggestion_box.height,
                          rustbox::RB_NORMAL,
                          Color::White,
                          Color::Default,
                          "─");
        }

        // Vertical lines
        for i in 1..suggestion_box.height {
            self.r.print(suggestion_box.x,
                          suggestion_box.y + i,
                          rustbox::RB_NORMAL,
                          Color::White,
                          Color::Default,
                          "│");
            self.r.print(suggestion_box.x + suggestion_box.width,
                          suggestion_box.y + i,
                          rustbox::RB_NORMAL,
                          Color::White,
                          Color::Default,
                          "│");
        }
    }
}

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let mut ctrl_xpressed: bool = false;
    let mut suggestionBox = SuggestionBox {
        x: 0,
        y: 0,
        pos: 0,
        width: 0,
        height: 0,
        suggestions: Vec::new()
    };
    let mut textarea = TextArea::new(
        &rustbox,
        &BoxStruct { x: 2, y: 2, width: 20, height: 10, border: true }
    );
    let mut interface = InterfaceStruct {
        r: &rustbox,
        textarea: &mut textarea,
        SuggestionBox: &mut suggestionBox
    };
    interface.print();
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
                interface.key_press(&key);
                rustbox.present();
            }
            Err(e) => panic!("{}", e.description()),
            _ => {}
        }
    }
}
