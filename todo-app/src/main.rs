use std::{fs::{OpenOptions}, io::{Write}};
use rocket::serde::{Deserialize, json::Json};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    item: &'r str
}

#[post("/addtask", data="<task>")]
fn add_task(task: Json<Task<'_>>) -> &'static str {
    let mut tasks = OpenOptions::new()
                    .read(true)
                    .append(true)
                    .create(true)
                    .open("tasks.txt")
                    .expect("unable to access tasks.txt");
    let task_item_string = format!("{}\n", task.item);
    let task_item_bytes = task_item_string.as_bytes();
    task.write(task_item_bytes).expect("unable to write to tasks.txt");
    "Task added successfully"
}

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, add_task])
}