use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::{Dialog, DummyView, LinearLayout, SelectView, TextView};
use cursive::Cursive;

use anyhow::{Context, Result};
use std::env::set_current_dir;
use std::fs::read_dir;
use std::fs::read_to_string;
use std::fs::Metadata;
use std::path::Path;
use std::path::PathBuf;

fn get_current_dir() -> Vec<std::fs::DirEntry> {
    let mut current_dir_entries: Vec<std::fs::DirEntry> = Vec::new();

    let entries = match read_dir(".") {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("failed to read current dir: {:?}", e);
            return current_dir_entries;
        }
    };
    for entry_result in entries {
        let entry = match entry_result {
            Ok(entry) => current_dir_entries.push(entry),
            Err(e) => {
                eprintln!("failed to read dir entry: {:?}", e);
                continue;
            }
        };
    }
    current_dir_entries
}

fn get_file_name(index: usize) -> Result<String> {
    let current_dir_entries = get_current_dir();
    let entry = current_dir_entries
        .get(index)
        .ok_or(anyhow::anyhow!("index out of bounds"))?;
    let file_name = entry.file_name();
    let file_name_string = file_name
        .into_string()
        .map_err(|_| anyhow::anyhow!("filename is not valid utf8"))?;
    Ok(file_name_string)
}

fn display_directory(session: &mut Cursive) {
    session.pop_layer();

    let mut display = SelectView::new().h_align(HAlign::Center);

    for (i, partition) in get_current_dir().iter().enumerate() {
        let file_name = match get_file_name(i) {
            Ok(file_name) => display.add_item(file_name, i),
            Err(e) => {
                eprintln!("failed to get file name at index {}, error: {:?}", i, e);
                continue;
            }
        };
    }
    display.set_on_submit(file_or_dir);

    session.add_layer(
        Dialog::around(display)
            .title("file explorer")
            .button("..", parent_dir)
            .button("quit", |session| session.quit())
            .padding(cursive::view::Margins::lrtb(6, 6, 6, 6)),
    );
}

fn file_or_dir(session: &mut Cursive, index_from_selection: &usize) -> Result<()> {
    //I need the entry, I get the index so i can make due
    let content = get_current_dir();
    let entry_in_question = content
        .get(*index_from_selection)
        .context("unable to locate entry")?;
    let entry_metadata = entry_in_question
        .metadata()
        .context("failed to retrieve metadata")?;

    if entry_metadata.is_dir() {
        set_current_dir(entry_in_question.path()).context(format!(
            "Failed to set current directory to {:?}",
            entry_in_question.path()
        ))?;
        display_directory(session);
        Ok(())
    } else if entry_metadata.is_file() {
        display_file(session, entry_in_question);
        Ok(())
    } else {
        return Err(anyhow::anyhow!("Entry not a file or directory"));
    }

}

fn display_file(session: &mut Cursive, entry: &std::fs::DirEntry) -> Result<()> {
    let file_contents = read_to_string(entry.path()).expect("file was not readable");
    session.pop_layer();
    session.add_layer(Dialog::text(file_contents).button("exit", display_directory));
    Ok(())
}

fn parent_dir(session: &mut Cursive) {
    let parent_dir = Path::new("..");
    set_current_dir(parent_dir);
    display_directory(session);
}

fn main() {
    let mut session = cursive::default();
    session.add_layer(
        Dialog::text("Would you like to enter your files?")
            .title("Spell Casting")
            .button("Yes", display_directory)
            .button("No", |session| session.quit())
            .padding(cursive::view::Margins::lrtb(6, 6, 6, 6)),
    );
    session.run();
    //    testing();
}
