// 20230611

use rusqlite::Connection;

use crate::beer_struct::Beer;
use crate::db;
use crate::ui;
use crate::tui_gen;
use crate::tui_menu;

#[derive(PartialEq, Copy, Clone)]
pub enum State {
    Summary,
    Detail,
}

//#[derive(Copy, Clone)]
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
        let lines_per_record: usize = match self.state {
            State::Summary => 1,
            State::Detail => 4,
        };
        let record_limit = (self.height - 10) / lines_per_record;
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
        //tui_gen::cls();
        //print_header();

        let query = format!(
            "SELECT COUNT(*) FROM Beer
            WHERE name LIKE '%{}%' 
            OR brewer LIKE '%{}%' 
            OR style LIKE '%{}%' 
            OR abv LIKE '%{}%' 
            OR rating LIKE '%{}%' 
            OR notes LIKE '%{}%' 
            ORDER BY brewer, name",
            self.filter, 
            self.filter, 
            self.filter, 
            self.filter, 
            self.filter, 
            self.filter, 
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
            if self.state == ui::State::Summary {
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
                    //self.show(&conn);
                },
                'k' => {
                    // scroll_up
                    //let n_rows = db::count_rows_in_table(conn, "Beer");
                    if self.offset >= self.limit() {
                        self.offset -= self.limit();
                    }
                    //self.show(&conn);
                },
                'v' => {
                    if self.state == ui::State::Summary {
                        self.state = ui::State::Detail;
                        self.offset = 0;
                    } else {
                        self.state = ui::State::Summary;
                        self.offset = 0;
                    }
                    //self.show(&conn);
                },
                _ => {
                    self.filter_count = db::count_rows_in_table(&conn, "Beer");
                    self.filter = String::from(" ");
                    //self.show(&conn);
                    break;
                }
            }
        }
    }
    fn display_filter(&self) {
        tui_gen::cmove(72, 1);
        println!("Search String: '{}'", self.filter);
        if self.state == ui::State::Summary {
            tui_gen::cmove(0, 5);
        } else {
            tui_gen::cmove(0, 4);
        }
    }
}

/*
pub fn show_found(conn: &Connection, search_string: &str) {
    tui_gen::cls();
    print_header();

    let query = format!(
        "SELECT * FROM Beer
        WHERE name LIKE '%{}%' 
        OR brewer LIKE '%{}%' 
        OR style LIKE '%{}%' 
        OR abv LIKE '%{}%' 
        OR rating LIKE '%{}%' 
        OR notes LIKE '%{}%' 
        ORDER BY brewer, name",
        search_string, 
        search_string, 
        search_string, 
        search_string, 
        search_string, 
        search_string, 
    );

    let mut beers: Vec<Beer> = Vec::new();
    db::vec_from_query(conn, query.as_str(), &mut beers);

    let (_width, height) = tui_gen::tsize();
    let page_limit: i32 = (height as i32 - 10) / 4;

    let mut index: usize = 0;
    let mut count: i32 = 0;
    for b in &beers {
        b.print_details(index as usize);
        count += 1;
        index += 1;
        if count == page_limit {
            tui_gen::pause();
            tui_gen::cls();
            print_header();
            count = 0;
        }
    }
    println!("");
}
*/

/*
pub fn show_all(conn: &Connection) {
    tui_gen::cls();
    print_header();

    let n_rows = db::count_rows_in_table(conn, "Beer");
    let (_width, height) = tui_gen::tsize();
    let page_limit: i32 = (height as i32 - 6) / 4;
    let mut n_pages = 1;
    if n_rows > page_limit {
        n_pages = n_rows / page_limit + 1;
    }

    let mut index: usize = 0;
    for i in 0..n_pages {
        let query = format!(
            "SELECT * FROM Beer ORDER BY brewer, name LIMIT {} OFFSET {}",
            page_limit,
            i * page_limit,
        );
        let mut beers: Vec<Beer> = Vec::new();
        db::vec_from_query(conn, query.as_str(), &mut beers);

        for b in beers {
            b.print_details(index);
            index += 1;
        }
        if i < n_pages - 1 {
            tui_gen::pause();
            tui_gen::cls();
            print_header();
        }
    }
    println!("");
}
*/

/*
pub fn show_summary(conn: &Connection) {
    tui_gen::cls();
    print_header();
    print_summary_header();

    // determine number of pages required
    let n_rows = db::count_rows_in_table(conn, "Beer");
    let (_width, height) = tui_gen::tsize();
    let page_limit: i32 = height as i32 - 10;
    let mut n_pages = 1;
    if n_rows > page_limit {
        n_pages = n_rows / page_limit + 1;
    }

    // loop thru pages
    let mut index: usize = 0;
    for i in 0..n_pages {
        let query = format!(
            "SELECT * FROM Beer ORDER BY brewer, name LIMIT {} OFFSET {}",
            page_limit,
            i * page_limit,
        );
        let mut beers: Vec<Beer> = Vec::new();
        db::vec_from_query(conn, query.as_str(), &mut beers);

        for b in beers {
            b.print_summary(index);
            index += 1;
        }
        if i < n_pages - 1 {
            tui_gen::pause();
            tui_gen::cls();
            print_header();
            print_summary_header();
        }
    }
    println!("");
}
*/

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
