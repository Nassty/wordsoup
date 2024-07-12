mod app;
mod board_display;
mod words;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
