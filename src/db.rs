// bdb - definitive beer database
// 20220623

use rusqlite::{params, Connection, ToSql};
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::beer_struct::Beer;
use crate::tui_gen;
use crate::tui_inp;
use crate::ui;

pub fn add(conn: &Connection) {
    tui_gen::cls();
    ui::print_header();

    println!("\nAdd new record...\n");

    // setup Beer struct
    let beer = Beer {
        id: generate_uuid(),
        timestamp: tui_gen::timestamp(),
        name: tui_inp::get_string("Enter name: "),
        brewer: tui_inp::get_string("Enter brewer: "),
        style: tui_inp::get_string("Enter style: "),
        abv: tui_inp::get_string("Enter abv: "),
        rating: tui_inp::get_string("Enter rating: "),
        notes: tui_inp::get_string("Enter notes: "),
    };

    // write to database
    let query = "INSERT INTO Beer (id, timestamp, name, brewer, style, abv, rating, notes) VALUES (?, ?, ?, ?, ?, ?, ?, ?)";

    let mut stmt = conn.prepare(query).expect("add stmt error");

    stmt.execute(&[
        &beer.id as &dyn ToSql,
        &beer.timestamp as &dyn ToSql,
        &beer.name as &dyn ToSql,
        &beer.brewer as &dyn ToSql,
        &beer.style as &dyn ToSql,
        &beer.abv as &dyn ToSql,
        &beer.rating as &dyn ToSql,
        &beer.notes as &dyn ToSql,
    ])
    .expect("add execute error");
}

pub fn remove(conn: &Connection) {
    // verify if index within displayed page
    let index = tui_inp::dialog_box_get_string(42, 4, "Remove", "Enter index of item to remove: ")
        .parse::<usize>()
        .unwrap();

    // read all data from database into vector of beers
    // limit/offset
    let query = "SELECT * FROM Beer ORDER BY brewer, name";
    let beers = vec_from_query(conn, query);

    let b = beers.get(index).expect("error");

    let prompt = format!(
        "Are you sure you want to remove index {}: \"{}\" - (y/n)? ",
        index, b.name
    );
    let width = prompt.len() + 7;
    let action = tui_inp::dialog_box_get_string(width, 4, "Verify", &prompt);

    if action.eq("y") {
        let query = format!("DELETE FROM Beer WHERE id = '{}'", b.id);
        conn.execute(query.as_str(), [])
            .expect("remove() execute error");
    }
}

pub fn edit(conn: &Connection) {
    // make sure index on displayed page
    let index = tui_inp::dialog_box_get_string(42, 4, "Edit", "Enter index of item to edit: ")
        .parse::<usize>()
        .unwrap();

    let query = "SELECT * FROM Beer ORDER BY brewer, name";
    let beers = vec_from_query(conn, query);

    let prompt = format!(
        "Are you sure you want to edit index {}: \"{}\" - (y/n)? ",
        index, beers[index].name
    );
    let width = prompt.len() + 7;
    let action = tui_inp::dialog_box_get_string(width, 4, "Verify", &prompt);

    if action.eq("y") {
        tui_gen::cls();
        ui::print_header();

        println!("");
        beers[index].print_details(index);
        println!("");

        // prompt for revised data and setup new beer struct
        let b = Beer {
            id: beers[index].id.clone(),
            timestamp: tui_gen::timestamp(),
            name: tui_inp::get_string_default("Enter new name", &beers[index].name),
            brewer: tui_inp::get_string_default("Enter new brewer", &beers[index].brewer),
            style: tui_inp::get_string_default("Enter new style", &beers[index].style),
            abv: tui_inp::get_string_default("Enter new abv", &beers[index].abv),
            rating: tui_inp::get_string_default("Enter new rating", &beers[index].rating),
            notes: tui_inp::get_string_default("Enter new notes", &beers[index].notes),
        };

        // update record
        let query = "UPDATE Beer SET timestamp = ?, name = ?, brewer = ?, style = ?, abv = ?, rating = ?, notes = ? WHERE id = ?";

        let mut stmt = conn.prepare(query).expect("add stmt error");

        stmt.execute(&[
            &b.timestamp as &dyn ToSql,
            &b.name as &dyn ToSql,
            &b.brewer as &dyn ToSql,
            &b.style as &dyn ToSql,
            &b.abv as &dyn ToSql,
            &b.rating as &dyn ToSql,
            &b.notes as &dyn ToSql,
            &b.id as &dyn ToSql,
        ])
        .expect("edit() execute error");

        println!("");
        println!("Updated record...");
        b.print_details(index);
    }
}

