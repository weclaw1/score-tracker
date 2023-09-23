use relm4::gtk::prelude::*;
use relm4::{factory::FactoryVecDeque, prelude::*};
use relm4_icons::icon_name;

use crate::tallied_score_row::TalliedScoreRowInput;
use crate::{
    player_name_row::{PlayerNameRow, PlayerNameRowInput},
    remove_turn_button::RemoveTurnButton,
    tallied_score_row::TalliedScoreRow,
    turn_number::{TurnNumber, TurnNumberInput},
    turn_score_row::{TurnScoreRow, TurnScoreRowInput},
};

pub struct ScorePage {
    players: usize,
    player_name_row: Controller<PlayerNameRow>,
    turn_score_rows: FactoryVecDeque<TurnScoreRow>,
    turn_numbers: FactoryVecDeque<TurnNumber>,
    remove_turn_buttons: FactoryVecDeque<RemoveTurnButton>,
    tallied_score_row: Controller<TalliedScoreRow>,
    player_scores: Vec<Vec<i32>>,
}

#[derive(Debug)]
pub enum ScorePageInput {
    AddPlayer,
    RemovePlayer,
    AddRow,
    RemoveScoreRow(DynamicIndex),
    ScoreChanged(DynamicIndex, DynamicIndex, i32),
}

#[relm4::component(pub)]
impl SimpleComponent for ScorePage {
    type Init = usize;
    type Input = ScorePageInput;
    type Output = ();

