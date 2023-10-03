use relm4::gtk::prelude::*;
use relm4::prelude::*;
use relm4_icons::icon_name;

#[derive(Debug)]
pub enum RemoveTurnButtonOutput {
    RemoveScoreRow(DynamicIndex),
}

pub struct RemoveTurnButton;

#[relm4::factory(pub)]
impl FactoryComponent for RemoveTurnButton {
    type Init = ();
    type Input = ();
    type Output = RemoveTurnButtonOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        gtk::Button {
            set_icon_name: icon_name::CROSS_FILLED,
            set_css_classes: &["destructive-action"],
            connect_clicked[sender, index] => move |_| sender.output(RemoveTurnButtonOutput::RemoveScoreRow(index.clone())),
        },
    }

    fn init_model(_init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self
    }
}
