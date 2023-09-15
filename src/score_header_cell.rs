use relm4::prelude::*;
use relm4::gtk::prelude::*;

use crate::score_header::ScoreHeaderInput;

pub struct ScoreHeaderCell {
    player_name: gtk::EntryBuffer,
}

#[relm4::factory(pub)]
impl FactoryComponent for ScoreHeaderCell {
    type Init = String;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentInput = ScoreHeaderInput;
    type ParentWidget = gtk::Box;

    view! {
        gtk::Entry {
            set_buffer: &self.player_name,
            connect_activate: |_| {},
        }
    }

    fn init_model(
        init: Self::Init,
        _index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self {
            player_name: gtk::EntryBuffer::new(Some(init)),
        }
    }
}