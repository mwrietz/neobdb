// bdb - definitive beer database
// 20220623

use rusqlite::{Connection, ToSql};
use std::fs;
use std::path::Path;

use crate::beer_struct::Beer;
use crate::tui_gen;
use crate::tui_inp;
use crate::ui;
use crate::ui::View;

pub fn query_full() -> String {
    //let query = format!("SELECT * FROM Beer ORDER BY brewer, name");
    "SELECT * FROM Beer ORDER BY brewer, name".to_string()
}

pub fn query_filtered(view: &View) -> String {
    // let query = match view.filter.len() {
    //     0 => query_full(),
    //     _ => format!(
    //         "SELECT * FROM Beer
    //             WHERE name LIKE '%{}%'
    //             OR brewer LIKE '%{}%'
    //             OR style LIKE '%{}%'
    //             OR abv LIKE '%{}%'
    //             OR rating LIKE '%{}%'
    //             OR notes LIKE '%{}%'
    //             ORDER BY brewer, name",
    //         view.filter, view.filter, view.filter, view.filter, view.filter, view.filter,
    //     ),
    // };
    // query
    match view.filter.len() {
        0 => query_full(),
        _ => format!(
            "SELECT * FROM Beer
                WHERE name LIKE '%{}%' 
                OR brewer LIKE '%{}%' 
                OR style LIKE '%{}%' 
                OR abv LIKE '%{}%' 
                OR rating LIKE '%{}%' 
                OR notes LIKE '%{}%' 
                ORDER BY brewer, name",
            view.filter, view.filter, view.filter, view.filter, view.filter, view.filter,
        ),
    }
}

pub fn query_for_display(view: &View) -> String {
    // let query = match view.filter.len() {
    //     0 => {
    //         format!(
    //             "SELECT * FROM Beer ORDER BY brewer, name LIMIT {} OFFSET {}",
    //             view.limit(),
    //             view.offset
    //         )
    //     }
    //     _ => {
    //         format!(
    //             "SELECT * FROM Beer WHERE name LIKE '%{}%'
    //                 OR brewer LIKE '%{}%'
    //                 OR style LIKE '%{}%'
    //                 OR abv LIKE '%{}%'
    //                 OR rating LIKE '%{}%'
    //                 OR notes LIKE '%{}%'
    //                 ORDER BY brewer, name
    //                 LIMIT {}
    //                 OFFSET {}",
    //             view.filter,
    //             view.filter,
    //             view.filter,
    //             view.filter,
    //             view.filter,
    //             view.filter,
    //             view.limit(),
    //             view.offset
    //         )
    //     }
    // };
    // query
    match view.filter.len() {
        0 => {
            format!(
                "SELECT * FROM Beer ORDER BY brewer, name LIMIT {} OFFSET {}",
                view.limit(),
                view.offset
            )
        }
        _ => {
            format!(
                "SELECT * FROM Beer WHERE name LIKE '%{}%' 
                    OR brewer LIKE '%{}%' 
                    OR style LIKE '%{}%' 
                    OR abv LIKE '%{}%' 
                    OR rating LIKE '%{}%' 
                    OR notes LIKE '%{}%' 
                    ORDER BY brewer, name
                    LIMIT {}
                    OFFSET {}",
                view.filter,
                view.filter,
                view.filter,
                view.filter,
                view.filter,
                view.filter,
                view.limit(),
                view.offset
            )
        }
    }
}

pub fn add(conn: &Connection) {
    tui_gen::cls();
    ui::print_header();

    println!("\nAdd new record...\n");

    // setup Beer struct
    let beer = Beer {
        id: uuid::Uuid::new_v4().to_string(),
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

    //stmt.execute(&[
    stmt.execute([
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

pub fn remove(conn: &Connection, view: &View) {
    let index = tui_inp::dialog_box_get_string(42, 4, "Remove", "Enter index of item to remove: ")
        .parse::<usize>()
        .unwrap();
    let query = query_for_display(view);
    let beers = vec_from_query(conn, query.as_str());
    let prompt = format!(
        "Are you sure you want to remove index {}: \"{}\" - (y/n)? ",
        index,
        beers[index - view.offset].name
    );
    let width = prompt.len() + 7;
    let action = tui_inp::dialog_box_get_string(width, 4, "Verify", &prompt);
    if action.eq("y") {
        let query = format!(
            "DELETE FROM Beer WHERE id = '{}'",
            beers[index - view.offset].id
        );
        conn.execute(query.as_str(), [])
            .expect("remove() execute error");
    }
}

pub fn edit(conn: &Connection, view: &View) {
    let index = tui_inp::dialog_box_get_string(42, 4, "Edit", "Enter index of item to edit: ")
        .parse::<usize>()
        .unwrap();
    let query = query_for_display(view);
    let beers = vec_from_query(conn, query.as_str());
    let prompt = format!(
        "Are you sure you want to edit index {}: \"{}\" - (y/n)? ",
        index,
        beers[index - view.offset].name
    );
    let width = prompt.len() + 7;
    let action = tui_inp::dialog_box_get_string(width, 4, "Verify", &prompt);
    if action.eq("y") {
        tui_gen::cls();
        ui::print_header();

        println!();
        beers[index - view.offset].print_details(index);
        println!();

        let b = Beer {
            id: beers[index - view.offset].id.clone(),
            timestamp: tui_gen::timestamp(),
            name: tui_inp::get_string_default("Enter new name", &beers[index - view.offset].name),
            brewer: tui_inp::get_string_default(
                "Enter new brewer",
                &beers[index - view.offset].brewer,
            ),
            style: tui_inp::get_string_default(
                "Enter new style",
                &beers[index - view.offset].style,
            ),
            abv: tui_inp::get_string_default("Enter new abv", &beers[index - view.offset].abv),
            rating: tui_inp::get_string_default(
                "Enter new rating",
                &beers[index - view.offset].rating,
            ),
            notes: tui_inp::get_string_default(
                "Enter new notes",
                &beers[index - view.offset].notes,
            ),
        };

        // update record
        let query = "UPDATE Beer SET timestamp = ?, name = ?, brewer = ?, style = ?, abv = ?, rating = ?, notes = ? WHERE id = ?";

        let mut stmt = conn.prepare(query).expect("add stmt error");

        //stmt.execute(&[
        stmt.execute([
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

        println!();
        println!("Updated record...");
        b.print_details(index);
    }
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
        id: uuid::Uuid::new_v4().to_string(),
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

    //stmt.execute(&[
    stmt.execute([
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
