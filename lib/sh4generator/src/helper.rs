use genco::fmt;
use genco::prelude::*;
use scraper::{ElementRef, Selector};
use std::io::Error;
use std::{fs::File, path::PathBuf};

pub fn get_nth_cell_of_table(element: &ElementRef, cell_idx: usize) -> String {
    let selector = Selector::parse(format!("div:nth-child({})", cell_idx).as_str()).unwrap();

    element.select(&selector).next().unwrap().text().collect()
}

pub fn cleanup_register_names(inp: &str) -> String {
    // TODO: Replace :d with rust specific format lines
    inp.to_string()
}

pub fn save_and_format(path_to_rs: PathBuf, tokens: rust::Tokens) -> Result<(), Error> {
    let output = File::create(path_to_rs)?;

    let mut w = fmt::IoWriter::new(output);

    let fmt = fmt::Config::from_lang::<Rust>().with_indentation(fmt::Indentation::Space(4));
    let config = rust::Config::default();

    let format = rust::Format::default();

    tokens
        .format(&mut w.as_formatter(&fmt), &config, &format)
        .unwrap();

    Ok(())
}
