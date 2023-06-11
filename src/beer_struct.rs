// beer_struct.rs
// 20230604

use serde::{Deserialize, Serialize};

use crate::tui_gen::print_color;
use crate::tui_gen::print_color_bold;

#[derive(Debug, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
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
        let buffer = format!("{:7} ", "index:");
        print_color(buffer.as_str(), "DARKBLUE");

        let buffer = format!("{:<5} ", index);
        print_color(buffer.as_str(), "WHITE");

        let buffer = format!("{:7} ", "name:");
        print_color(buffer.as_str(), "DARKBLUE");

        let buffer = format!("{:36} ", self.name);
        print_color_bold(buffer.as_str(), "DARKGREEN");

        let buffer = format!("{:6} ", "style:");
        print_color(buffer.as_str(), "DARKBLUE");

        let buffer = format!("{:25}", self.style);
        print_color(buffer.as_str(), "WHITE");
        println!("");

        // line 2
        let buffer = format!("{:7} ", "rating:");
        print_color(buffer.as_str(), "DARKBLUE");

        let buffer = format!("{:5} ", self.rating);
        print_color(buffer.as_str(), "WHITE");

        let buffer = format!("{:7} ", "brewer:");
        print_color(buffer.as_str(), "DARKBLUE");

        let buffer = format!("{:36} ", self.brewer);
        print_color(buffer.as_str(), "DARKYELLOW");

        let buffer = format!("{:6} ", "notes:");
        print_color(buffer.as_str(), "DARKBLUE");

        let buffer = format!("{:25}", self.notes);
        print_color(buffer.as_str(), "WHITE");
        println!("");

        // line 3
        let buffer = format!("{:7} ", "abv:");
        print_color(buffer.as_str(), "DARKBLUE");

        let buffer = format!("{:5} ", self.abv);
        print_color(buffer.as_str(), "WHITE");

        let buffer = format!("{:7} ", "time:");
        print_color(buffer.as_str(), "DARKBLUE");

        let buffer = format!("{:36} ", self.timestamp);
        print_color(buffer.as_str(), "WHITE");

        let buffer = format!("{:6} ", "uuid:");
        print_color(buffer.as_str(), "DARKBLUE");

        let buffer = format!("{:25}", self.id);
        print_color(buffer.as_str(), "WHITE");
        println!("");
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
