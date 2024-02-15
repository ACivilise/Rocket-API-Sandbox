extern crate leptos;

fn main() {
    leptos::App::new().start(|app| {
        app.get("/", |req, res| {
            res.send("Hello, world!");
        });
    });
}