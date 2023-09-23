use app::{App, Page};
use relm4::RelmApp;

mod app;
mod score_page;
mod timer_page;
mod player_name_cell;
mod turn_score_row;
mod turn_score_cell;
mod turn_number;
mod remove_turn_button;
mod player_name_row;
mod tallied_score_cell;
mod tallied_score_row;

fn main() {
    let app = RelmApp::new("com.github.weclaw1.ScoreTracker");
    relm4_icons::initialize_icons();
    app.run::<App>(Page::Score);
}