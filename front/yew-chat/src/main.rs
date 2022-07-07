use app::App;

pub use weblog;

mod comp;
mod app;
mod services;

fn main() {
    yew::start_app::<App>();
}