use relm4::gtk::prelude::*;
use relm4::{factory::FactoryVecDeque, prelude::*};
use relm4_icons::icon_name;

use crate::tallied_score_row::TalliedScoreRowInput;
use crate::utils;
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
    players_with_highest_score: Vec<usize>,
    players_with_lowest_score: Vec<usize>,
}

impl ScorePage {
    fn add_player(&mut self) {
        self.players += 1;
        self.player_name_row.emit(PlayerNameRowInput::AddPlayer);
        self.turn_score_rows
            .guard()
            .broadcast(TurnScoreRowInput::AddPlayer);
        self.tallied_score_row.emit(TalliedScoreRowInput::AddPlayer);
        self.player_scores.iter_mut().for_each(|row| row.push(0));
    }

    fn remove_player(&mut self) {
        self.players -= 1;
        self.player_name_row.emit(PlayerNameRowInput::RemovePlayer);
        self.turn_score_rows
            .guard()
            .broadcast(TurnScoreRowInput::RemovePlayer);
        self.tallied_score_row
            .emit(TalliedScoreRowInput::RemovePlayer);
        self.player_scores.iter_mut().for_each(|row| {
            row.pop();
        });
        self.players_with_highest_score
            .retain(|player_index| *player_index < self.players);
        self.players_with_lowest_score
            .retain(|player_index| *player_index < self.players);
    }

    fn add_row(&mut self) {
        self.turn_score_rows.guard().push_back(self.players);
        self.turn_numbers.guard().push_back(());
        self.remove_turn_buttons.guard().push_back(());
        self.player_scores.push(vec![0; self.players]);
    }

    fn calculate_tallied_score(player_index: usize, player_scores: &[Vec<i32>]) -> i32 {
        player_scores.iter().map(|row| row[player_index]).sum()
    }

    fn calculate_tallied_scores(player_scores: &[Vec<i32>]) -> Vec<i32> {
        if player_scores.is_empty() {
            return Vec::new();
        }
        (0..player_scores[0].len())
            .map(|player_index| Self::calculate_tallied_score(player_index, player_scores))
            .collect()
    }

    fn remove_score_row(&mut self, index: usize) {
        self.turn_score_rows.guard().remove(index);
        self.turn_numbers.guard().remove(index);
        self.turn_numbers
            .guard()
            .broadcast(TurnNumberInput::UpdateTurnNumber);
        self.remove_turn_buttons.guard().remove(index);
        let removed_score_row = self.player_scores.remove(index);
        removed_score_row
            .into_iter()
            .enumerate()
            .filter(|(_, score)| *score != 0)
            .map(|(player_index, _)| {
                (
                    player_index,
                    Self::calculate_tallied_score(player_index, &self.player_scores),
                )
            })
            .for_each(|(player_index, score)| {
                self.tallied_score_row
                    .emit(TalliedScoreRowInput::ScoreChanged(player_index, score));
            });
    }

    fn score_changed(&mut self, row_index: usize, player_index: usize, score: i32) {
        self.player_scores[row_index][player_index] = score;
        let tallied_player_score =
            ScorePage::calculate_tallied_score(player_index, &self.player_scores);
        self.tallied_score_row
            .emit(TalliedScoreRowInput::ScoreChanged(
                player_index,
                tallied_player_score,
            ));
    }

    fn find_players_with_lowest_score(player_scores: &[Vec<i32>]) -> Vec<usize> {
        let tallied_scores = Self::calculate_tallied_scores(player_scores);
        let lowest_score = tallied_scores.iter().min();
        match lowest_score {
            Some(lowest_score) => tallied_scores
                .iter()
                .enumerate()
                .filter(|(_, score)| *score == lowest_score)
                .map(|(player_index, _)| player_index)
                .collect(),
            None => Vec::new(),
        }
    }

    fn find_players_with_highest_score(player_scores: &[Vec<i32>]) -> Vec<usize> {
        let tallied_scores = Self::calculate_tallied_scores(player_scores);
        let highest_score = tallied_scores.iter().max();
        match highest_score {
            Some(highest_score) => tallied_scores
                .iter()
                .enumerate()
                .filter(|(_, score)| *score == highest_score)
                .map(|(player_index, _)| player_index)
                .collect(),
            None => Vec::new(),
        }
    }

