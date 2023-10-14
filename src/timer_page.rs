use std::time::Duration;

use adw::prelude::*;
use relm4::{factory::FactoryVecDeque, prelude::*};

use crate::timer::Timer;

pub struct TimerPage {
    timers: FactoryVecDeque<Timer>,
}

#[relm4::component(pub)]
impl SimpleComponent for TimerPage {
    type Init = Duration;
    type Input = ();
    type Output = ();

    view! {
        gtk::ScrolledWindow {
            set_hscrollbar_policy: gtk::PolicyType::Never,
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_halign: gtk::Align::Center,
                set_margin_all: 5,

                #[local_ref]
                timer_box -> gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                },
            }
        }
    }

    fn update(&mut self, _msg: (), _sender: ComponentSender<Self>) {}

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut timers = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .detach();

        timers.guard().push_back(init);

        let model = TimerPage { timers };

        let timer_box = model.timers.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
