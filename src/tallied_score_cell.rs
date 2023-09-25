use relm4::gtk::prelude::*;
use relm4::prelude::*;

use crate::tallied_score_row::TalliedScoreRowInput;

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum TalliedScoreCellInput {
    ScoreChanged(i32),
    FirstPlaceChanged(bool),
    LastPlaceChanged(bool),
}

pub struct TalliedScoreCell {
    score: i32,
    first_place: bool,
    last_place: bool,
}

impl TalliedScoreCell {
    fn css_class_list(&self) -> Vec<&str> {
        match (self.first_place, self.last_place) {
            (true, true) | (true, false) => vec!["frame", "success"],
            (false, true) => vec!["frame", "error"],
            (false, false) => vec!["frame"],
        }
    }
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
        gtk::Box {
            #[watch]
            set_css_classes: &self.css_class_list(),
            set_hexpand: true,

            gtk::Label {
                set_halign: gtk::Align::Center,
                set_hexpand: true,
                add_css_class: "title-2",
                set_width_chars: 10,

                #[watch]
                set_label: &self.score.to_string(),
            }
        }
    }

    fn update(&mut self, message: TalliedScoreCellInput, _sender: FactorySender<Self>) {
        match message {
            TalliedScoreCellInput::ScoreChanged(score) => {
                self.score = score;
            }
            TalliedScoreCellInput::FirstPlaceChanged(first_place) => {
                self.first_place = first_place;
            }
            TalliedScoreCellInput::LastPlaceChanged(last_place) => {
                self.last_place = last_place;
            }
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            score: init,
            first_place: false,
            last_place: false,
        }
    }
}
