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

fn main() {
    let args: Vec<String> = env::args().collect();
    tui_gen::splash_screen(
        "D E F I N I T I V E  B E E R  D A T A B A S E",
        env!("CARGO_PKG_VERSION"),
    );

    // read configuration file
    let config: Config = config::read_config_file();
    let db_path = config.data_dir + "/" + config.db_filename.as_str();

    // check if db file exists
    let p = Path::new(db_path.as_str());
    if !p.exists() {
        db::create_datafile_if_not_exist(&p);
    }

    // connect to database
    let conn = Connection::open(&db_path)
        .expect("cannot connecte to db");

    if args.len() < 2 {
        tui_gen::cls();
        ui::print_header();
        ui::show_summary(&conn);
        menu(&conn);
    } else {
        let cmd = &args[1];
        match &cmd[..] {
            "-a" | "--add" => db::add(&conn),
            "-r" | "--remove" => db::remove(&conn),
            "-e" | "--edit" => db::edit(&conn),
            "-f" | "--find" => ui::show_found(&conn, &args[2]),
            "-sa" | "--show_all" => ui::show_all(&conn),
            "-ss" | "--show_sum" => ui::show_summary(&conn),
            "-p" | "--pdf" => pdf::create_pdf(&conn),
            "-m" | "--menu" => {
                tui_gen::cls();
                ui::print_header();
                ui::show_summary(&conn);
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
    loop {
        let menu_items = vec![
            ("d", "Details"),
            ("s", "Summary"),
            ("a", "Add"),
            ("r", "Remove"),
            ("e", "Edit"),
            ("f", "Find"),
            ("p", "Pdf"),
            ("q", "Quit"),
        ];

        let selection = tui_menu::menu_horiz(&menu_items);

        match selection {
            'a' => db::add(conn),
            'r' => db::remove(conn),
            'e' => db::edit(conn),
            'd' => ui::show_all(conn),
            's' => ui::show_summary(conn),
            'p' => pdf::create_pdf(conn),
            'f' => {
                let search_string =
                    tui_inp::dialog_box_get_string(50, 4, "Find", "Enter search string: ");
                ui::show_found(conn, search_string.as_str());
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
