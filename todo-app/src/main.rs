use rocket::serde::{json::Json, Deserialize, Serialize};
use std::io::BufRead;
use std::io::BufReader;
use std::{fs::OpenOptions, io::Write};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Todo<'r> {
    item: &'r str,
}

#[post("/addtodo", data = "<todo>")]
fn add_todo(todo: Json<Todo<'_>>) -> &'static str {
    let mut todos = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("todos.txt")
        .expect("unable to access todos.txt");
    let reader = BufReader::new(&todos);
    let id = reader.lines().count();
    let todo_item_string = format!("{},{}\n", id, todo.item);
    let todo_item_bytes = todo_item_string.as_bytes();
    todos
        .write(todo_item_bytes)
        .expect("unable to write to todos.txt");
    "todo added successfully"
}

#[get("/readtodos")]
fn read_todos() -> Json<Vec<String>> {
    let todos = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("todos.txt")
        .expect("unable to access todos.txt");
    let reader = BufReader::new(todos);
    Json(
        reader
            .lines()
            .map(|line| {
                let line_string: String = line.expect("could not read line");
                let line_pieces: Vec<&str> = line_string.split(",").collect();
                line_pieces[1].to_string()
            })
            .collect(),
    )
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct TodoUpdate<'r> {
    id: u8,
    item: &'r str,
}

#[put("/edittodo", data = "<todo_update>")]
fn edit_task(todo_update: Json<TodoUpdate<'_>>) -> &'static str {
    let todos = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("todos.txt")
        .expect("unable to access todos.txt");
    let mut temp = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("temp.txt")
        .expect("unable to access temp.txt");

    let reader = BufReader::new(todos);
    for line in reader.lines() {
        let line_string: String = line.expect("could not read line");
        let line_pieces: Vec<&str> = line_string.split(",").collect();

        if line_pieces[0]
            .parse::<u8>()
            .expect("unable to parse id as u8")
            == todo_update.id
        {
            let todo_items: [&str; 2] = [line_pieces[0], todo_update.item];
            let todo = format!("{}\n", todo_items.join(","));
            temp.write(todo.as_bytes())
                .expect("could not write to temp file");
        } else {
            let todo = format!("{}\n", line_string);
            temp.write(todo.as_bytes())
                .expect("could not write to temp file");
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct TodoId {
    id: u8,
}

#[delete("/deletetask", data = "<todo_id>")]
fn delete_task(todo_id: Json<TodoId>) -> &'static str {
    let todos = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("todos.txt")
        .expect("unable to access todos.txt");
    let mut temp = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("temp.txt")
        .expect("unable to access temp.txt");
    let reader = BufReader::new(todos);

    for line in reader.lines() {
        let line_string: String = line.expect("could not read line");
        let line_pieces: Vec<&str> = line_string.split(",").collect();

        if line_pieces[0]
            .parse::<u8>()
            .expect("unable to parse id as u8")
            != todo_id.id
        {
            let todo = format!("{}\n", line_string);
            temp.write(todo.as_bytes())
                .expect("could not write to temp file");
        }
    }
}

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, add_todo, read_todos])
}
