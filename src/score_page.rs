use relm4::prelude::*;
use adw::prelude::*;
use std::convert::identity;

use crate::score_header::{ScoreHeader, ScoreHeaderInput, self};

pub struct ScorePage {
    players: usize,
    score_header: Controller<ScoreHeader>,
}

#[derive(Debug)]
pub enum ScorePageInput {
    AddPlayer,
    RemovePlayer,
}

#[relm4::component(pub)]
impl SimpleComponent for ScorePage {
    type Init = usize;
    type Input = ScorePageInput;
    type Output = ();

    view! {
        gtk::ScrolledWindow {
            set_vexpand: true,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_halign: gtk::Align::Center,

                append: model.score_header.widget(),
            }
        }
    }

    fn update(&mut self, msg: ScorePageInput, _sender: ComponentSender<Self>) {
        match msg {
            ScorePageInput::AddPlayer => {
                self.players = self.players.saturating_add(1);
                self.score_header.sender().send(ScoreHeaderInput::AddPlayer).unwrap();
            }
            ScorePageInput::RemovePlayer => {
                self.players = self.players.saturating_sub(1);
                self.score_header.sender().send(ScoreHeaderInput::RemovePlayer).unwrap();
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let score_header = ScoreHeader::builder()
            .launch(init)
            .detach();

        let model = ScorePage {
            players: init,
            score_header,

        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
