// pdf.rs

use std::fs::File;
use std::io::Write;
use std::path::Path;

use rusqlite::Connection;

use crate::db;
use crate::process;
use crate::tui_gen;
use crate::ui;
use crate::beer_struct::Beer;
use crate::config::{self, Config};

pub fn create_pdf(conn: &Connection) {
    let config: Config = config::read_config_file();
    let parent_path = Path::new(&config.data_dir);

    let md_path = parent_path.join("definitive_beer_database.md");
    let pdf_path = parent_path.join("definitive_beer_database.pdf");

    tui_gen::cls();
    ui::print_header();

    let query = db::query_full();
    let beers = db::vec_from_query(conn, query.as_str());

    create_md_file(&md_path, beers);
    md_to_pdf(&md_path, &pdf_path);
    println!();
}

fn create_md_file(md_path: &Path, beers: Vec<Beer>) {
    println!("creating {}...", md_path.display());
    let mut output = File::create(md_path).expect("error opening file");
    output.write_all(b"---\n").expect("write error");
    output
        .write_all(b"geometry: margin=2cm\n")
        .expect("write error");
    output.write_all(b"---\n").expect("write error");
    output
        .write_all(b"Table: **Definitive Beer Database**\n\n")
        .expect("write error");
    output
        .write_all(b"| name | brewer | style | abv | rating | notes |\n")
        .expect("write error");
    output
        .write_all(b"| :------- | :------- | :--- | :--- | :---: | :-------- |\n")
        .expect("write error");
    output
        .write_all(b"|  |  |  |  |  |  |\n")
        .expect("write error");
    for beer in beers {
        let buffer = format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            beer.name, beer.brewer, beer.style, beer.abv, beer.rating, beer.notes
        );
        output.write_all(buffer.as_bytes()).expect("write error");
        output
            .write_all(b"|  |  |  |  |  |  |\n")
            .expect("write error");
    }
}

fn md_to_pdf(md_path: &Path, pdf_path: &Path) {
    println!("creating {}...", pdf_path.display());
    process::Command::new("pandoc")
        .arg(md_path)
        .arg("-o")
        .arg(pdf_path)
        .spawn()
        .expect("pandoc pdf failed");
    println!("complete!");
}