pub fn count_rows_in_table(conn: &Connection, table_name: &str) -> usize {
    let query = format!("SELECT COUNT(*) FROM {}", table_name);
    let count: i64 = conn
        .query_row(query.as_str(), params![], |row| row.get(0))
        .expect("count_rows_in_table() error");
    count as usize
}

pub fn count_rows_in_query(conn: &Connection, query: &str) -> usize {
    let count: i64 = conn
        .query_row(query, params![], |row| row.get(0))
        .expect("count_rows_in_query() error");
    count as usize
}

fn generate_uuid() -> String {
    let output = Command::new("uuidgen")
        .output()
        .expect("generate_uuid() error");
    let uuid = String::from_utf8_lossy(&output.stdout)
        .to_string()
        .trim_end_matches('\n')
        .to_string();
    uuid
}

pub fn vec_from_query(conn: &Connection, query: &str) -> Vec<Beer> {
    let mut stmt = conn.prepare(query).expect("vec_from_query() error 1");

    let beer_iter = stmt
        .query_map([], |row| {
            Ok(Beer {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                name: row.get(2)?,
                brewer: row.get(3)?,
                style: row.get(4)?,
                abv: row.get(5)?,
                rating: row.get(6)?,
                notes: row.get(7)?,
            })
        })
        .expect("vec_from_query() error 2");

    let mut beers: Vec<Beer> = Vec::new();
    for beer in beer_iter {
        beers.push(beer.unwrap());
    }
    beers
}

pub fn create_database_if_not_exist(db_path: &Path) {
    let db_parent_name = db_path.parent().unwrap();

    // create data folder if it doesn't exist
    fs::create_dir_all(db_parent_name).expect("cannot create backup folder");

    // create struct for first record in db
    let beer = Beer {
        id: generate_uuid(),
        timestamp: tui_gen::timestamp(),
        name: "Budweiser".to_string(),
        brewer: "Anheuser-Busch".to_string(),
        style: "Pilsner".to_string(),
        abv: "5.0%".to_string(),
        rating: "*".to_string(),
        notes: "yuck!".to_string(),
    };

    // create db file
    let conn = Connection::open(db_path).expect("cannot create db file");

    // create table: Beer
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Beer (
            id TEXT PRIMARY KEY,
            timestamp TEXT NOT NULL,
            name TEXT NOT NULL,
            brewer TEXT NOT NULL,
            style TEXT NOT NULL,
            abv TEXT NOT NULL,
            rating TEXT NOT NULL,
            notes TEXT NOT NULL
        )",
        [],
    )
    .expect("create table error");

    // write to database
    let query = "INSERT INTO Beer (id, timestamp, name, brewer, style, abv, rating, notes) VALUES (?, ?, ?, ?, ?, ?, ?, ?)";

    let mut stmt = conn.prepare(query).expect("add stmt error");

    stmt.execute(&[
        &beer.id as &dyn ToSql,
        &beer.timestamp as &dyn ToSql,
        &beer.name as &dyn ToSql,
        &beer.brewer as &dyn ToSql,
        &beer.style as &dyn ToSql,
        &beer.abv as &dyn ToSql,
        &beer.rating as &dyn ToSql,
        &beer.notes as &dyn ToSql,
    ])
    .expect("create_datafile execute error");

    println!("Data file not found...");
    println!("Data file created: {:?}", db_path);
    println!("Please start program again to use new data file.");
}
