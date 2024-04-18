#![allow(dead_code)]

use crossterm::style::Color;
use crate::tui_gen::{cursor_move, print_color};

pub struct Frame<'a> {
    pub title: &'a str,
    pub title_color: Color,
    pub frame_color: Color,
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

impl Frame<'_> {
    pub fn clear(&self) {
        // draw middle
        for i in 0..(self.h - 1) {
            cursor_move(self.x + 1, self.y + i + 1);
            for _j in 0..(self.w - 2) {
                print!(" ");
            }
        }
    }
    pub fn display(&self) {
        let ul = "╭";
        let ur = "╮";
        let ll = "╰";
        let lr = "╯";
        let hor = "─";
        let ver = "│";

        // draw top horizontal
        cursor_move(self.x, self.y);
        print_color(ul, self.frame_color);
        for _i in 0..(self.w - 2) {
            print_color(hor, self.frame_color);
        }
        print_color(ur, self.frame_color);

        // draw middle
        for i in 0..(self.h - 1) {
            cursor_move(self.x, self.y + i + 1);
            print_color(ver, self.frame_color);
            for _j in 0..(self.w - 2) {
                print!(" ");
            }
            print_color(ver, self.frame_color);
        }

        // draw bottom horizontal
        cursor_move(self.x, self.y + self.h);
        print_color(ll, self.frame_color);
        for _i in 0..(self.w - 2) {
            print_color(hor, self.frame_color);
        }
        print_color(lr, self.frame_color);
        println!();

        if !self.title.is_empty() {
            cursor_move(self.x + 2, self.y);
            print!(" ");
            print_color(self.title, self.title_color);
            print!(" ");
        }
    }
}

pub struct MsgFrame<'a> {
    pub frame: Frame<'a>,
    pub msg: Vec<&'a str>,
}

impl MsgFrame<'_> {
    pub fn display_msg(&self) {
        for i in 0..self.msg.len() {
            if self.msg.len() > (self.frame.h - 1) {
                if i > (self.msg.len() - self.frame.h) {
                    cursor_move(
                        self.frame.x + 2,
                        self.frame.y + (i - (self.msg.len() - self.frame.h)),
                    );
                    print!("{}", self.msg[i]);
                }
            } else {
                cursor_move(self.frame.x + 2, self.frame.y + (i + 1));
                print!("{}", self.msg[i]);
            }
        }
    }
}
