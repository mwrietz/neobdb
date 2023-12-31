#![allow(dead_code)]

//use crossterm::style::ResetColor;
//use colored::Colorize;
use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use getch::Getch;
use std::io;
use std::io::{stdout, Write};

use crate::tui_gen;
use crate::tui_gen::cursor_move;
use crate::tui_gen::horiz_line;
use crate::tui_gen::print_color;
use crate::tui_gen::tsize;
//use crate::tui_gen::get_prog_name;
//mod tui_gen;

pub fn menu(menu_title: &str, items: &Vec<&str>) -> u8 {
    println!("{}", menu_title);
    for (i, item) in items.iter().enumerate() {
        println!("    {}) {}", i + 1, item);
    }

    println!();
    print!("Selection: ");
    io::stdout().flush().unwrap();

    let mut _a: u8 = 0;
    let menu_len = items.len();
    loop {
        let g = Getch::new();
        _a = g.getch().unwrap();
        if _a <= 48 || _a > (48 + menu_len as u8) {
            continue;
        }
        break;
    }

    println!();

    _a - 48
}

// menu_h - example use
//
//    let menu_items = vec![
//        ("a", "Add"),
//        ("r", "Remove"),
//        ("e", "Edit"),
//        ("d", "Details"),
//        ("s", "Summary"),
//        ("m", "Menu"),
//        ("q", "Quit")
//    ];

//    let val = menu_horiz(menu_items);

//pub fn menu_horiz(items: &Vec<(&str, &str)>) -> char {
pub fn menu_horiz(items: &[(&str, &str)]) -> char {
    let (_width, height) = tsize();
    cursor_move(0, height - 2);

    horiz_line(Color::Blue);
    for item in items.iter() {
        //print!("{:>4}:{}", item.0.green(), item.1);
        let buffer = format!("{:>4}", item.0);
        print_color(&buffer, Color::DarkGreen);
        let buffer = format!(":{}", item.1);
        print_color(&buffer, Color::White);
    }
    execute!(stdout(), cursor::Hide).unwrap();
    io::stdout().flush().unwrap();

    let mut _a: u8 = 0;
    loop {
        let mut flag = false;
        let g = Getch::new();
        _a = g.getch().unwrap();

        for item in items.iter() {
            //let ch = item.0.chars().nth(0).unwrap();
            let ch = item.0.chars().next().unwrap();
            if (_a as char) == ch {
                flag = true;
                break;
            }
        }
        if flag {
            break;
        }
    }

    _a as char
}

//pub fn menu_horiz_neo(items: &Vec<(&str, &str)>) -> char {
pub fn menu_horiz_neo(items: &[(&str, &str)]) -> char {
    let (_width, height) = tsize();
    cursor_move(0, height - 1);

    // horiz_line(Color::Blue);
    print_title_block();

    for item in items.iter() {
        //print!("{:>4}:{}", item.0.green(), item.1);
        let buffer = format!("{:>4}", item.0);
        print_color(&buffer, Color::DarkGreen);
        let buffer = format!(":{}", item.1);
        //print_color(&buffer, Color::White);
        print_color(&buffer, Color::Grey);
    }
    execute!(stdout(), cursor::Hide).unwrap();
    io::stdout().flush().unwrap();

    let mut _a: u8 = 0;
    loop {
        let mut flag = false;
        let g = Getch::new();
        _a = g.getch().unwrap();

        for item in items.iter() {
            //let ch = item.0.chars().nth(0).unwrap();
            let ch = item.0.chars().next().unwrap();
            if (_a as char) == ch {
                flag = true;
                break;
            }
        }
        if flag {
            break;
        }
    }

    _a as char
}

fn print_title_block() {
    let prog_name = tui_gen::get_prog_name();
    execute!(
        stdout(),
        SetForegroundColor(Color::Black),
        // 208 DarkOrange 255,135,0
        SetBackgroundColor(Color::Rgb {
            r: 255,
            g: 135,
            b: 0
        }),
        //Print(" tui_menu "),
        Print(format!(" {} ", prog_name)),
        ResetColor
    )
    .expect("print_title_block error");
}
