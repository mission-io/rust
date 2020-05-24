#[get("/world")]
pub fn world() -> &'static str {
    "Hello, world!"
}

#[get("/<name>/<age>")]
pub fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}