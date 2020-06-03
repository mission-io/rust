#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
mod routes;

use rocket::response::content;

#[catch(404)]
fn not_found(req: &rocket::Request<'_>) -> content::Html<String> {
    content::Html(format!(
        "<p>Sorry, but '{}' is not a valid path!</p>
            <p>Try visiting /hello/&lt;name&gt;/&lt;age&gt; instead.</p>",
        req.uri()
    ))
}

fn main() {
    rocket::ignite()
        .mount("/hello", routes![routes::user::hello, routes::user::world])
        .register(catchers![not_found])
        .launch();
}
