https://betterprogramming.pub/how-to-write-a-web-app-in-rust-part-1-3047156660a7


﻿# rust-todo
notable:
  - rust
  - rocket
 
includes: 
  - web app for todos
  - http requests

biggest challenges:

  - error started as parseError for id: u8, to editTodo and deleteTodo
    - refactored wrong names, rePost original test todo, new post included id
    - then able to editTodo, delete success w/ 404 error
    - -> delete success with 200, had to simply remove a semicolon after removing the Option<>, None for the '&string  
