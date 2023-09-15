use app::{App, Page};
use relm4::RelmApp;

mod app;
mod score_page;
mod timer_page;
mod score_header;
mod score_header_cell;

fn main() {
    let app = RelmApp::new("com.github.weclaw1.ScoreTracker");
    relm4_icons::initialize_icons();
    app.run::<App>(Page::Score);
}