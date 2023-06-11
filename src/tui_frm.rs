#![allow(dead_code)]

use crate::tui_gen::cmove;
use crate::tui_gen::print_color;
//use colored::Colorize;

pub struct Frame<'a> {
    pub title: &'a str,
    pub title_color: &'a str,
    pub frame_color: &'a str,
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

impl Frame<'_> {
    pub fn clear(&self) {
        // draw middle
        for i in 0..(self.h - 1) {
            cmove(self.x + 1, self.y + i + 1);
            for _j in 0..(self.w - 2) {
                print!(" ");
            }
        }
    }
    pub fn display(&self) {
        //let ul = "╭".color(self.frame_color);
        //let ur = "╮".color(self.frame_color);
        //let ll = "╰".color(self.frame_color);
        //let lr = "╯".color(self.frame_color);
        //let hor = "─".color(self.frame_color);
        //let ver = "│".color(self.frame_color);
        let ul = "╭";
        let ur = "╮";
        let ll = "╰";
        let lr = "╯";
        let hor = "─";
        let ver = "│";

        // draw top horizontal
        cmove(self.x, self.y);
        //print!("{}", ul);
        print_color(ul, self.frame_color);
        for _i in 0..(self.w - 2) {
            //print!("{}", hor);
            print_color(hor, self.frame_color);
        }
        //print!("{}", ur);
        print_color(ur, self.frame_color);

        // draw middle
        for i in 0..(self.h - 1) {
            cmove(self.x, self.y + i + 1);
            //print!("{}", ver);
            print_color(ver, self.frame_color);
            for _j in 0..(self.w - 2) {
                print!(" ");
            }
            //print!("{}", ver);
            print_color(ver, self.frame_color);
        }

        // draw bottom horizontal
        cmove(self.x, self.y + self.h);
        //print!("{}", ll);
        print_color(ll, self.frame_color);
        for _i in 0..(self.w - 2) {
            //print!("{}", hor);
            print_color(hor, self.frame_color);
        }
        //println!("{}", lr);
        print_color(lr, self.frame_color);
        println!();

        if self.title.len() > 0 {
            // print title
            cmove(self.x + 2, self.y);
            //print!(" {} ", self.title.color(self.title_color));
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
                    cmove(
                        self.frame.x + 2,
                        self.frame.y + (i - (self.msg.len() - self.frame.h)),
                    );
                    print!("{}", self.msg[i]);
                }
            } else {
                cmove(self.frame.x + 2, self.frame.y + (i + 1));
                print!("{}", self.msg[i]);
            }
        }
    }
}
