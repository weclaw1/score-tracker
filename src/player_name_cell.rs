use relm4::gtk::prelude::*;
use relm4::prelude::*;

use crate::player_name_row::PlayerNameRowInput;

pub struct PlayerNameCell {
    value: gtk::EntryBuffer,
}

#[relm4::factory(pub)]
impl FactoryComponent for PlayerNameCell {
    type Init = String;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentInput = PlayerNameRowInput;
    type ParentWidget = gtk::Box;

    view! {
        gtk::Entry {
            set_hexpand: true,
            set_buffer: &self.value,
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            value: gtk::EntryBuffer::new(Some(init)),
        }
    }
}
