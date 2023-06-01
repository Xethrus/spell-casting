use cursive::traits::*;
use cursive::views::{Dialog, DummyView, LinearLayout, SelectView, TextView};
use cursive::align::HAlign;
use cursive::Cursive;

use std::fs::read_dir;
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

fn get_file_path(index: usize) -> Result<PathBuf> {
    let dir_entries = get_current_dir()?;
    let entry = dir_entries.get(index).ok_or(anyhow::anyhow!("index out of bounds"))?;
    let file_path = entry.path();
    Ok(file_path)
}

fn is_file_or_dir(index: usize) -> Result<Metadata> {
    let current_dir_entries = get_current_dir()?;
    let entry = current_dir_entries.get(index).ok_or(anyhow::anyhow!("index out of bounds"))?;
    let entry_metadata = entry.metadata();
    if entry_metadata
}

//fn entry_exists(index: usize) -> Result<bool> {
//    if get_current_dir()? {
//        return true;
//    } else {
//        return false;
//    }
//}


fn display_current_dir() -> Result<()> {
    
    let mut siv = cursive::default();
    let mut file_display = SelectView::new().h_align(HAlign::Center);

    let items = get_current_dir()?;

    for (i , item) in items.iter().enumerate() {
        let file_name = get_file_name(i)?; // Assuming this function returns a Result<String, Error>
       file_display.add_item(file_name, i);
    }

    file_display.set_on_submit(|s, file| {
        s.pop_layer();
        let text = format!("you have selected {}", file);
        s.add_layer(
            Dialog::around(TextView::new(text)).button("Quit", |s| s.quit()),
        );
    });

    siv.add_layer(Dialog::around(file_display).title("file display"));
    siv.run();
    Ok(())
}

fn testing() -> () {
    let mut siv = cursive::default();
    let mut time_select = SelectView::new().h_align(HAlign::Center);
    time_select.add_item("Short", 1);
    time_select.add_item("Medium", 5);
    time_select.add_item("Long", 10);

    time_select.set_on_submit(|s, time| {
        s.pop_layer();
        let text = format!("You will wait for {} minutes...", time);
        s.add_layer(
            Dialog::around(TextView::new(text)).button("Quit", |s| s.quit()),
        );
    });

    siv.add_layer(Dialog::around(time_select).title("How long is your wait?"));
    siv.run();
}

fn main() {
    display_current_dir();
//    testing();
}
