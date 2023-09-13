use relm4::prelude::*;
use adw::prelude::*;

pub struct TimerPage;

#[relm4::component(pub)]
impl SimpleComponent for TimerPage {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        gtk::ScrolledWindow {
            set_vexpand: true,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                gtk::Label {
                    #[watch]
                    set_label: &format!("Timer Page"),
                    set_margin_all: 5,
                }
            }
        }
    }

    fn update(&mut self, _msg: (), _sender: ComponentSender<Self>) {
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = TimerPage;
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
