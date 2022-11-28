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
    - then able to editTodo, delete sucess w/ 404 error
