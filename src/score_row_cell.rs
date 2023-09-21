use relm4::prelude::*;
use relm4::gtk::prelude::WidgetExt;

use crate::score_row::ScoreRowInput;

pub struct ScoreRowCell {
    score: i32,
}

#[relm4::factory(pub)]
impl FactoryComponent for ScoreRowCell {
    type Init = i32;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentInput = ScoreRowInput;
    type ParentWidget = gtk::Box;

    view! {
        gtk::SpinButton {
            set_hexpand: true,
            set_halign: gtk::Align::Fill,
            set_adjustment: &gtk::Adjustment::new(0.0, 0.0, 9999.0, 1.0, 0.0, 0.0),
            set_climb_rate: 0.0,
            set_digits: 0,
            set_numeric: true,
            set_value: self.score as f64,
        }
    }

    fn init_model(
        init: Self::Init,
        _index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self {
            score: init,
        }
    }
}