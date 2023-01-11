use std::{sync::Arc, rc::Rc, cell::RefCell};

use slint::SharedString;

use crate::files;

slint::include_modules!();

pub fn run() {
    let app = PrimaryWindow::new();

    let data = Rc::new(RefCell::new(PrimaryData {
        file_data: files::HypAbFile::new(),
    }));

    // Clear working data and unlink working file
    let data_ref = data.clone();
    let app_ref = app.as_weak();
    app.global::<PersistantLogic>().on_new_file(move || {
        let app = app_ref.unwrap(); //fix
        data_ref.replace_with(|_| PrimaryData{file_data: files::HypAbFile::new()});
        app.set_window_title(SharedString::from(data_ref.borrow().file_data.title()));
        app.set_current_view(0);
        app.set_project_title(SharedString::from(&data_ref.borrow().file_data.map_data.name));
    });

    //Open a file specified in dialog and obtain working data from that file
    let data_ref = data.clone();
    let app_ref = app.as_weak();
    app.global::<PersistantLogic>().on_open_file(move || {
        let app = app_ref.unwrap(); //fix
        data_ref.replace_with(|x| PrimaryData {
            file_data: files::HypAbFile::query_file().unwrap_or(x.file_data.clone())
        });
        app.set_window_title(SharedString::from(data_ref.borrow().file_data.title()));
        app.set_current_view(0);
        app.set_project_title(SharedString::from(&data_ref.borrow().file_data.map_data.name));
    });

    //Save working data to working file (if file dne move to save as functionality)
    let data_ref = data.clone();
    let app_ref = app.as_weak();
    app.global::<PersistantLogic>().on_save_file(move || {
        let app = app_ref.unwrap(); //fix
        data_ref.borrow_mut().file_data.save_file().unwrap_or_else(|e| eprintln!("{:?}", e));
        app.set_window_title(SharedString::from(data_ref.borrow().file_data.title()));
    });

    //Save working data to specified file in dialog
    let data_ref = data.clone();
    let app_ref = app.as_weak();
    app.global::<PersistantLogic>().on_save_as_file(move || {
        let app = app_ref.unwrap(); //fix
        data_ref.borrow_mut().file_data.save_file_as().unwrap_or_else(|e| eprintln!("{:?}", e));
        app.set_window_title(SharedString::from(data_ref.borrow().file_data.title()));
    });

    //Update working data's title
    let data_ref = data.clone();
    let app_ref = app.as_weak();
    app.global::<PersistantLogic>().on_update_title(move |new_title| {
        let app = app_ref.unwrap(); //fix
        data_ref.borrow_mut().file_data.update_title(new_title.to_string());
        app.set_window_title(SharedString::from(data_ref.borrow().file_data.title()));
        app.set_project_title(SharedString::from(&data_ref.borrow().file_data.map_data.name));
    });

    /*
        Overall View
    */

    //create a new section with given name
    let data_ref = data.clone();
    app.global::<OverallLogic>().on_create_section(move |section_title| {
        data_ref.borrow_mut().file_data.map_data.section_insert(section_title.to_string());
        let new_data: Vec<(i32, SharedString)> = data_ref.borrow().file_data.map_data.get_sections()
            .iter().map(|(x, y)| (*x, SharedString::from(&y.name))).collect();
        let model = slint::VecModel::from(new_data);
        slint::ModelRc::new(model)
    });

    let data_ref = data.clone();
    app.global::<OverallLogic>().on_get_all_sections(move || {
        let new_data: Vec<(i32, SharedString)> = data_ref.borrow().file_data.map_data.get_sections()
            .iter().map(|(x, y)| (*x, SharedString::from(&y.name))).collect();
        let model = slint::VecModel::from(new_data);
        slint::ModelRc::new(model)
    });

    app.run();
}

#[derive(Clone, Debug)]
struct PrimaryData {
    file_data: files::HypAbFile,
}