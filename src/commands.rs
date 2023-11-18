use crate::{
    helpers::{current_time, parse_data, reset_data, save_data},
    MyResultExt,
};
use console::style;
use dialoguer::Confirm;
use json::{self, object};
use prettytable::{color, format, Attr, Cell, Row, Table};

pub fn list() -> Result<(), String> {
    let data = parse_data();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);

    table.add_row(Row::new(vec![
        Cell::new("ID"),
        Cell::new("Text"),
        Cell::new("Done"),
        Cell::new("Created At"),
        Cell::new("Updated At"),
    ]));

    for todo in data["todos"].members() {
        table.add_row(Row::new(vec![
            Cell::new(todo["id"].as_number().unwrap().to_string().as_str())
                .with_style(Attr::ForegroundColor(color::BLUE)),
            Cell::new(todo["text"].as_str().unwrap()),
            Cell::new(if todo["done"].as_bool().unwrap() {
                "Yes"
            } else {
                "No"
            })
            .with_style(Attr::ForegroundColor(if todo["done"].as_bool().unwrap() {
                color::GREEN
            } else {
                color::RED
            })),
            Cell::new(todo["created_at"].as_str().unwrap()),
            Cell::new(todo["updated_at"].as_str().unwrap()),
        ]));
    }

    table.printstd();
    return Ok(());
}

pub fn new(data: Vec<String>, flags: Vec<char>) -> Result<(), &'static str> {
    let mut todo_data = parse_data();
    let id_cursor = todo_data["id_cursor"].as_i32().unwrap();
    let current_time = current_time();
    let current_time = current_time.as_str();

    if data.len() < 1 {
        return Err("Missing text of a todo");
    }

    let mut new_todo = object! {id: id_cursor+ 1, text: data[0].as_str(), done: false, created_at:current_time, updated_at: current_time };

    for flag in flags {
        if flag == 'd' {
            new_todo["done"] = true.into();
        }

        if flag == 'n' {
            new_todo["done"] = false.into();
        }
    }

    todo_data["todos"]
        .push(new_todo.clone())
        .expect("Something went wrong writing the data file");

    todo_data["id_cursor"] = (id_cursor + 1).into();

    save_data(todo_data).handle_error();
    println!(
        "New todo \"{}\" have been created with ID {}",
        style(new_todo["text"].as_str().unwrap()).green(),
        style(new_todo["id"].as_i32().unwrap()).blue()
    );

    return Ok(());
}

pub fn remove(data: Vec<String>) -> Result<(), &'static str> {
    let mut todo_data = parse_data();
    if data.len() < 1 {
        return Err("Missing id of a todo");
    }

    let id_to_remove: i32 = match data[0].parse() {
        Ok(id) => id,
        Err(_) => return Err("Invalid id"),
    };

    if let Some(index_to_remove) = todo_data["todos"]
        .members()
        .position(|todo| todo["id"].as_i32() == Some(id_to_remove))
    {
        todo_data["todos"].array_remove(index_to_remove);
        save_data(todo_data).handle_error();
    } else {
        return Err("Todo not found");
    }

    println!(
        "Todo with ID {} have been removed",
        style(id_to_remove).blue()
    );

    Ok(())
}

pub fn update(data: Vec<String>, flags: Vec<char>) -> Result<(), &'static str> {
    let mut todo_data = parse_data();
    let current_time = current_time();
    let current_time = current_time.as_str();

    if data.len() < 2 {
        return Err("Missing arguments");
    }

    let id_to_update: i32 = match data[0].clone().parse() {
        Ok(id) => id,
        Err(_) => return Err("Invalid id"),
    };

    let text = data[1].clone();

    if let Some(index_to_update) = todo_data["todos"]
        .members()
        .position(|todo| todo["id"].as_i32() == Some(id_to_update))
    {
        todo_data["todos"][index_to_update]["text"] = text.into();
        for flag in flags {
            if flag == 'd' {
                todo_data["todos"][index_to_update]["done"] = true.into();
            }

            if flag == 'n' {
                todo_data["todos"][index_to_update]["done"] = false.into();
            }
        }
        todo_data["todos"][index_to_update]["updated_at"] = current_time.into();
        save_data(todo_data).handle_error();
    } else {
        return Err("Todo not found");
    }

    println!(
        "Todo with ID {} have been updated",
        style(id_to_update).blue()
    );

    Ok(())
}

pub fn set_done(data: Vec<String>, is_done: bool) -> Result<(), &'static str> {
    let mut todo_data = parse_data();
    let current_time = current_time();
    let current_time = current_time.as_str();

    if data.len() < 1 {
        return Err("Missing id argument");
    }

    let id_to_update: i32 = match data[0].parse() {
        Ok(id) => id,
        Err(_) => return Err("Invalid id"),
    };

    if let Some(index_to_update) = todo_data["todos"]
        .members()
        .position(|todo| todo["id"].as_i32() == Some(id_to_update))
    {
        todo_data["todos"][index_to_update]["done"] = is_done.into();
        todo_data["todos"][index_to_update]["updated_at"] = current_time.into();
        save_data(todo_data).handle_error();
    } else {
        return Err("Todo not found");
    }

    println!(
        "Todo with ID {} have been updated",
        style(id_to_update).blue()
    );

    Ok(())
}

pub fn reset(flags: Vec<char>) -> Result<(), &'static str> {
    let mut is_forced = false;
    let confirmed: bool;

    for flag in flags {
        if flag == 'f' {
            is_forced = true;
        }
    }

    if !is_forced {
        confirmed = Confirm::new()
            .with_prompt("This will delete all your todos are you sure?")
            .interact()
            .unwrap();
    } else {
        confirmed = true;
    }

    if confirmed {
        reset_data().handle_error();
    }

    println!("Data file successfully reseted");

    return Ok(());
}
