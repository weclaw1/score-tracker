use relm4::gtk::prelude::*;
use relm4::prelude::*;

use crate::score_page::ScorePageInput;

#[derive(Debug, Clone)]
pub enum TurnNumberInput {
    UpdateTurnNumber,
}

pub struct TurnNumber {
    index: DynamicIndex,
    value: usize,
}

#[relm4::factory(pub)]
impl FactoryComponent for TurnNumber {
    type Init = ();
    type Input = TurnNumberInput;
    type Output = ();
    type CommandOutput = ();
    type ParentInput = ScorePageInput;
    type ParentWidget = gtk::Box;

    view! {
        gtk::Label {
            set_margin_top: 7,
            set_margin_bottom: 8,
            #[watch]
            set_label: &format!("{}.", self.value),
        }
    }

    fn init_model(_init: Self::Init, index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            index: index.clone(),
            value: index.current_index() + 1,
        }
    }

    fn update(&mut self, msg: TurnNumberInput, _sender: FactorySender<Self>) {
        match msg {
            TurnNumberInput::UpdateTurnNumber => {
                self.value = self.index.current_index() + 1;
            }
        }
    }
}
