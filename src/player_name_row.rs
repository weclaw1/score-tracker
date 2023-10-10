use relm4::factory::FactoryVecDeque;
use relm4::gtk::prelude::*;
use relm4::prelude::*;

use crate::fl;
use crate::player_name_cell::PlayerNameCell;

pub struct PlayerNameRow {
    player_name_cells: FactoryVecDeque<PlayerNameCell>,
}

#[derive(Debug, Clone)]
pub enum PlayerNameRowInput {
    AddPlayer,
    RemovePlayer,
}
#[relm4::component(pub)]
impl SimpleComponent for PlayerNameRow {
    type Init = usize;
    type Input = PlayerNameRowInput;
    type Output = ();

    view! {
        gtk::Box {
            #[local_ref]
            player_names_box -> gtk::Box {
                set_spacing: 5,
                set_orientation: gtk::Orientation::Horizontal,
            },

        }
    }

    fn update(&mut self, message: PlayerNameRowInput, _sender: ComponentSender<Self>) {
        match message {
            PlayerNameRowInput::AddPlayer => {
                let new_player_number = self.player_name_cells.guard().len() + 1;
                self.player_name_cells
                    .guard()
                    .push_back(format!("{} {}", fl!("player"), new_player_number));
            }
            PlayerNameRowInput::RemovePlayer => {
                self.player_name_cells.guard().pop_back();
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let player_name_cells = FactoryVecDeque::from_iter(
            (1..=init).map(|n| format!("{} {}", fl!("player"), n)),
            gtk::Box::default(),
        );

        let model = Self { player_name_cells };

        let player_names_box = model.player_name_cells.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