    view! {
        gtk::ScrolledWindow {
            set_hscrollbar_policy: gtk::PolicyType::Never,
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_hexpand: true,
                set_spacing: 5,
                set_margin_vertical: 5,

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_hexpand: true,
                    set_spacing: 5,
                    set_vexpand: true,

                    gtk::Button {
                        set_margin_start: 5,
                        set_halign: gtk::Align::End,
                        set_icon_name: icon_name::PERSON_ADD_REGULAR,
                        connect_clicked => ScorePageInput::AddPlayer,
                    },

                    gtk::ScrolledWindow {
                        set_hexpand: true,
                        set_vadjustment: Some(&main_column_scrolled_window.vadjustment()),
                        set_vexpand: true,
                        set_vscrollbar_policy: gtk::PolicyType::External,
                        set_hscrollbar_policy: gtk::PolicyType::Never,

                        #[local_ref]
                        turn_numbers_box -> gtk::Box {
                            set_margin_start: 5,
                            set_hexpand: true,
                            set_halign: gtk::Align::End,
                            set_spacing: 5,
                            set_orientation: gtk::Orientation::Vertical,
                        },
                    },

                    gtk::Button {
                        set_margin_start: 5,
                        set_halign: gtk::Align::End,
                        set_valign: gtk::Align::End,
                        set_icon_name: icon_name::PLUS,
                        connect_clicked => ScorePageInput::AddRow,
                    },
                },
                gtk::Box {
                    set_spacing: 5,
                    set_hexpand: true,
                    set_orientation: gtk::Orientation::Vertical,
                    set_vexpand: true,

                    append = model.player_name_row.widget(),

                    #[name="main_column_scrolled_window"]
                    gtk::ScrolledWindow {
                        set_vexpand: true,
                        set_valign: gtk::Align::Fill,
                        set_vscrollbar_policy: gtk::PolicyType::External,
                        set_hscrollbar_policy: gtk::PolicyType::Never,

                        #[local_ref]
                        turn_score_row_box -> gtk::Box {
                            set_spacing: 5,
                            set_hexpand: true,
                            set_vexpand: true,
                            set_orientation: gtk::Orientation::Vertical,
                            set_halign: gtk::Align::Fill,
                        },
                    },

                    append = model.tallied_score_row.widget(),
                },
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_hexpand: true,
                    set_spacing: 5,
                    set_vexpand: true,

                    gtk::Button {
                        set_margin_end: 5,
                        set_halign: gtk::Align::Start,
                        set_icon_name: icon_name::PERSON_SUBTRACT_REGULAR,
                        connect_clicked => ScorePageInput::RemovePlayer,
                    },

                    gtk::ScrolledWindow {
                        set_hexpand: true,
                        set_vadjustment: Some(&main_column_scrolled_window.vadjustment()),
                        set_vexpand: true,
                        set_vscrollbar_policy: gtk::PolicyType::Automatic,
                        set_hscrollbar_policy: gtk::PolicyType::Never,
                        set_margin_bottom: 39,

                        #[local_ref]
                        remove_turn_buttons_box -> gtk::Box {
                            set_margin_end: 5,
                            set_halign: gtk::Align::Start,
                            set_spacing: 5,
                            set_orientation: gtk::Orientation::Vertical,
                        },
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
                self.turn_score_rows
                    .guard()
                    .broadcast(TurnScoreRowInput::AddPlayer);
                self.tallied_score_row.emit(TalliedScoreRowInput::AddPlayer);
                self.player_scores.iter_mut().for_each(|row| row.push(0));
            }
            ScorePageInput::RemovePlayer => {
                self.players = self.players.saturating_sub(1);
                self.player_name_row.emit(PlayerNameRowInput::RemovePlayer);
                self.turn_score_rows
                    .guard()
                    .broadcast(TurnScoreRowInput::RemovePlayer);
                self.tallied_score_row
                    .emit(TalliedScoreRowInput::RemovePlayer);
                self.player_scores.iter_mut().for_each(|row| {
                    row.pop();
                });
            }
            ScorePageInput::AddRow => {
                self.turn_score_rows.guard().push_back(self.players);
                self.turn_numbers.guard().push_back(());
                self.remove_turn_buttons.guard().push_back(());
                self.player_scores.push(vec![0; self.players]);
            }
            ScorePageInput::RemoveScoreRow(index) => {
                self.turn_score_rows.guard().remove(index.current_index());
                self.turn_numbers.guard().remove(index.current_index());
                self.turn_numbers
                    .guard()
                    .broadcast(TurnNumberInput::UpdateTurnNumber);
                self.remove_turn_buttons
                    .guard()
                    .remove(index.current_index());
                let removed_score_row = self.player_scores.remove(index.current_index());
                removed_score_row
                    .into_iter()
                    .enumerate()
                    .filter(|(_, score)| *score != 0)
                    .map(|(player_index, _)| {
                        (
                            player_index,
                            self.player_scores.iter().map(|row| row[player_index]).sum(),
                        )
                    })
                    .for_each(|(player_index, score)| {
                        self.tallied_score_row
                            .emit(TalliedScoreRowInput::ScoreChanged(player_index, score));
                    });
            }
            ScorePageInput::ScoreChanged(row_index, player_index, score) => {
                self.player_scores[row_index.current_index()][player_index.current_index()] = score;
                let tallied_player_score = self
                    .player_scores
                    .iter()
                    .map(|row| row[player_index.current_index()])
                    .sum();
                self.tallied_score_row
                    .emit(TalliedScoreRowInput::ScoreChanged(
                        player_index.current_index(),
                        tallied_player_score,
                    ));
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let player_name_row = PlayerNameRow::builder().launch(init).detach();

        let mut turn_numbers = FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());
        for _ in 1..=10 {
            turn_numbers.guard().push_back(());
        }
        //turn_numbers.guard().push_back(());

        let mut remove_turn_buttons =
            FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());
        for _ in 1..=10 {
            remove_turn_buttons.guard().push_back(());
        }
        //remove_turn_buttons.guard().push_back(());

        let mut turn_score_rows = FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());
        for _ in 1..=10 {
            turn_score_rows.guard().push_back(init);
        }
        //score_rows.guard().push_back(init);

        let tallied_score_row = TalliedScoreRow::builder().launch(init).detach();

        let model = Self {
            players: init,
            player_name_row,
            turn_score_rows,
            turn_numbers,
            remove_turn_buttons,
            tallied_score_row,
            player_scores: vec![vec![0; init]; 10],
        };

        let turn_score_row_box = model.turn_score_rows.widget();
        let turn_numbers_box = model.turn_numbers.widget();
        let remove_turn_buttons_box = model.remove_turn_buttons.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
