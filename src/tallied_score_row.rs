use relm4::factory::FactoryVecDeque;
use relm4::gtk::prelude::*;
use relm4::prelude::*;

use crate::tallied_score_cell::{TalliedScoreCell, TalliedScoreCellInput};

pub struct TalliedScoreRow {
    tallied_score_cells: FactoryVecDeque<TalliedScoreCell>,
}

#[derive(Debug, Clone)]
pub enum TalliedScoreRowInput {
    AddPlayer,
    RemovePlayer,
    ScoreChanged(usize, i32),
    FirstPlaceChanged(usize, bool),
    LastPlaceChanged(usize, bool),
}

#[relm4::component(pub)]
impl SimpleComponent for TalliedScoreRow {
    type Init = usize;
    type Input = TalliedScoreRowInput;
    type Output = ();

    view! {
        gtk::Box {
            set_hexpand: true,
            set_spacing: 5,
            set_halign: gtk::Align::Fill,
            set_valign: gtk::Align::End,

            #[local_ref]
            tallied_score_cells_box -> gtk::Box {
                set_spacing: 5,
                set_hexpand: true,
                set_halign: gtk::Align::Fill,
                set_orientation: gtk::Orientation::Horizontal,
            },
        }
    }

    fn update(&mut self, message: TalliedScoreRowInput, _sender: ComponentSender<Self>) {
        match message {
            TalliedScoreRowInput::AddPlayer => {
                self.tallied_score_cells.guard().push_back(0);
            }
            TalliedScoreRowInput::RemovePlayer => {
                self.tallied_score_cells.guard().pop_back();
            }
            TalliedScoreRowInput::ScoreChanged(player_index, score) => {
                self.tallied_score_cells
                    .guard()
                    .send(player_index, TalliedScoreCellInput::ScoreChanged(score));
            }
            TalliedScoreRowInput::FirstPlaceChanged(player_index, first_place) => {
                self.tallied_score_cells.guard().send(
                    player_index,
                    TalliedScoreCellInput::FirstPlaceChanged(first_place),
                );
            }
            TalliedScoreRowInput::LastPlaceChanged(player_index, last_place) => {
                self.tallied_score_cells.guard().send(
                    player_index,
                    TalliedScoreCellInput::LastPlaceChanged(last_place),
                );
            }
        }
    }

    fn init(
        init: Self::Init,
        _root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let tallied_score_cells = FactoryVecDeque::from_iter(vec![0; init], gtk::Box::default());

        let model = Self {
            tallied_score_cells,
        };

        let tallied_score_cells_box = model.tallied_score_cells.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
