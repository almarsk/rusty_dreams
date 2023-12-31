#[macro_use]
extern crate rocket;
use std::collections::HashMap;

use rocket::form::Form;
use rocket::response::content::RawHtml;
mod get_template;
use get_template::get_template;

#[derive(FromForm)]
struct LoginForm {
    username: String,
    password: String,
}

#[rocket::post("/submit", data = "<form>")]
fn submit(form: Form<LoginForm>) -> RawHtml<String> {
    println!("got {} and {}", form.username, form.password);
    let mut data = HashMap::new();
    data.insert("nickname".to_string(), form.username.clone());
    get_template("chat", Some(data))
}

#[rocket::get("/")]
fn index() -> RawHtml<String> {
    get_template("login", None)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, submit])
}
