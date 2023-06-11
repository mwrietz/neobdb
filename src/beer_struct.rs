// beer_struct.rs
// 20230611

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
        let mut buff_tuple: Vec<(String, &str)> = Vec::new();
        buff_tuple.push((format!("{:7} ", "index:"),   "DARKBLUE"));
        buff_tuple.push((format!("{:<5} ", index),     "WHITE"));
        buff_tuple.push((format!("{:7} ", "name:"),    "DARKBLUE"));
        buff_tuple.push((format!("{:36} ", self.name), "DARKGREEN"));
        buff_tuple.push((format!("{:6} ", "style:"),   "DARKBLUE"));
        buff_tuple.push((format!("{:25}", self.style), "WHITE"));
        for b in buff_tuple {
            print_color(b.0.as_str(), b.1);
        }
        println!();

        // line 2
        let mut buff_tuple: Vec<(String, &str)> = Vec::new();
        buff_tuple.push((format!("{:7} ", "rating:"),    "DARKBLUE"));
        buff_tuple.push((format!("{:5} ", self.rating),  "WHITE"));
        buff_tuple.push((format!("{:7} ", "brewer:"),    "DARKBLUE"));
        buff_tuple.push((format!("{:36} ", self.brewer), "DARKYELLOW"));
        buff_tuple.push((format!("{:6} ", "notes:"),     "DARKBLUE"));
        buff_tuple.push((format!("{:25}", self.notes),   "WHITE"));
        for b in buff_tuple {
            print_color(b.0.as_str(), b.1);
        }
        println!();

        // line 3
        let mut buff_tuple: Vec<(String, &str)> = Vec::new();
        buff_tuple.push((format!("{:7} ", "abv:"),          "DARKBLUE"));
        buff_tuple.push((format!("{:5} ", self.abv),        "WHITE"));
        buff_tuple.push((format!("{:7} ", "time:"),         "DARKBLUE"));
        buff_tuple.push((format!("{:36} ", self.timestamp), "WHITE"));
        buff_tuple.push((format!("{:6} ", "uuid:"),         "DARKBLUE"));
        buff_tuple.push((format!("{:25}", self.id),         "WHITE"));
        for b in buff_tuple {
            print_color(b.0.as_str(), b.1);
        }
        println!();
    }

    pub fn print_summary(&self, index: usize) {
        print!("{:4} ", index);
        let buffer = format!("{:30} ", self.name);
        print_color_bold(buffer.as_str(), "DARKGREEN");
        print!("{:35} ", self.brewer);
        print!("{:20}", self.style);
        println!();
    }
}
