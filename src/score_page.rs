use relm4::prelude::*;
use adw::prelude::*;

pub struct ScorePage {
    players: Vec<String>,
}

#[derive(Debug)]
pub enum ScorePageMsg {
    AddPlayer,
    RemovePlayer,
}

#[relm4::component(pub)]
impl SimpleComponent for ScorePage {
    type Init = Vec<String>;
    type Input = ScorePageMsg;
    type Output = ();

    view! {
        gtk::ScrolledWindow {
            set_vexpand: true,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                gtk::Label {
                    #[watch]
                    set_label: &format!("Players: {}", model.players.len()),
                    set_margin_all: 5,
                }
            }
        }
    }

    fn update(&mut self, msg: ScorePageMsg, _sender: ComponentSender<Self>) {
        match msg {
            ScorePageMsg::AddPlayer => {
                self.players.push("Player".to_string());
            }
            ScorePageMsg::RemovePlayer => {
                self.players.pop();
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ScorePage {
            players: init,
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