    fn update_players_with_lowest_score(&mut self) {
        let updated_players_with_lowest_score =
            Self::find_players_with_lowest_score(&self.player_scores);
        let players_with_lowest_score_diff = utils::symetric_difference_between_two_arrays(
            &self.players_with_lowest_score,
            &updated_players_with_lowest_score,
        );
        players_with_lowest_score_diff
            .into_iter()
            .for_each(|player_index| {
                self.tallied_score_row
                    .emit(TalliedScoreRowInput::LastPlaceChanged(
                        *player_index,
                        updated_players_with_lowest_score.contains(player_index),
                    ));
            });
        self.players_with_lowest_score = updated_players_with_lowest_score;
    }

    fn update_players_with_highest_score(&mut self) {
        let updated_players_with_highest_score =
            Self::find_players_with_highest_score(&self.player_scores);
        let players_with_highest_score_diff = utils::symetric_difference_between_two_arrays(
            &self.players_with_highest_score,
            &updated_players_with_highest_score,
        );
        players_with_highest_score_diff
            .into_iter()
            .for_each(|player_index| {
                self.tallied_score_row
                    .emit(TalliedScoreRowInput::FirstPlaceChanged(
                        *player_index,
                        updated_players_with_highest_score.contains(player_index),
                    ));
            });
        self.players_with_highest_score = updated_players_with_highest_score;
    }
}

#[derive(Debug)]
pub enum ScorePageInput {
    AddPlayer,
    RemovePlayer,
    AddRow,
    RemoveScoreRow(DynamicIndex),
    ScoreChanged(DynamicIndex, DynamicIndex, i32),
    UpdatePlayersWithHighestScore,
    UpdatePlayersWithLowestScore,
}

#[relm4::component(pub)]
impl SimpleComponent for ScorePage {
    type Init = (usize, usize);
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
                        add_css_class: "success",
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
                        add_css_class: "suggested-action",
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
                        set_margin_bottom: 4,
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
                        add_css_class: "error",

                        #[watch]
                        set_sensitive: model.players > 1,

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

