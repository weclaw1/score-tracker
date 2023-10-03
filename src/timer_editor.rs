use std::time::Duration;

use relm4::gtk::glib::Propagation;
use relm4::gtk::prelude::*;
use relm4::prelude::*;

pub struct TimerEditor {
    hours: u8,
    minutes: u8,
    seconds: u8,
}

impl TimerEditor {
    pub fn duration(&self) -> Duration {
        Duration::from_secs(
            self.hours as u64 * 3600 + self.minutes as u64 * 60 + self.seconds as u64,
        )
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub enum TimerEditorInput {
    SetHours(u8),
    SetMinutes(u8),
    SetSeconds(u8),
}

#[derive(Debug, Clone)]
pub enum TimerEditorOutput {
    DurationChanged(Duration),
}

#[relm4::component(pub)]
impl SimpleComponent for TimerEditor {
    type Init = Duration;
    type Input = TimerEditorInput;
    type Output = TimerEditorOutput;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 5,
            set_halign: gtk::Align::Center,
            set_css_classes: &["title-1"],

            gtk::SpinButton {
                set_adjustment: &gtk::Adjustment::new(0.0, 0.0, 99.0, 1.0, 0.0, 0.0),
                set_climb_rate: 0.0,
                set_digits: 0,
                set_numeric: true,
                set_value: model.hours as f64,
                set_orientation: gtk::Orientation::Vertical,
                connect_output => move |spin_button| {
                    let value = spin_button.value() as u8;
                    if value < 10 {
                        spin_button.set_text(&format!("0{}", value));
                        return Propagation::Stop
                    }
                    Propagation::Proceed
                },
                connect_value_changed[sender] => move |spin_button| {
                    sender.input(TimerEditorInput::SetHours(spin_button.value() as u8));
                }
            },
            gtk::Label {
                set_label: ":",
            },
            gtk::SpinButton {
                set_adjustment: &gtk::Adjustment::new(0.0, 0.0, 59.0, 1.0, 0.0, 0.0),
                set_climb_rate: 0.0,
                set_digits: 0,
                set_numeric: true,
                set_value: model.minutes as f64,
                set_orientation: gtk::Orientation::Vertical,
                connect_output => move |spin_button| {
                    let value = spin_button.value() as u8;
                    if value < 10 {
                        spin_button.set_text(&format!("0{}", value));
                        return Propagation::Stop
                    }
                    Propagation::Proceed
                },
                connect_value_changed[sender] => move |spin_button| {
                    sender.input(TimerEditorInput::SetMinutes(spin_button.value() as u8));
                }
            },
            gtk::Label {
                set_label: ":",
            },
            gtk::SpinButton {
                set_adjustment: &gtk::Adjustment::new(0.0, 0.0, 59.0, 1.0, 0.0, 0.0),
                set_climb_rate: 0.0,
                set_digits: 0,
                set_numeric: true,
                set_value: model.seconds as f64,
                set_orientation: gtk::Orientation::Vertical,
                connect_output => move |spin_button| {
                    let value = spin_button.value() as u8;
                    if value < 10 {
                        spin_button.set_text(&format!("0{}", value));
                        return Propagation::Stop
                    }
                    Propagation::Proceed
                },
                connect_value_changed[sender] => move |spin_button| {
                    sender.input(TimerEditorInput::SetSeconds(spin_button.value() as u8));
                }
            },
        }
    }

    fn update(&mut self, message: TimerEditorInput, sender: ComponentSender<Self>) {
        match message {
            TimerEditorInput::SetHours(hours) => {
                self.hours = hours;
            }
            TimerEditorInput::SetMinutes(minutes) => {
                self.minutes = minutes;
            }
            TimerEditorInput::SetSeconds(seconds) => {
                self.seconds = seconds;
            }
        }
        sender
            .output(TimerEditorOutput::DurationChanged(self.duration()))
            .unwrap();
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let hours = init.as_secs() / 3600;
        let minutes = (init.as_secs() % 3600) / 60;
        let seconds = init.as_secs() % 60;

        let model = Self {
            hours: hours as u8,
            minutes: minutes as u8,
            seconds: seconds as u8,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
