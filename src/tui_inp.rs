#![allow(dead_code)]

use crossterm::style::Color;
use std::io::Write;

use crate::tui_gen::{cursor_move, print_color, tsize};
use crate::tui_frm::Frame;

// replaces get_int(), get_float()
pub fn get_val<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        let mut buffer = String::new();
        print!("{}", prompt);

        std::io::stdout().flush().unwrap();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        let val: T = match buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return val;
    }
}

// replaces get_int_default(), get_float_default()
pub fn get_val_default<T: std::str::FromStr + std::fmt::Display>(prompt: &str, default: T) -> T {
    loop {
        let mut buffer = String::new();
        print!("{} [{:.3}]: ", prompt, default);

        std::io::stdout().flush().unwrap();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        if buffer.eq("\n") {
            return default;
        }

        let val: T = match buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return val;
    }
}

pub fn get_string(prompt: &str) -> String {
    let mut buffer = String::new();
    print!("{}", prompt);

    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    while buffer.ends_with('\n') || buffer.ends_with('\r') {
        buffer.pop();
    }
    //return buffer;
    buffer
}

pub fn get_string_default(prompt: &str, default: &str) -> String {
    let mut buffer = String::new();
    print!("{} [{}]: ", prompt, default);

    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    while buffer.ends_with('\n') || buffer.ends_with('\r') {
        buffer.pop();
    }

    if buffer.eq("") {
        default.to_string()
    } else {
        buffer
    }
}

pub fn dialog_box_get_string(width: usize, height: usize, title: &str, prompt: &str) -> String {
    let (term_width, term_height) = tsize();
    let x = (term_width - width) / 2;
    let y = (term_height - height) / 2;

    let frm = Frame {
        title,
        //title_color: "white",
        title_color: Color::White,
        //frame_color: "white",
        frame_color: Color::White,
        x,
        y,
        w: width,
        h: height,
    };
    frm.display();

    // print title and get string
    cursor_move(x + 2, y);
    print!(" ");
    print_color(title, Color::Red);
    print!(" ");
    cursor_move(x + 3, y + 2);

    get_string(prompt)
}
