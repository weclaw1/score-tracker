use relm4::{prelude::*, factory::FactoryVecDeque};
use adw::prelude::*;
use relm4_icons::icon_name;

use crate::{score_row::{ScoreRow, ScoreRowInput}, turn_number::{TurnNumber, TurnNumberInput}, remove_turn_button::RemoveTurnButton, player_name_row::{PlayerNameRow, self, PlayerNameRowInput}};

pub struct ScorePage {
    players: usize,
    player_name_row: Controller<PlayerNameRow>,
    score_rows: FactoryVecDeque<ScoreRow>,
    turn_numbers: FactoryVecDeque<TurnNumber>,
    remove_turn_buttons: FactoryVecDeque<RemoveTurnButton>,
}

#[derive(Debug)]
pub enum ScorePageInput {
    AddPlayer,
    RemovePlayer,
    AddRow,
    RemoveScoreRow(DynamicIndex),
}

#[relm4::component(pub)]
impl SimpleComponent for ScorePage {
    type Init = usize;
    type Input = ScorePageInput;
    type Output = ();

    view! {
        #[name="scrolled_window"]
        gtk::ScrolledWindow {
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_halign: gtk::Align::Center,
                set_margin_all: 5,
                set_spacing: 5,

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                    gtk::Button {
                        set_icon_name: icon_name::PERSON_ADD_REGULAR,
                        connect_clicked => ScorePageInput::AddPlayer,
                    },

                    #[local_ref]
                    turn_numbers_box -> gtk::Box {
                        set_spacing: 5,
                        set_vexpand: true,
                        set_orientation: gtk::Orientation::Vertical,
                    },
                },
                gtk::Box {
                    set_spacing: 5,
                    set_orientation: gtk::Orientation::Vertical,
    
                    append = model.player_name_row.widget(),

                    #[local_ref]
                    score_row_box -> gtk::Box {
                        set_spacing: 5,
                        set_hexpand: true,
                        set_orientation: gtk::Orientation::Vertical,
                        set_halign: gtk::Align::Fill,
                    },
                    
                    gtk::Button {
                        set_icon_name: icon_name::PLUS,
                        connect_clicked => ScorePageInput::AddRow,
                    },
                },
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,

                    gtk::Button {
                        set_icon_name: icon_name::PERSON_SUBTRACT_REGULAR,
                        connect_clicked => ScorePageInput::RemovePlayer,
                    },

                    #[local_ref]
                    remove_turn_buttons_box -> gtk::Box {
                        set_spacing: 5,
                        set_orientation: gtk::Orientation::Vertical,
                    },
                }
            }
        }
    }

    fn update(&mut self, message: ScorePageInput, _sender: ComponentSender<Self>) {
        match message {
            ScorePageInput::AddPlayer => {
                self.players = self.players.saturating_add(1);
                self.player_name_row.emit(PlayerNameRowInput::AddPlayer);
                self.score_rows.guard().broadcast(ScoreRowInput::AddPlayer);
            }
            ScorePageInput::RemovePlayer => {
                self.players = self.players.saturating_sub(1);
                self.player_name_row.emit(PlayerNameRowInput::RemovePlayer);
                self.score_rows.guard().broadcast(ScoreRowInput::RemovePlayer);
            }
            ScorePageInput::AddRow => {
                self.score_rows.guard().push_back(self.players);
                self.turn_numbers.guard().push_back(());
                self.remove_turn_buttons.guard().push_back(());
            }
            ScorePageInput::RemoveScoreRow(index) => {
                self.score_rows.guard().remove(index.current_index());
                self.turn_numbers.guard().remove(index.current_index());
                self.turn_numbers.guard().broadcast(TurnNumberInput::UpdateTurnNumber);
                self.remove_turn_buttons.guard().remove(index.current_index());
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let player_name_row = PlayerNameRow::builder()
            .launch(init)
            .detach();

        let mut turn_numbers = FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());
        for _ in 1..=100 {
            turn_numbers.guard().push_back(());
        }
        //turn_numbers.guard().push_back(());

        let mut remove_turn_buttons = FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());
        for _ in 1..=100 {
            remove_turn_buttons.guard().push_back(());
        }
        //remove_turn_buttons.guard().push_back(());

        let mut score_rows = FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());
        for _ in 1..=100 {
            score_rows.guard().push_back(init);
        }
        //score_rows.guard().push_back(init);
    
        let model = Self {
            players: init,
            player_name_row,
            score_rows,
            turn_numbers,
            remove_turn_buttons,
        };

        let score_row_box = model.score_rows.widget();
        let turn_numbers_box = model.turn_numbers.widget();
        let remove_turn_buttons_box = model.remove_turn_buttons.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
