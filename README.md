# It's a Rusty TO-DO list!

## To run:
1. Install docker, docker-compose
2. Run docker-compose up

## Routes:
* GET http://127.0.0.1:8000/tasks
* CREATE http://127.0.0.1:8000/tasks
```json
{
  "title": "Some title",
  "body": "Some body"
}
```
* DELETE http://127.0.0.1:8000/tasks/{id}

## Powered by:
* Rust!
* PostgreSQL - as the datastore
* [Rocket](https://rocket.rs/) - as the web framework
* [Diesel](https://diesel.rs/) - as the ORM
