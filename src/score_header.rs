use relm4::{prelude::*, factory::FactoryVecDeque};
use relm4::gtk::prelude::*;

use crate::score_header_cell::ScoreHeaderCell;

pub struct ScoreHeader {
    score_header_cells: FactoryVecDeque<ScoreHeaderCell>,
}

#[derive(Debug)]
pub enum ScoreHeaderInput {
    AddPlayer,
    RemovePlayer,
}

#[relm4::component(pub)]
impl SimpleComponent for ScoreHeader {
    type Init = usize;
    type Input = ScoreHeaderInput;
    type Output = ();

    view! {
        gtk::Box {
            #[local_ref]
            players_box -> gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
            }
        }
    }

    fn update(&mut self, msg: ScoreHeaderInput, _sender: ComponentSender<Self>) {
        match msg {
            ScoreHeaderInput::AddPlayer => {
                let new_player_number = self.score_header_cells.len() + 1;
                self.score_header_cells.guard().push_back(format!("Player {}", new_player_number));
            }
            ScoreHeaderInput::RemovePlayer => {
                self.score_header_cells.guard().pop_back();
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut score_header_cells = FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());
        for n in 1..=init {
            score_header_cells.guard().push_back(format!("Player {}", n));
        }

        let model = ScoreHeader {
            score_header_cells,
        };

        let players_box = model.score_header_cells.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
