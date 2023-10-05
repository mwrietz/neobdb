// neobdb - definitive beer database
// 20230624

use rusqlite::Connection;
use std::env;
use std::path::Path;
use std::process;
//use crossterm::{execute, Result, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};
//use std::io::stdout;

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

//fn main() -> Result<()> {
fn main() {
    //execute!(stdout(), EnterAlternateScreen)?;

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

    let conn = Connection::open(db_path).expect("cannot connect to db");

    menu(&conn);

    //execute!(stdout(), LeaveAlternateScreen)
}

fn menu(conn: &Connection) {
    let (_w, h) = tui_gen::tsize();
    let mut view = ui::View {
        state: ui::State::Summary,
        //height: tui_gen::t_height(),
        height: h,
        offset: 0,
        filter: String::from(""),
    };
    tui_gen::cls();
    ui::print_header();
    view.clone().show(&conn);

    let full_menu_items = vec![
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

    let filter_menu_items = vec![
        ("j", "Scroll_DN"),
        ("k", "Scroll_UP"),
        ("v", "Detail/Summary"),
        ("r", "Remove"),
        ("e", "Edit"),
        ("c", "Clear search"),
    ];

    loop {
        let selection = match view.filter.len() {
            0 => tui_menu::menu_horiz_neo(&full_menu_items),
            _ => tui_menu::menu_horiz_neo(&filter_menu_items),
        };

        match selection {
            'a' => {
                db::add(&conn);
            }
            'c' => {
                view.filter = "".to_string();
                view.offset = 0;
            }
            'e' => {
                db::edit(&conn, &view);
            }
            'f' => {
                view.filter =
                    tui_inp::dialog_box_get_string(50, 4, "Find", "Enter search string: ");
                view.offset = 0;
            }
            'j' => {
                if (view.offset + view.limit()) < view.clone().filter_count(conn) {
                    view.offset += view.limit();
                }
            }
            'k' => {
                if view.offset >= view.limit() {
                    view.offset -= view.limit();
                }
            }
            'p' => {
                pdf::create_pdf(&conn);
                tui_gen::pause();
            }
            'q' => {
                tui_gen::cls();
                //process::exit(1);
                break;
            }
            'r' => {
                db::remove(&conn, &view);
            }
            'v' => {
                match view.state {
                    ui::State::Summary => view.state = ui::State::Detail,
                    ui::State::Detail => view.state = ui::State::Summary,
                }
                view.offset = 0;
            }
            _ => break, //process::exit(1),
        }
        view.clone().show(&conn);
    }
}
