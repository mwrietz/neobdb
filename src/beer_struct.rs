// beer_struct.rs
// 20231026

use crossterm::style::Color;

use serde::{Deserialize, Serialize};

use crate::tui_gen::print_color;
use crate::tui_gen::print_color_bold;

#[derive(Debug, Serialize, Deserialize)]
pub struct Beer {
    pub id: String,
    pub timestamp: String,
    pub name: String,
    pub brewer: String,
    pub style: String,
    pub abv: String,
    pub rating: String,
    pub notes: String,
}

impl Beer {
    pub fn print_details(&self, index: usize) {
        // line 1
        let mut buff_tuple: Vec<(String, Color)> = Vec::new();
        buff_tuple.push((format!("{:7} ", "index:"), Color::DarkGrey));
        buff_tuple.push((format!("{:<5} ", index), Color::White));
        buff_tuple.push((format!("{:5} ", "name:"), Color::DarkGrey));
        buff_tuple.push((format!("{:37} ", self.name), Color::DarkGreen));
        buff_tuple.push((format!("{:7} ", "brewer:"), Color::DarkGrey));
        buff_tuple.push((format!("{:36} ", self.brewer), Color::Rgb{r:255, g:135, b:0}));
        for b in buff_tuple {
            print_color(b.0.as_str(), b.1);
        }
        println!();
        // line 2
        let mut buff_tuple: Vec<(String, Color)> = Vec::new();
        buff_tuple.push((format!("{:7} ", "rating:"), Color::DarkGrey));
        buff_tuple.push((format!("{:5} ", self.rating), Color::White));
        buff_tuple.push((format!("{:5} ", "time:"), Color::DarkGrey));
        buff_tuple.push((format!("{:37} ", self.timestamp), Color::White));
        buff_tuple.push((format!("{:7} ", "style:"), Color::DarkGrey));
        buff_tuple.push((format!("{:36}", self.style), Color::White));
        for b in buff_tuple {
            print_color(b.0.as_str(), b.1);
        }
        println!();

        // line 3
        let mut buff_tuple: Vec<(String, Color)> = Vec::new();
        buff_tuple.push((format!("{:7} ", "abv:"), Color::DarkGrey));
        buff_tuple.push((format!("{:5} ", self.abv), Color::White));
        buff_tuple.push((format!("{:5} ", "uuid:"), Color::DarkGrey));
        buff_tuple.push((format!("{:37} ", self.id.trim_end_matches('\n')), Color::White));
        buff_tuple.push((format!("{:7} ", "notes:"), Color::DarkGrey));
        buff_tuple.push((format!("{:36}", self.notes), Color::White));
        for b in buff_tuple {
            print_color(b.0.as_str(), b.1);
        }
        println!();
        println!();
    }

    pub fn print_summary(&self, index: usize) {
        //print!("{:4} ", index);
        let buffer = format!("{:4} ", index);
        print_color(buffer.as_str(), Color::DarkGrey);
        let buffer = format!("{:30} ", self.name);
        print_color_bold(buffer.as_str(), Color::DarkGreen);
        print!("{:35} ", self.brewer);
        print!("{:20}", self.style);
        println!();
    }
}
