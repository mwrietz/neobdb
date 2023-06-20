// 20230616

use rusqlite::Connection;

use crate::beer_struct::Beer;
use crate::db;
use crate::tui_gen;
use crate::tui_menu;

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
    pub filter_count: usize,
    pub filter: String,
}

impl View {
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

    pub fn show(self, conn: &Connection) {
        match self.state {
            State::Summary => self.summary(conn),
            State::Detail => self.detail(conn),
        };
    }

    pub fn summary(mut self, conn: &Connection) {
        tui_gen::cls();
        print_header();
        print_summary_header();

        self.height = tui_gen::t_height();

        let query = format!(
            "SELECT * FROM Beer ORDER BY brewer, name LIMIT {} OFFSET {}",
            self.limit(),
            self.offset,
        );
        let mut beers: Vec<Beer> = Vec::new();
        db::vec_from_query(conn, query.as_str(), &mut beers);

        let mut index: usize = self.offset;
        for b in beers {
            b.print_summary(index);
            index += 1;
        }
        println!("");
    }

    pub fn detail(mut self, conn: &Connection) {
        tui_gen::cls();
        print_header();

        self.height = tui_gen::t_height();

        let query = format!(
            "SELECT * FROM Beer ORDER BY brewer, name LIMIT {} OFFSET {}",
            self.limit(),
            self.offset,
        );
        let mut beers: Vec<Beer> = Vec::new();
        db::vec_from_query(conn, query.as_str(), &mut beers);

        let mut index: usize = self.offset;
        for b in beers {
            b.print_details(index);
            index += 1;
        }
        println!("");
    }

    pub fn find(&mut self, conn: &Connection) {
        let query = format!(
            "SELECT COUNT(*) FROM Beer
            WHERE name LIKE '%{}%' 
            OR brewer LIKE '%{}%' 
            OR style LIKE '%{}%' 
            OR abv LIKE '%{}%' 
            OR rating LIKE '%{}%' 
            OR notes LIKE '%{}%' 
            ORDER BY brewer, name",
            self.filter, self.filter, self.filter, self.filter, self.filter, self.filter,
        );

        self.filter_count = db::count_rows_in_query(conn, query.as_str());

        let find_menu_items = vec![
            ("j", "Scroll_DN"),
            ("k", "Scroll_UP"),
            ("v", "Detail/Summary"),
            ("c", "Clear Search"),
        ];

        self.offset = 0;

        loop {
            let query = format!(
                "SELECT * FROM Beer
                WHERE name LIKE '%{}%' 
                OR brewer LIKE '%{}%' 
                OR style LIKE '%{}%' 
                OR abv LIKE '%{}%' 
                OR rating LIKE '%{}%' 
                OR notes LIKE '%{}%' 
                ORDER BY brewer, name
                LIMIT {}
                OFFSET {}",
                self.filter,
                self.filter,
                self.filter,
                self.filter,
                self.filter,
                self.filter,
                self.limit(),
                self.offset
            );

            let mut beers: Vec<Beer> = Vec::new();
            db::vec_from_query(conn, query.as_str(), &mut beers);

            tui_gen::cls();
            print_header();
            if self.state == State::Summary {
                print_summary_header();
            }
            self.display_filter();

            let mut index: usize = self.offset;
            for b in beers {
                if self.state == State::Detail {
                    b.print_details(index);
                } else {
                    b.print_summary(index);
                }
                index += 1;
            }
            println!("");

            let selection = tui_menu::menu_horiz(&find_menu_items);
            match selection {
                'j' => {
                    // scroll_dn if not last page
                    if (self.offset + self.limit()) < self.filter_count {
                        self.offset += self.limit();
                    }
                }
                'k' => {
                    // scroll_up
                    if self.offset >= self.limit() {
                        self.offset -= self.limit();
                    }
                }
                'v' => {
                    if self.state == State::Summary {
                        self.state = State::Detail;
                        self.offset = 0;
                    } else {
                        self.state = State::Summary;
                        self.offset = 0;
                    }
                }
                _ => {
                    self.filter_count = db::count_rows_in_table(&conn, "Beer");
                    self.filter = String::from(" ");
                    break;
                }
            }
        }
    }
    fn display_filter(&self) {
        tui_gen::cmove(72, 1);
        print!("Search String: '");
        tui_gen::print_color(self.filter.as_str(), "DARKGREEN");
        print!("'");
        /*
        if self.state == State::Summary {
            tui_gen::cmove(0, 5);
        } else {
            tui_gen::cmove(0, 4);
        }
        */
        match self.state {
            State::Summary => tui_gen::cmove(0, 5),
            State::Detail => tui_gen::cmove(0, 4),
        }
    }
}

pub fn print_summary_header() {
    let header_string = format!(
        "{:4} {:30} {:35} {:20}\n",
        "idx:", "name:", "brewer:", "style:"
    );
    tui_gen::print_color(&header_string, "DARKBLUE");
}
pub fn print_header() {
    let title = "DEFINITIVE BEER DATABASE";
    tui_gen::print_title(&title, "DARKBLUE");
    tui_gen::cmove(50, 1);
    tui_gen::print_color("(", "DARKBLUE");
    tui_gen::print_color(tui_gen::get_prog_name().as_str(), "DARKGREEN");
    tui_gen::print_color(
        format!(" v{}", env!("CARGO_PKG_VERSION")).as_str(),
        "DARKBLUE",
    );
    tui_gen::print_color(")", "DARKBLUE");
    tui_gen::cmove(0, 4);
}

pub fn usage() {
    tui_gen::cls();
    print_header();
    println!("Invalid arguments provided...");
    println!("");
    println!("USAGE:");
    println!("    ./bdb [OPTION]");
    println!("");
    println!("OPTIONS:");
    println!("    -a,  --add                    Add new record to database");
    println!("    -r,  --remove                 Remove record from database");
    println!("    -e,  --edit                   Edit existing record");
    println!("    -f,  --find <SEARCHSTRING>    Find records with searchstring (eg. ./bdb -f searchstring)");
    println!("    -sa, --show_all               Show all records in database (detailed)");
    println!("    -ss, --show_sum               Show all records in database (summary)");
    println!("    -p,  --pdf                    Create a pdf of database");
    println!("    -m,  --menu                   Display menu");
    println!("");
}
