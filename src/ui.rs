// 20230623

//use crossterm::{
//    cursor,
//    execute,
//    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
//};
use crossterm::style::Color;

use rusqlite::Connection;

use crate::db;
use crate::tui_gen;

#[derive(PartialEq, Copy, Clone)]
pub enum State {
    Summary,
    Detail,
}

#[derive(Clone)]
pub struct View {
    pub state: State,
    pub height: usize,
    pub offset: usize,
    pub filter: String,
}

impl View {
    pub fn filter_count(self, conn: &Connection) -> usize {
        let query = match self.filter.len() {
            0 => db::query_full(),
            _ => db::query_filtered(&self),
        };
        let beers = db::vec_from_query(conn, query.as_str());
        beers.len()
    }

    pub fn limit(&self) -> usize {
        let lines_per_record: usize;
        let record_limit: usize;
        match self.state {
            State::Summary => {
                lines_per_record = 1;
                record_limit = (self.height - 8) / lines_per_record;
            }
            State::Detail => {
                lines_per_record = 4;
                record_limit = (self.height - 6) / lines_per_record;
            }
        };
        record_limit
    }

    pub fn show(mut self, conn: &Connection) {
        tui_gen::cls();
        //self.height = tui_gen::t_height();
        let (_w, h) = tui_gen::tsize();
        self.height = h;
        match self.state {
            State::Summary => {
                print_header();
                print_summary_header();
            }
            State::Detail => print_header(),
        }

        let query = db::query_for_display(&self);
        let beers = db::vec_from_query(conn, query.as_str());
        let mut index: usize = self.offset;
        for b in beers {
            match self.state {
                State::Summary => b.print_summary(index),
                State::Detail => b.print_details(index),
            }
            index += 1;
        }
        println!("");

        if self.filter.len() > 0 {
            self.display_filter();
        }
    }

    fn display_filter(&self) {
        //tui_gen::cmove(72, 1);
        tui_gen::cursor_move(72, 1);
        print!("Search String: '");
        //tui_gen::print_color(self.filter.as_str(), "DARKGREEN");
        tui_gen::print_color(self.filter.as_str(), Color::DarkGreen);
        print!("'");
        match self.state {
            State::Summary => tui_gen::cursor_move(0, 5),
            State::Detail => tui_gen::cursor_move(0, 4),
        }
    }
}

pub fn print_summary_header() {
    let header_string = format!(
        "{:4} {:30} {:35} {:20}\n",
        "idx:", "name:", "brewer:", "style:"
    );
    //tui_gen::print_color(&header_string, "DARKBLUE");
    tui_gen::print_color(&header_string, Color::DarkBlue);
}
pub fn print_header() {
    let title = "DEFINITIVE BEER DATABASE";
    //tui_gen::print_title(&title, "DARKBLUE");
    tui_gen::print_title(&title, Color::DarkBlue);
    tui_gen::cursor_move(50, 1);
    //tui_gen::print_color("(", "DARKBLUE");
    tui_gen::print_color("(", Color::DarkBlue);
    //tui_gen::print_color(tui_gen::get_prog_name().as_str(), "DARKGREEN");
    //tui_gen::print_color(tui_gen::get_prog_name().as_str(), Color::DarkGreen);
    tui_gen::print_color(tui_gen::get_prog_name().as_str(), Color::Rgb{r:255, g:135, b:0});
    //Color::Rgb{r:255, g:135, b:0}   //tui_gen::print_color(
    //    format!(" v{}", env!("CARGO_PKG_VERSION")).as_str(),
    //    "DARKBLUE",
    //);
    tui_gen::print_color(
        format!(" v{}", env!("CARGO_PKG_VERSION")).as_str(),
       Color::Rgb{r:255, g:135, b:0},
    );
    //tui_gen::print_color(")", "DARKBLUE");
    tui_gen::print_color(")", Color::DarkBlue);
    tui_gen::cursor_move(0, 4);
}
