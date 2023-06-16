// bdb2 - definitive beer database
// 20230611

use rusqlite::Connection;
use std::env;
use std::process;
use std::path::Path;

mod beer_struct;
mod config;
mod pdf;
mod db;
mod tui_frm;
mod tui_gen;
mod tui_inp;
mod tui_menu;
mod ui;

use crate::config::Config;
//use ui::State;

fn main() {
    let args: Vec<String> = env::args().collect();

    tui_gen::splash_screen(
        "D E F I N I T I V E  B E E R  D A T A B A S E",
        env!("CARGO_PKG_VERSION"),
    );

    // read configuration file and assemble db filepath
    let config: Config = config::read_config_file();
    let db_path = config.data_dir + "/" + config.db_filename.as_str();
    let db_path = Path::new(db_path.as_str());

    // check if db file exists
    if !db_path.exists() {
        db::create_database_if_not_exist(&db_path);
    }

    // connect to database
    let conn = Connection::open(db_path)
        .expect("cannot connecte to db");

    let mut v = ui::View {
        state: ui::State::Summary,
        height: tui_gen::t_height(),
        offset: 0,
        filter_count: db::count_rows_in_table(&conn, "Beer"),
        filter: String::from("*"),
    };

    // check for command line arguments
    if args.len() < 2 {
        tui_gen::cls();
        ui::print_header();
        //ui::show_summary(&conn);
        v.summary(&conn);

        menu(&conn);
    } 
    else {
        let cmd = &args[1];
        match &cmd[..] {
            "-a" | "--add" => db::add(&conn),
            "-r" | "--remove" => db::remove(&conn),
            "-e" | "--edit" => db::edit(&conn),
            //"-f" | "--find" => ui::show_found(&conn, &args[2]),
            "-f" | "--find" => {
                v.filter = args[2].clone();
                v.find(&conn);
            }
            //"-sa" | "--show_all" => ui::show_all(&conn),
            "-sa" | "--show_all" => v.detail(&conn),
            //"-ss" | "--show_sum" => ui::show_summary(&conn),
            "-ss" | "--show_sum" => v.summary(&conn),
            "-p" | "--pdf" => pdf::create_pdf(&conn),
            "-m" | "--menu" => {
                tui_gen::cls();
                ui::print_header();
                //ui::show_summary(&conn);
                v.summary(&conn);
                menu(&conn);
            }
            _ => {
                ui::usage();
                quit();
            }
        }
    }
}

fn menu(conn: &Connection) {
    let mut v = ui::View {
        state: ui::State::Summary,
        height: tui_gen::t_height(),
        offset: 0,
        filter_count: db::count_rows_in_table(conn, "Beer"),
        filter: String::from("*"),
    };
    let menu_items = vec![
        ("j", "Scroll_DN"),
        ("k", "Scroll_UP"),
        ("v", "Detail/Summary"),
        //("d", "Details"),
        //("s", "Summary"),
        ("a", "Add"),
        ("r", "Remove"),
        ("e", "Edit"),
        ("f", "Find"),
        ("p", "Pdf"),
        ("q", "Quit"),
    ];

    loop {
        let selection = tui_menu::menu_horiz(&menu_items);
        match selection {
            'j' => {
                // scroll_dn if not last page

                // if filter == '*'
                //let n_rows = db::count_rows_in_table(conn, "Beer");
                //if (v.offset + v.limit()) < n_rows {
                if (v.offset + v.limit()) < v.filter_count {
                    v.offset += v.limit();
                }
                // if search_sting == 'else'
                // n_rows = query_len()
                v.clone().show(&conn);
            },
            'k' => {
                // scroll_up
                //let n_rows = db::count_rows_in_table(conn, "Beer");
                if v.offset >= v.limit() {
                    v.offset -= v.limit();
                }
                v.clone().show(&conn);
            },
            'v' => {
                if v.state == ui::State::Summary {
                    v.state = ui::State::Detail;
                    v.offset = 0;
                } else {
                    v.state = ui::State::Summary;
                    v.offset = 0;
                }
                v.clone().show(&conn);
            },
            'a' => {
                db::add(&conn);
                v.clone().show(&conn);
            }
            'r' => {
                db::remove(&conn);
                v.clone().show(&conn);
            }
            'e' => {
                db::edit(&conn);
                v.clone().show(&conn);
            }
            //'d' => ui::show_all(conn),
            //'s' => ui::show_summary(conn),
            'p' => pdf::create_pdf(&conn),
            'f' => {
                v.filter = tui_inp::dialog_box_get_string(50, 4, "Find", "Enter search string: ");
                //ui::show_found(conn, search_string.as_str());
                v.find(&conn);
                v.clone().show(&conn);
            }
            'q' => {
                tui_gen::cls();
                quit();
            }
            _ => ui::usage(),
        }
    }
}

fn quit() {
    process::exit(1);
}
