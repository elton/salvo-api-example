// Provides RESTful API for this example:
//
// - `GET /users`: return JSON list of users
// - `POST /users`: create a new user entry
// - `PUT /users/:id`: update a specific user
// - `DELETE /users/:id`: delete a specific user

use salvo::prelude::*;

use crate::handlers;

pub fn api() -> Router {
    Router::with_path("api")
        .get(handlers::index)
        .push(Router::with_path("name/<name>").get(handlers::get_params))
        .push(Router::with_path("title").get(handlers::get_query))
        .push(Router::with_path("user").post(handlers::create_user))
}
