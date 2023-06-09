use cursive::align::HAlign;
use std::ffi::OsString;
use cursive::views::{Dialog, SelectView, TextView};
use cursive::Cursive;
use cursive::views::LinearLayout;
use cursive::view::Scrollable;
use cursive::direction::Orientation;
use cursive::view::Nameable;
use cursive::view::IntoBoxedView;


use anyhow::{Context, Result};
use std::env::set_current_dir;
use std::fs::read_dir;
use std::fs::read_to_string;
use std::path::Path;

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
        match entry_result {
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

    for (i, _partition) in get_current_dir().iter().enumerate() {
        match get_file_name(i) {
            Ok(file_name) => display.add_item(file_name, i),
            Err(e) => {
                eprintln!("failed to get file name at index {}, error: {:?}", i, e);
                continue;
            }
        };
    }
    display.set_on_select(file_preview);
    display.set_on_submit(file_or_dir);

//    session.add_layer(
//        Dialog::around(display)
//            .title("file explorer")
//            .button("..", parent_dir)
//            .button("quit", |session| session.quit())
//            .padding(cursive::view::Margins::lrtb(6, 6, 6, 6)),
//    );
    let display_trim = Dialog::around(display.scrollable())
        .title("File Explorer")
        .button("..", parent_dir)
        .button("Quit", |session| session.quit())
        .padding(cursive::view::Margins::lrtb(6, 6, 6, 6));

    let mut layout = LinearLayout::new(Orientation::Horizontal)
        .with_name("layout");
    layout.get_mut().add_child(display_trim);
    session.add_fullscreen_layer(layout);
}

fn file_preview(session: &mut Cursive, index_from_selection: &usize) {
    let content = get_current_dir();
    let entry_in_question = match content.get(*index_from_selection) {
        Some(entry) => entry,
        None => {
            eprintln!("unable to get entry from entry vector");
            return;
        }
    };

    let entry_metadata = match entry_in_question.metadata() {
        Ok(metadata) => metadata,
        Err(e) => {
            eprintln!("unable to retrieve metadata from entry: {}", e);
            return;
        }
    };

    if entry_metadata.is_file() {
        let content = get_current_dir();
        let entry = match content.get(*index_from_selection) {
            Some(entry) => entry,
            None => {
                eprintln!("unable to get entry from entry vector");
                return;
            }
        };
        let file_contents = match read_to_string(entry.path()) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("failed to read file contents to string {}", e);
                let failed_output = String::from("file failed to be displayed");
                failed_output
            }
        };
        let view = TextView::new(file_contents);
        session.call_on_name("layout",  |layout: &mut LinearLayout| {
            layout.remove_child(1);
            layout.add_child(view);
        });
    }
}

fn file_or_dir(session: &mut Cursive, index_from_submition: &usize) -> Result<()> {
    //I need the entry, I get the index so i can make due
    let content = get_current_dir();
    let entry_in_question = content
        .get(*index_from_submition)
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
        display_file(session, entry_in_question)?;
        Ok(())
    } else {
        return Err(anyhow::anyhow!("Entry not a file or directory"));
    }
}

fn display_file(session: &mut Cursive, entry: &std::fs::DirEntry) -> Result<()> {
    let file_contents = match read_to_string(entry.path()) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("failed to read file contents to string {}", e);
            let failed_output = String::from("file failed to be displayed");
            failed_output
        }
    };
    let file_path = entry.path();
    session.pop_layer();
    session.add_layer(
        Dialog::text(file_contents)
            .button("edit", move |session| edit_file(session, file_path.to_owned()))
            .button("exit", display_directory),
    );
    Ok(())
}

fn edit_file(session: &mut Cursive, file_path: std::path::PathBuf) {
    std::process::Command::new("vim")
        .arg(file_path.to_string_lossy().as_ref())
        .spawn()
        .expect("Error: failed to run editor")
        .wait()
        .expect("Error: Editor returned non-zero exit code");
    session.quit();
}

fn parent_dir(session: &mut Cursive) {
    let parent_dir = Path::new("..");
    match set_current_dir(parent_dir) {
        Ok(parent_dir) => parent_dir,
        Err(e) => {
            eprintln!("failed to set parent dir {}", e);
        }
    };
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
