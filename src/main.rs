use app::{App, Page};
use relm4::RelmApp;

mod app;
mod localization;
mod player_name_cell;
mod player_name_row;
mod remove_turn_button;
mod score_page;
mod tallied_score_cell;
mod tallied_score_row;
mod timer;
mod timer_editor;
mod timer_page;
mod turn_number;
mod turn_score_cell;
mod turn_score_row;
mod utils;

fn main() {
    let app = RelmApp::new("io.github.weclaw1.ScoreTracker");
    relm4_icons::initialize_icons();
    localization::init();
    app.run::<App>(Page::Score);
}
