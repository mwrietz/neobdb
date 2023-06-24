// neobdb - definitive beer database
// 20230624

use rusqlite::Connection;
use std::env;
use std::path::Path;
use std::process;

mod beer_struct;
mod config;
mod db;
mod pdf;
mod tui_frm;
mod tui_gen;
mod tui_inp;
mod tui_menu;
mod ui;

use crate::config::Config;

fn main() {

    tui_gen::splash_screen(
        "D E F I N I T I V E  B E E R  D A T A B A S E",
        format!("v{}", env!("CARGO_PKG_VERSION")).as_str(),
    );

    let config: Config = config::read_config_file();
    let db_path = config.data_dir + "/" + config.db_filename.as_str();
    let db_path = Path::new(db_path.as_str());

    if !db_path.exists() {
        db::create_database_if_not_exist(&db_path);
    }

    let conn = Connection::open(db_path).expect("cannot connecte to db");

    menu(&conn);
}

fn menu(conn: &Connection) {
    let mut v = ui::View {
        state: ui::State::Summary,
        height: tui_gen::t_height(),
        offset: 0,
        filter_count: db::count_rows_in_table(conn, "Beer"),
        filter: String::from("*"),
    };
    tui_gen::cls();
    ui::print_header();
    v.clone().show(&conn);

    let menu_items = vec![
        ("j", "Scroll_DN"),
        ("k", "Scroll_UP"),
        ("v", "Detail/Summary"),
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
                if (v.offset + v.limit()) < v.filter_count {
                    v.offset += v.limit();
                }
                v.clone().show(&conn);
            }
            'k' => {
                if v.offset >= v.limit() {
                    v.offset -= v.limit();
                }
                v.clone().show(&conn);
            }
            'v' => {
                match v.state {
                    ui::State::Summary => v.state = ui::State::Detail,
                    ui::State::Detail => v.state = ui::State::Summary,
                }
                v.offset = 0;
                v.clone().show(&conn);
            }
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
            'p' => pdf::create_pdf(&conn),
            'f' => {
                v.filter = tui_inp::dialog_box_get_string(50, 4, "Find", "Enter search string: ");
                v.find(&conn);
                v.clone().show(&conn);
            }
            'q' => {
                tui_gen::cls();
                process::exit(1);
            }
            _ => ui::usage(),
        }
    }
}
