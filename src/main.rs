use cursive::traits::*;
use cursive::views::{Dialog, DummyView, LinearLayout, SelectView, TextView};
use cursive::align::HAlign;
use cursive::Cursive;

use std::fs::read_dir;
use std::path::PathBuf;
use anyhow::{Result};

fn get_current_dir() -> Result<Vec<std::fs::DirEntry>> {
    let mut current_dir_contents: Vec<std::fs::DirEntry> = Vec::new();
    for entry_result in read_dir(".")? {
        let entry = entry_result?;
        current_dir_contents.push(entry);
    }
    Ok(current_dir_contents)
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
//fn entry_exists(index: usize) -> Result<bool> {
//    if get_current_dir()? {
//        return true;
//    } else {
//        return false;
//    }
//}
//
fn display_current_dir() -> Result<()> {
    let mut siv = cursive::default();
    let mut select_view = SelectView::new()
        .on_select(|s, item| {
            s.add_layer(Dialog::text(format!("You selected: {}", item)).button("Ok", |s| s.quit()));
        })
        .with_name("menu");

    let items = get_current_dir()?;
    for (i , item) in items.iter().enumerate() {
        let file_name = get_file_name(i)?; // Assuming this function returns a Result<String, Error>
        select_view.get_mut().add_item(file_name, i);
    }

    siv.add_layer(
        Dialog::around(select_view)
            .title("Files")
            .button("Exit", |s| s.quit())
            .fixed_width(30),
    );

    siv.run();
    Ok(())
}
//
//fn display_current_dir() -> Result<()> { //let mut current_dir: Vec<std::fs::DirEntry> = Vec::new();
//    let mut siv = cursive::default();
//    let mut select_view = SelectView::new()
//        .on_select(|s, item| {
//            s.add_layer(Dialog::text(format!("you selected: {}", item)).button("Ok", |s| s.quit()));
//        })
//        .with_name("menu");
//
//    
//    let items = get_current_dir()?;
//    for (i , item) in items.iter().enumerate() {
//        select_view.add_item(get_file_name(i)?, i);
//    }
//    
//
//    siv.add_layer(
//        Dialog::around(select_view).title("files")
//        .button("exit", |s| s.quit())
//        .fixed_width(30),
//    );
//        //.h_align(HAlign::Center),
//    siv.run();
//    Ok(())
//}
//

fn main() {
    //cursive root
//    let mut siv = cursive::default();
//    let text = "nothing right now"; 
    //dialog with quit
    display_current_dir();
//    siv.add_layer(
//        Dialog::around(
//            LinearLayout::vertical()
//                .child(TextView::new("view").h_align(HAlign::Center))
//                .child(DummyView.fixed_height(1))
//                .child(TextView::new(text))
//                .child(TextView::new(text).scrollable())
//                .child(TextView::new(text).scrollable())
//                .child(TextView::new(text).scrollable())
//                .fixed_width(30),
//        )
//        .button("Quit", |s| s.quit())
//        .h_align(HAlign::Center),
//    );
//    //starts the event loop
//    siv.run();
}
