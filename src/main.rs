use app::{App, Page};
use relm4::RelmApp;

mod app;
mod player_name_cell;
mod player_name_row;
mod remove_turn_button;
mod score_page;
mod tallied_score_cell;
mod tallied_score_row;
mod timer_page;
mod turn_number;
mod turn_score_cell;
mod turn_score_row;
mod utils;

fn main() {
    let app = RelmApp::new("com.github.weclaw1.ScoreTracker");
    relm4_icons::initialize_icons();
    app.run::<App>(Page::Score);
}
