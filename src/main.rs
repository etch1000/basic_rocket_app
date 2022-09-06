#[macro_use]
extern crate rocket;

#[get("/hello")]
fn hello() -> &'static str {
    "Hi!"
}

#[get("/world")]
fn world() -> &'static str {
    "Hello, World!"
}

#[get("/<name>/<age>")]
fn name_age(name: String, age: u8) -> String {
    format!("Hello {}, you entered your age as: {}", name, age)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/hello", routes![world])
        .mount("/wave", routes![name_age])
}