    fn update(&mut self, message: ScorePageInput, sender: ComponentSender<Self>) {
        match message {
            ScorePageInput::AddPlayer => {
                self.add_player();
                sender.input(ScorePageInput::UpdatePlayersWithLowestScore);
                sender.input(ScorePageInput::UpdatePlayersWithHighestScore);
            }
            ScorePageInput::RemovePlayer => {
                self.remove_player();
                sender.input(ScorePageInput::UpdatePlayersWithLowestScore);
                sender.input(ScorePageInput::UpdatePlayersWithHighestScore);
            }
            ScorePageInput::AddRow => {
                self.add_row();
                sender.input(ScorePageInput::UpdatePlayersWithLowestScore);
                sender.input(ScorePageInput::UpdatePlayersWithHighestScore);
            }
            ScorePageInput::RemoveScoreRow(index) => {
                self.remove_score_row(index.current_index());
                sender.input(ScorePageInput::UpdatePlayersWithLowestScore);
                sender.input(ScorePageInput::UpdatePlayersWithHighestScore);
            }
            ScorePageInput::ScoreChanged(row_index, player_index, score) => {
                self.score_changed(
                    row_index.current_index(),
                    player_index.current_index(),
                    score,
                );
                sender.input(ScorePageInput::UpdatePlayersWithLowestScore);
                sender.input(ScorePageInput::UpdatePlayersWithHighestScore);
            }
            ScorePageInput::UpdatePlayersWithLowestScore => self.update_players_with_lowest_score(),
            ScorePageInput::UpdatePlayersWithHighestScore => {
                self.update_players_with_highest_score()
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (initial_players, initial_score_rows) = init;
        let player_name_row = PlayerNameRow::builder().launch(initial_players).detach();

        let turn_numbers = FactoryVecDeque::from_iter(
            vec![(); initial_score_rows],
            gtk::Box::default(),
            sender.input_sender(),
        );

        let remove_turn_buttons = FactoryVecDeque::from_iter(
            vec![(); initial_score_rows],
            gtk::Box::default(),
            sender.input_sender(),
        );

        let turn_score_rows = FactoryVecDeque::from_iter(
            vec![initial_players; initial_score_rows],
            gtk::Box::default(),
            sender.input_sender(),
        );

        let tallied_score_row = TalliedScoreRow::builder().launch(initial_players).detach();

        let model = Self {
            players: initial_players,
            player_name_row,
            turn_score_rows,
            turn_numbers,
            remove_turn_buttons,
            tallied_score_row,
            player_scores: vec![vec![0; initial_players]; initial_score_rows],
            players_with_highest_score: Vec::new(),
            players_with_lowest_score: Vec::new(),
        };

        let turn_score_row_box = model.turn_score_rows.widget();
        let turn_numbers_box = model.turn_numbers.widget();
        let remove_turn_buttons_box = model.remove_turn_buttons.widget();
        let widgets = view_output!();

        sender.input(ScorePageInput::UpdatePlayersWithHighestScore);
        sender.input(ScorePageInput::UpdatePlayersWithLowestScore);

        ComponentParts { model, widgets }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_tallied_score() {
        let player_scores = vec![vec![10, 20, 30], vec![5, 15, 25], vec![15, 25, 35]];
        let tallied_score = ScorePage::calculate_tallied_score(1, &player_scores);
        assert_eq!(tallied_score, 60);
    }

    #[test]
    fn test_calculate_tallied_score_empty() {
        let player_scores = vec![];
        let tallied_score = ScorePage::calculate_tallied_score(0, &player_scores);
        assert_eq!(tallied_score, 0);
    }

    #[test]
    fn test_calculate_tallied_score_first_player() {
        let player_scores = vec![vec![10, 20, 30], vec![5, 15, 25], vec![15, 25, 35]];
        let tallied_score = ScorePage::calculate_tallied_score(0, &player_scores);
        assert_eq!(tallied_score, 30);
    }

    #[test]
    fn test_calculate_tallied_score_last_player() {
        let player_scores = vec![vec![10, 20, 30], vec![5, 15, 25], vec![15, 25, 35]];
        let tallied_score = ScorePage::calculate_tallied_score(2, &player_scores);
        assert_eq!(tallied_score, 90);
    }

    #[test]
    fn test_calculate_tallied_score_with_negative_score() {
        let player_scores = vec![vec![10, 20, -30], vec![5, 15, 25], vec![15, 25, -35]];
        let tallied_score = ScorePage::calculate_tallied_score(2, &player_scores);
        assert_eq!(tallied_score, -40);
    }

    #[test]
    fn test_calculate_tallied_scores() {
        let player_scores = vec![vec![10, 20, -30], vec![20, 30, -40], vec![30, 40, -50]];
        let tallied_scores = ScorePage::calculate_tallied_scores(&player_scores);
        assert_eq!(tallied_scores.len(), 3);
        assert_eq!(tallied_scores[0], 60);
        assert_eq!(tallied_scores[1], 90);
        assert_eq!(tallied_scores[2], -120);
    }

    #[test]
    fn test_calculate_tallied_scores_empty() {
        let player_scores = vec![];
        let tallied_scores = ScorePage::calculate_tallied_scores(&player_scores);
        assert_eq!(tallied_scores.len(), 0);
    }

    #[test]
    fn test_find_players_with_lowest_score() {
        let player_scores = vec![vec![10, 20, 1], vec![20, 30, 2], vec![30, 40, 3]];
        let players_with_lowest_score = ScorePage::find_players_with_lowest_score(&player_scores);
        assert_eq!(players_with_lowest_score.len(), 1);
        assert_eq!(players_with_lowest_score[0], 2);
    }

    #[test]
    fn test_find_players_with_lowest_score_empty() {
        let player_scores = vec![];
        let players_with_lowest_score = ScorePage::find_players_with_lowest_score(&player_scores);
        assert_eq!(players_with_lowest_score.len(), 0);
    }

    #[test]
    fn test_find_players_with_lowest_score_tie() {
        let player_scores = vec![vec![10, 20, 20], vec![20, 30, 30], vec![30, 40, 10]];
        let players_with_lowest_score = ScorePage::find_players_with_lowest_score(&player_scores);
        assert_eq!(players_with_lowest_score.len(), 2);
        assert!(players_with_lowest_score.contains(&0));
        assert!(players_with_lowest_score.contains(&2));
    }

    #[test]
    fn test_find_players_with_highest_score() {
        let player_scores = vec![vec![10, 20, 1], vec![20, 30, 2], vec![30, 40, 3]];
        let players_with_highest_score = ScorePage::find_players_with_highest_score(&player_scores);
        assert_eq!(players_with_highest_score.len(), 1);
        assert_eq!(players_with_highest_score[0], 1);
    }

    #[test]
    fn test_find_players_with_highest_score_empty() {
        let player_scores = vec![];
        let players_with_highest_score = ScorePage::find_players_with_highest_score(&player_scores);
        assert_eq!(players_with_highest_score.len(), 0);
    }

    #[test]
    fn test_find_players_with_highest_score_tie() {
        let player_scores = vec![vec![10, 20, 40], vec![20, 30, 30], vec![30, 40, 20]];
        let players_with_highest_score = ScorePage::find_players_with_highest_score(&player_scores);
        assert_eq!(players_with_highest_score.len(), 2);
        assert!(players_with_highest_score.contains(&1));
        assert!(players_with_highest_score.contains(&2));
    }
}
