use relm4::prelude::*;
use relm4::gtk::prelude::*;

use crate::tallied_score_row::TalliedScoreRowInput;

#[derive(Debug)]
pub enum TalliedScoreCellInput {
    ScoreChanged(i32),
}

pub struct TalliedScoreCell {
    score: i32,
}

#[relm4::factory(pub)]
impl FactoryComponent for TalliedScoreCell {
    type Init = i32;
    type Input = TalliedScoreCellInput;
    type Output = ();
    type CommandOutput = ();
    type ParentInput = TalliedScoreRowInput;
    type ParentWidget = gtk::Box;

    view! {
        gtk::Button {
            set_hexpand: true,
            set_halign: gtk::Align::Fill,
            #[watch]
            set_label: &self.score.to_string(),
        }
    }

    fn update(&mut self, message: TalliedScoreCellInput, _sender: FactorySender<Self>) {
        match message {
            TalliedScoreCellInput::ScoreChanged(score) => {
                self.score = score;
            }
        }
    }

    fn init_model(
        init: Self::Init,
        _index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self {
            score: init,
        }
    }
}