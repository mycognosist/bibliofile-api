// 'src/bin/main.rs'


#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate tera;

// Bring Template and Context into scope
use rocket_contrib::Template;
use tera::Context;

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing()) // This lets us use templating
        .launch();
}

#[get("/")]
fn index() -> Template {
    let mut context = Context::new();

    context.add("my_message", &"Hola from template context!");
    Template::render("base", &context)
}
