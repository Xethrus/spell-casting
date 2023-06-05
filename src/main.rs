use cursive::traits::*;
use cursive::views::{Dialog, DummyView, LinearLayout, SelectView, TextView};
use cursive::align::HAlign;
use cursive::Cursive;

use std::env::set_current_dir;
use std::fs::read_dir;
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

fn get_file_path(index: usize) -> Result<PathBuf> {
    let dir_entries = get_current_dir()?;
    let entry = dir_entries.get(index).ok_or(anyhow::anyhow!("index out of bounds"))?;
    let file_path = entry.path();
    Ok(file_path)
}
//fn is_dir_then_enter(index: usize) -> Result<Metadata> {
//    let dir_entries = get_current_dir()?;
//    let entry = dir_entries.get(index).ok_or(anyhow::anyhow!("index out of bounds"))?;
//    let entry_metadata = entry.metadata();
//    if entry_metadata?.is_dir() {
//        match set_current_dir(entry.path()) {
//            Ok => {
//                println!("entred directory");
//            }
//            _ => {
//                println!("operation failed");
//            }
//        }
//    } else {
//        !
//    }
//    Ok(entry_metadata?)
//}


//fn entry_exists(index: usize) -> Result<bool> {
//    if get_current_dir()? {
//        return true;
//    } else {
//        return false;
//    }
//}


//fn display_current_dir(siv: &mut cursive::Cursive) -> Result<()> {
//    
//   // let mut siv = cursive::default();
//    let mut file_display = SelectView::new().h_align(HAlign::Center);
//
//    let items = get_current_dir()?;
//
//    for (i , item) in items.iter().enumerate() {
//        let file_name = get_file_name(i)?; // Assuming this function returns a Result<String, Error>
//       file_display.add_item(file_name, i);
//    }
//
//    file_display.set_on_submit(|s, file| {
//        s.pop_layer();
//        let text = format!("you have selected {}", file);
//        is_dir_then_enter(*file);
//        s.add_layer(
//            Dialog::around(TextView::new(text)).button("Quit", |s| s.quit()),
//        );
//    });
//
//    siv.add_layer(Dialog::around(file_display).title("file display"));
//    Ok(())
//}
//

fn display_directory(session: &mut Cursive) {
    session.pop_layer();
    
    let mut display = SelectView::new().h_align(HAlign::Center);
    let content = get_current_dir().unwrap();  //need to make this sweeter... syntactically 

    for(i , partition) in content.iter().enumerate() {
        let file_name = get_file_name(i).unwrap(); //need to make this sweeter... syntactically 
        display.add_item(file_name, i);
    }
    display.set_on_select(testing); //Goal: to get this to behave like a .button() where I could
                                   //pass the name of the function and it calls it with the &mut
                                   //cursive object automatically

    session.add_layer(Dialog::around(display).title("file explorer"));
}
fn testing(session: &mut Cursive, _needed_param: &usize) {
    session.pop_layer();
    session.add_layer(Dialog::text("did this change!?"));
}


fn main() {
    let mut session = cursive::default();
    session.add_layer(Dialog::text("Would you like to enter your files?")
        .button("Yes", display_directory)
    );
    session.run();
//    testing();
}
