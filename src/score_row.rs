use relm4::factory::FactoryVecDeque;
use relm4::prelude::*;
use relm4::gtk::prelude::*;
use relm4_icons::icon_name;

use crate::score_page::ScorePageInput;
use crate::score_row_cell::ScoreRowCell;

pub struct ScoreRow {
    score_row_cells: FactoryVecDeque<ScoreRowCell>,
}

#[derive(Debug, Clone)]
pub enum ScoreRowInput {
    AddPlayer,
    RemovePlayer,
}

#[relm4::factory(pub)]
impl FactoryComponent for ScoreRow {
    type Init = usize;
    type Input = ScoreRowInput;
    type Output = ();
    type CommandOutput = ();
    type ParentInput = ScorePageInput;
    type ParentWidget = gtk::Box;

    view! {
        gtk::Box {
            set_hexpand: true,
            set_spacing: 5,
            set_halign: gtk::Align::Fill,

            #[local_ref]
            players_box -> gtk::Box {
                set_spacing: 5,
                set_hexpand: true,
                set_halign: gtk::Align::Fill,
                set_orientation: gtk::Orientation::Horizontal,
            },
        }
    }

    fn update(&mut self, msg: ScoreRowInput, _sender: FactorySender<Self>) {
        match msg {
            ScoreRowInput::AddPlayer => {
                self.score_row_cells.guard().push_back(0);
            }
            ScoreRowInput::RemovePlayer => {
                self.score_row_cells.guard().pop_back();
            }
        }
    }

    fn init_model(
        init: Self::Init,
        _index: &DynamicIndex,
        sender: FactorySender<Self>,
    ) -> Self {
        let mut score_row_cells = FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());
        for _ in 1..=init {
            score_row_cells.guard().push_back(0);
        }

        Self {
            score_row_cells,
        }
    }

    fn init_widgets(
            &mut self,
            _index: &Self::Index,
            _root: &Self::Root,
            _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
            _sender: FactorySender<Self>,
        ) -> Self::Widgets {
            let players_box = self.score_row_cells.widget();
            let widgets = view_output!();
            widgets
    }
}