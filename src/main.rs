use std::net::TcpListener; //Importing the library for the TcpListener
use rocket::serde::{Serialize, json::Json}; //Importing the rocket library for the Json type

#[macro_use] extern crate rocket; //Importing the rocket library

#[get("/")] //Defining the route -> GET /
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize)] //Deriving the Serialize trait to be able to convert the struct to JSON
struct NotFoundError {
    status: u16,
    message: String,
}

#[catch(404)]
fn not_found(req: &rocket::Request<'_>) -> Json<NotFoundError> {
    Json(NotFoundError {
        status: 404,
        message: format!("'{}' was not found.", req.uri()),
    })
}

#[launch] //Launching rocket
fn rocket() -> _ {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap(); //Creating a TcpListener to get a free port
    let port = listener.local_addr().unwrap().port(); //Getting the port
    drop(listener); //Dropping the listener

    let figment = rocket::Config::figment() //Creating a figment (Rocket's configuration)
        .merge(("address", "127.0.0.1")) //Setting the address to localhost
        .merge(("port", port)) //Setting the port to the free port
        .merge(("cli_colors", false)) //Disabling the colors in the console
        .merge(("log_level", "normal")); //Disabling the logs in the console

    rocket::custom(figment).mount("/", routes![index]).register("/", catchers![not_found]) //Creating the rocket instance
}