use rusqlite::Connection;

use crate::beer_struct::Beer;
use crate::db;
use crate::tui_gen;

pub fn show_found(conn: &Connection, search_string: &str) {
    tui_gen::cls();
    print_header();

    let mut where_string: String = format!("name LIKE '%{}%'", search_string);
    where_string.push_str(format!(" OR brewer LIKE '%{}%'", search_string).as_str());
    where_string.push_str(format!(" OR style LIKE '%{}%'", search_string).as_str());
    where_string.push_str(format!(" OR abv LIKE '%{}%'", search_string).as_str());
    where_string.push_str(format!(" OR rating LIKE '%{}%'", search_string).as_str());
    where_string.push_str(format!(" OR notes LIKE '%{}%'", search_string).as_str());

    let query = format!(
        "SELECT * FROM Beer WHERE {} ORDER BY brewer, name",
        where_string 
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

pub fn print_summary_header() {
    let header_string = format!(
        "{:4} {:30} {:35} {:20}\n",
        "idx:", "name:", "brewer:", "style:"
    );
    tui_gen::print_color(&header_string, "DARKBLUE");
}
pub fn print_header() {
    let title = "DEFINITIVE BEER DATABASE";
    //i_o::print_title_blue(&title);
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
