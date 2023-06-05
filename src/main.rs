use cursive::traits::*;
use cursive::views::{Dialog, DummyView, LinearLayout, SelectView, TextView};
use cursive::align::HAlign;
use cursive::Cursive;

use std::env::set_current_dir;
use std::fs::read_dir;
use std::fs::read_to_string;
use std::fs::Metadata;
use std::path::PathBuf;
use anyhow::{Result};

fn get_current_dir() -> Result<Vec<std::fs::DirEntry>> {
    let mut current_dir_entries: Vec<std::fs::DirEntry> = Vec::new();
    for entry_result in read_dir(".")? {
        let entry = entry_result?;
        current_dir_entries.push(entry);
    }
    Ok(current_dir_entries)
}

fn get_file_name(index: usize) -> Result<String> {
    let current_dir_entries = get_current_dir()?;
    let entry = current_dir_entries.get(index).ok_or(anyhow::anyhow!("index out of bounds"))?;
    let file_name = entry.file_name();
    let file_name_string = file_name.into_string().map_err(|_| anyhow::anyhow!("filename is not valid utf8"))?;
    Ok(file_name_string)
}

fn display_directory(session: &mut Cursive) {
    session.pop_layer();
    
    let mut display = SelectView::new().h_align(HAlign::Center);
    let content = get_current_dir().unwrap();  //need to make this sweeter... syntactically 

    for(i , partition) in content.iter().enumerate() {
        let file_name = get_file_name(i).unwrap(); //need to make this sweeter... syntactically 
        display.add_item(file_name, i);
    }
    display.set_on_submit(file_or_dir); 

    session.add_layer(Dialog::around(display).title("file explorer")
        .button("quit", |session| session.quit())
    );
}

fn file_or_dir(session: &mut Cursive, index_from_selection: &usize) {
    //I need the entry, I get the index so i can make due 
    let content = get_current_dir().unwrap();
    let entry_in_question = &content[*index_from_selection];
    let entry_metadata = entry_in_question.metadata().unwrap();

    if entry_metadata.is_dir() {
        set_current_dir(entry_in_question.path());
        display_directory(session);
    } else if entry_metadata.is_file() {
        display_file(session, entry_in_question);
    }
}

fn display_file(session: &mut Cursive, entry: &std::fs::DirEntry) {
    let file_contents = read_to_string(entry.path())
        .expect("file was not readable");
    session.pop_layer();
    session.add_layer(Dialog::text(file_contents)
        .button("exit", display_directory)
    );
}

fn testing(session: &mut Cursive, _needed_param: &usize) {
    session.pop_layer();
    session.add_layer(Dialog::text(_needed_param.to_string())
        .button("quit", display_directory)
    );
}


fn main() {
    let mut session = cursive::default();
    session.add_layer(Dialog::text("Would you like to enter your files?")
        .button("Yes", display_directory)
        .button("No", |session| session.quit())
    );
    session.run();
//    testing();
}
