extern crate rustbox;

use rustbox::{Color, RustBox};

#[derive(Clone)]
struct TextError {
    position_start: usize,
    position_end: usize,
}

pub struct BoxStruct {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub border: bool
}

#[derive(Clone)]
pub struct TextArea<'a> {
    rustbox: &'a RustBox,
    pub x: usize,
    pub y: usize,
    width: usize,
    height: usize,
    border: bool,
    pub words: Vec<String>,
    pub text: Vec<String>,
    pub cursor_position_x: usize,
    pub cursor_position_y: usize,
    errors: Vec<TextError>,
}

pub trait Box<'a> {
    fn new(rustbox: &'a RustBox, box_s: &BoxStruct) -> TextArea<'a>;
    fn push(&mut self, character: char);
    fn pop(&mut self);
    fn display(&mut self);
}

impl<'a> Box<'a> for TextArea<'a> {
    fn new(rustbox: &'a RustBox, box_s: &BoxStruct) -> TextArea<'a> {
        let mut text = Vec::new();
        text.push(String::new());
        TextArea {
            rustbox: rustbox,
            x: box_s.x, y: box_s.y, width: box_s.width, height: box_s.height,
            border: box_s.border,
            words: Vec::new(), text: text,
            cursor_position_x: 0, cursor_position_y: 0,
            errors: Vec::new()
        }
    }
    fn push(&mut self, character: char) {
        if self.cursor_position_y + 2 == self.height
        && self.cursor_position_x + 1 == self.width {
            return;
        }
        if self.cursor_position_x + 1 == self.width {
            self.cursor_position_x = 0;
            self.cursor_position_y += 1;
        }
        else {
            self.text.push(String::new());
        }
        self.cursor_position_x += 1;
        self.text[self.cursor_position_y].push(character);
    }

    fn pop(&mut self) {
        if self.cursor_position_y == 0
        && self.cursor_position_x == 0 {
            return;
        }
        if self.cursor_position_x == 0 {
            self.cursor_position_x = self.width;
            self.cursor_position_y -= 1;
        }
        self.cursor_position_x -= 1;
        self.text[self.cursor_position_y].pop();
    }

    fn display(&mut self) {
        // The 4th corners
        self.rustbox.print(self.x,
                      self.y,
                      rustbox::RB_NORMAL,
                      Color::White,
                      Color::Default,
                      "┌");
        self.rustbox.print(self.x + self.width,
                      self.y,
                      rustbox::RB_NORMAL,
                      Color::White,
                      Color::Default,
                      "┐");
        self.rustbox.print(self.x,
                      self.y + self.height,
                      rustbox::RB_NORMAL,
                      Color::White,
                      Color::Default,
                      "└");
        self.rustbox.print(self.x + self.width,
                      self.y + self.height,
                      rustbox::RB_NORMAL,
                      Color::White,
                      Color::Default,
                      "┘");

        // Horizontal lines
        for i in 1..self.width {
            self.rustbox.print(self.x + i,
                          self.y,
                          rustbox::RB_NORMAL,
                          Color::White,
                          Color::Default,
                          "─");
            self.rustbox.print(self.x + i,
                          self.y + self.height,
                          rustbox::RB_NORMAL,
                          Color::White,
                          Color::Default,
                          "─");
        }

        // Vertical lines
        for i in 1..self.height {
            self.rustbox.print(self.x,
                          self.y + i,
                          rustbox::RB_NORMAL,
                          Color::White,
                          Color::Default,
                          "│");
            self.rustbox.print(self.x + self.width,
                          self.y + i,
                          rustbox::RB_NORMAL,
                          Color::White,
                          Color::Default,
                          "│");
        }

        // cursor
        self.rustbox.set_cursor(
          (self.x + self.cursor_position_x + 1) as isize,
          (self.y + self.cursor_position_y + 1) as isize
        );

        // inner text
        for i in 3..self.width + 2 {
            for j in 3..self.height + 2 {
                self.rustbox.print(i,
                              j,
                              rustbox::RB_NORMAL,
                              Color::White,
                              Color::Default,
                              " ");
            }
        }

        let mut inc = 0;
        for line in &self.text {
            self.rustbox.print(self.x + 1,
                                self.y + inc + 1,
                                  rustbox::RB_NORMAL,
                                  Color::White,
                                  Color::Default,
                                  &line.to_string()
                                );
            inc += 1;
        }
    }
}