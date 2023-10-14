use relm4::factory::FactoryVecDeque;
use relm4::gtk::prelude::*;
use relm4::prelude::*;

use crate::turn_score_cell::{TurnScoreCell, TurnScoreCellOutput};

pub struct TurnScoreRow {
    turn_row_index: DynamicIndex,
    turn_score_cells: FactoryVecDeque<TurnScoreCell>,
}

#[derive(Debug, Clone)]
pub enum TurnScoreRowInput {
    AddPlayer,
    RemovePlayer,
    ScoreChanged(DynamicIndex, i32),
}

#[derive(Debug)]
pub enum TurnScoreRowOutput {
    ScoreChanged(DynamicIndex, DynamicIndex, i32),
}

#[relm4::factory(pub)]
impl FactoryComponent for TurnScoreRow {
    type Init = usize;
    type Input = TurnScoreRowInput;
    type Output = TurnScoreRowOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        gtk::Box {
            set_hexpand: true,
            set_spacing: 5,
            set_halign: gtk::Align::Fill,

            #[local_ref]
            turn_score_cells_box -> gtk::Box {
                set_spacing: 5,
                set_hexpand: true,
                set_halign: gtk::Align::Fill,
                set_orientation: gtk::Orientation::Horizontal,
            },
        }
    }

    fn update(&mut self, message: TurnScoreRowInput, sender: FactorySender<Self>) {
        match message {
            TurnScoreRowInput::AddPlayer => {
                self.turn_score_cells.guard().push_back(0);
            }
            TurnScoreRowInput::RemovePlayer => {
                self.turn_score_cells.guard().pop_back();
            }
            TurnScoreRowInput::ScoreChanged(player_index, score) => {
                sender
                    .output(TurnScoreRowOutput::ScoreChanged(
                        self.turn_row_index.clone(),
                        player_index,
                        score,
                    ))
                    .unwrap();
            }
        }
    }

    fn init_model(init: Self::Init, index: &DynamicIndex, sender: FactorySender<Self>) -> Self {
        let mut turn_score_cells = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .forward(sender.input_sender(), |msg| match msg {
                TurnScoreCellOutput::ScoreChanged(player_index, score) => {
                    TurnScoreRowInput::ScoreChanged(player_index, score)
                }
            });

        {
            let mut turn_score_cells = turn_score_cells.guard();
            for _ in 0..init {
                turn_score_cells.push_back(0);
            }
        }

        Self {
            turn_row_index: index.clone(),
            turn_score_cells,
        }
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        _root: &Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        _sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let turn_score_cells_box = self.turn_score_cells.widget();
        let widgets = view_output!();
        widgets
    }
}
