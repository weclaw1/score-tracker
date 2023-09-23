use relm4::gtk::prelude::WidgetExt;
use relm4::prelude::*;

use crate::turn_score_row::TurnScoreRowInput;

#[derive(Debug)]
pub enum TurnScoreCellOutput {
    ScoreChanged(DynamicIndex, i32),
}

pub struct TurnScoreCell {
    score: i32,
}

#[relm4::factory(pub)]
impl FactoryComponent for TurnScoreCell {
    type Init = i32;
    type Input = ();
    type Output = TurnScoreCellOutput;
    type CommandOutput = ();
    type ParentInput = TurnScoreRowInput;
    type ParentWidget = gtk::Box;

    view! {
        gtk::SpinButton {
            set_hexpand: true,
            set_halign: gtk::Align::Fill,
            set_adjustment: &gtk::Adjustment::new(0.0, -9999.0, 9999.0, 1.0, 0.0, 0.0),
            set_climb_rate: 0.0,
            set_digits: 0,
            set_numeric: true,
            set_value: self.score as f64,
            connect_value_changed[sender, index] => move |spin_button| {
                sender.output(TurnScoreCellOutput::ScoreChanged(index.clone(), spin_button.value() as i32));
            },
        }
    }

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(match output {
            TurnScoreCellOutput::ScoreChanged(player_index, score) => {
                TurnScoreRowInput::ScoreChanged(player_index, score)
            }
        })
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { score: init }
    }
}
