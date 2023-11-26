use crate::db;
use crate::models::dog::Dog;
use crate::utils::response::*;
use crate::*;
use rocket_contrib::json::{Json, JsonError};

#[get("/")]
fn read(conn: db::Connection) -> Result<ApiResponse, ApiError> {
    let result = Dog::read(&conn);
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}

#[get("/<id>")]
fn read_by_id(id: i32, conn: db::Connection) -> Result<ApiResponse, ApiError> {
    let result = Dog::read_by_id(id, &conn);
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}

#[post("/", data = "<dog>")]
fn create(
    dog: Result<Json<Dog>, JsonError>,
    conn: db::Connection,
) -> Result<ApiResponse, ApiError> {
    match dog {
        Ok(u) => {
            let insert = Dog {
                id: None,
                ..u.into_inner()
            };
            let result = Dog::create(insert, &conn);
            match result {
                Ok(r) => Ok(success(json!(r))),
                Err(e) => Err(db_error(e)),
            }
        }
        Err(e) => Err(json_error(e)),
    }
}

#[put("/<id>", data = "<dog>")]
fn update(
    id: i32,
    dog: Result<Json<Dog>, JsonError>,
    conn: db::Connection,
) -> Result<ApiResponse, ApiError> {
    match dog {
        Ok(u) => {
            let update = Dog {
                id: Some(id),
                ..u.into_inner()
            };
            let result = Dog::update(id, update, &conn);
            match result {
                Ok(r) => Ok(success(json!(r))),
                Err(e) => Err(db_error(e)),
            }
        }
        Err(e) => Err(json_error(e)),
    }
}

#[delete("/<id>")]
fn delete(id: i32, conn: db::Connection) -> ApiResponse {
    let result = Dog::delete(id, &conn);
    success(json!(result))
}

// -- routes
pub fn routes() -> Vec<rocket::Route> {
    routes![read, read_by_id, create, update, delete]
}