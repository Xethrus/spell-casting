use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::{Dialog, DummyView, LinearLayout, TextView};

use std::fs::read_dir;
use std::ffi::OsString;
use anyhow::{anyhow, Result};

fn get_current_dir() -> Result<Vec<std::fs::DirEntry>> {
    let mut current_dir_contents: Vec<std::fs::DirEntry> = Vec::new();
    for entry_result in read_dir(".")? {
        let entry = entry_result?;
        current_dir_contents.push(entry);
    }
    Ok(current_dir_contents)
}

fn get_file_name(index: usize, current_dir_result: Result<Vec<std::fs::DirEntry>>) -> OsString {
    match current_dir_result {
        Ok(entries) => {
            if let Some(entry) = entries.get(index) {
                let file_name = entry.file_name();
                return file_name;
            } else {
                //WTF do i do here D:  
            }
        }
        Err(e) => {
            println!("failed to read DirEntry: {:?}", e); 
        }
    }
}

fn display_current_dir() -> Result<()> {
    //let mut current_dir: Vec<std::fs::DirEntry> = Vec::new();
    let mut siv = cursive::default();
    let mut current_dir = get_current_dir();
    let mut current_index = 0;

    siv.add_layer(
        Dialog::around(
           LinearLayout::vertical()
            .child(TextView::new("view").h_align(HAlign::Center))
            .child(DummyView.fixed_height(1))
            .child(TextView::new(get_file_name(current_index, current_dir)))
            .fixed_width(30),
        )
        .button("exit", |s| s.quit())
        .h_align(HAlign::Center),
    );
    siv.run();
    Ok(())
}

//debug
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

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
