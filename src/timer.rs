use std::time::Duration;

use relm4::gtk::prelude::*;
use relm4::prelude::*;
use relm4_icons::icon_name;

use crate::timer_editor::{TimerEditor, TimerEditorOutput};
use crate::utils::HumanReadableDuration;

#[derive(Debug, Clone, PartialEq)]
pub enum TimerState {
    Running(Duration, Duration),
    Paused(Duration, Duration),
    Stopped(Duration),
}

impl TimerState {
    pub fn remaining_duration(&self) -> Duration {
        match self {
            TimerState::Running(elapsed, duration) => duration.saturating_sub(*elapsed),
            TimerState::Paused(elapsed, duration) => duration.saturating_sub(*elapsed),
            TimerState::Stopped(duration) => *duration,
        }
    }

    fn start(self) -> Self {
        match self {
            TimerState::Stopped(duration) => TimerState::Running(Duration::from_secs(0), duration),
            TimerState::Paused(elapsed, duration) => TimerState::Running(elapsed, duration),
            _ => self,
        }
    }

    fn pause(self) -> Self {
        match self {
            TimerState::Running(elapsed, duration) => TimerState::Paused(elapsed, duration),
            _ => self,
        }
    }

    fn reset(self) -> Self {
        match self {
            TimerState::Paused(_, duration) => TimerState::Stopped(duration),
            _ => self,
        }
    }

    fn tick(self) -> Self {
        match self {
            TimerState::Running(elapsed, duration)
                if elapsed + Duration::from_secs(1) >= duration =>
            {
                TimerState::Stopped(duration)
            }
            TimerState::Running(elapsed, duration) => {
                TimerState::Running(elapsed + Duration::from_secs(1), duration)
            }
            _ => self,
        }
    }
}

pub struct Timer {
    state: Option<TimerState>,
    editor_duration: Duration,
    timer_editor: Controller<TimerEditor>,
}

#[derive(Debug)]
pub enum TimerInput {
    Start,
    Pause,
    Reset,
    Create,
    Remove,
    EditorDurationChanged(Duration),
}

#[derive(Debug)]
pub enum TimerCommandOutput {
    Tick,
}

impl Timer {
    fn start(&mut self) {
        self.state = self.state.take().map(|state| state.start());
    }

    fn pause(&mut self) {
        self.state = self.state.take().map(|state| state.pause());
    }

    fn reset(&mut self) {
        self.state = self.state.take().map(|state| state.reset());
    }

    fn tick(&mut self) {
        self.state = self.state.take().map(|state| state.tick());
    }

    fn remove(&mut self) {
        self.state = None;
    }

    fn create(&mut self) {
        self.state = Some(TimerState::Stopped(self.editor_duration));
    }
}

#[relm4::factory(pub)]
impl FactoryComponent for Timer {
    type Init = Duration;
    type Input = TimerInput;
    type Output = ();
    type CommandOutput = TimerCommandOutput;
    type ParentWidget = gtk::Box;

    view! {
        adw::Clamp {
            set_css_classes: &["card"],
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 10,
                set_spacing: 10,

                if let Some(timer_state) = &self.state {
                    gtk::Label {
                        set_css_classes: &["title-1"],
                        #[watch]
                        set_label: &HumanReadableDuration::new(timer_state.remaining_duration()).to_string(),
                    }
                } else {
                    gtk::Box {
                        set_halign: gtk::Align::Center,
                        append: self.timer_editor.widget()
                    }
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,

                    match &self.state {
                        Some(TimerState::Running(_, _)) => {
                            gtk::Button {
                                set_icon_name: icon_name::PAUSE,
                                set_width_request: 40,
                                set_height_request: 40,
                                set_css_classes: &["circular"],
                                set_halign: gtk::Align::Center,
                                connect_clicked => TimerInput::Pause,
                            }
                        }
                        Some(TimerState::Paused(_, _)) => {
                            gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: 10,
                                set_halign: gtk::Align::Center,

                                gtk::Button {
                                    set_icon_name: icon_name::REFRESH,
                                    set_css_classes: &["circular"],
                                    set_halign: gtk::Align::Center,
                                    set_valign: gtk::Align::Center,
                                    connect_clicked => TimerInput::Reset,
                                },
                                gtk::Button {
                                    set_icon_name: icon_name::PLAY,
                                    set_width_request: 40,
                                    set_height_request: 40,
                                    set_css_classes: &["circular", "suggested-action"],
                                    set_halign: gtk::Align::Center,
                                    connect_clicked => TimerInput::Start,
                                },
                                gtk::Button {
                                    set_icon_name: icon_name::DELETE_FILLED,
                                    set_valign: gtk::Align::Center,
                                    set_css_classes: &["circular"],
                                    set_halign: gtk::Align::Center,
                                    connect_clicked => TimerInput::Remove,
                                },
                            }
                        }
                        Some(TimerState::Stopped(_)) => {
                            gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: 10,
                                set_margin_start: 44,
                                set_halign: gtk::Align::Center,

                                gtk::Button {
                                    set_icon_name: icon_name::PLAY,
                                    set_width_request: 40,
                                    set_height_request: 40,
                                    set_css_classes: &["circular", "suggested-action"],
                                    set_halign: gtk::Align::Center,
                                    connect_clicked => TimerInput::Start,
                                },
                                gtk::Button {
                                    set_icon_name: icon_name::DELETE_FILLED,
                                    set_valign: gtk::Align::Center,
                                    set_css_classes: &["circular"],
                                    set_halign: gtk::Align::Center,
                                    connect_clicked => TimerInput::Remove,
                                },
                            }
                        }
                        None => {
                            gtk::Button {
                                set_icon_name: icon_name::PLAY,
                                set_width_request: 40,
                                set_height_request: 40,
                                set_css_classes: &["circular", "suggested-action"],
                                set_halign: gtk::Align::Center,
                                #[watch]
                                set_sensitive: !self.editor_duration.is_zero(),
                                connect_clicked => TimerInput::Create,
                            }
                        }
                    },
                }
            }
        }
    }

    fn update(&mut self, message: TimerInput, sender: FactorySender<Self>) {
        match message {
            TimerInput::Start => {
                self.start();
                sender.spawn_oneshot_command(|| {
                    std::thread::sleep(Duration::from_secs(1));
                    TimerCommandOutput::Tick
                });
            }
            TimerInput::Pause => self.pause(),
            TimerInput::Reset => self.reset(),
            TimerInput::Remove => self.remove(),
            TimerInput::Create => {
                self.create();
                sender.input(TimerInput::Start);
            }
            TimerInput::EditorDurationChanged(duration) => {
                self.editor_duration = duration;
            }
        }
    }

    fn update_cmd(&mut self, message: TimerCommandOutput, sender: FactorySender<Self>) {
        match message {
            TimerCommandOutput::Tick => {
                self.tick();
                if let Some(TimerState::Running(_, _)) = &self.state {
                    sender.spawn_oneshot_command(|| {
                        std::thread::sleep(Duration::from_secs(1));
                        TimerCommandOutput::Tick
                    });
                }
            }
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, sender: FactorySender<Self>) -> Self {
        let timer_editor =
            TimerEditor::builder()
                .launch(init)
                .forward(sender.input_sender(), |message| match message {
                    TimerEditorOutput::DurationChanged(duration) => {
                        TimerInput::EditorDurationChanged(duration)
                    }
                });

        Self {
            state: None,
            editor_duration: init,
            timer_editor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remaining_duration() {
        let duration = Duration::from_secs(10);
        let state = TimerState::Stopped(duration);
        assert_eq!(state.remaining_duration(), duration);

        let elapsed = Duration::from_secs(5);
        let state = TimerState::Running(elapsed, duration);
        assert_eq!(state.remaining_duration(), duration - elapsed);

        let state = TimerState::Paused(elapsed, duration);
        assert_eq!(state.remaining_duration(), duration - elapsed);
    }

    #[test]
    fn test_start() {
        let duration = Duration::from_secs(10);
        let state = TimerState::Stopped(duration);
        assert_eq!(
            state.start(),
            TimerState::Running(Duration::from_secs(0), duration)
        );

        let elapsed = Duration::from_secs(5);
        let state = TimerState::Paused(elapsed, duration);
        assert_eq!(state.start(), TimerState::Running(elapsed, duration));

        let state = TimerState::Running(elapsed, duration);
        assert_eq!(state.clone().start(), state);
    }

    #[test]
    fn test_pause() {
        let duration = Duration::from_secs(10);
        let elapsed = Duration::from_secs(5);
        let state = TimerState::Running(elapsed, duration);
        assert_eq!(state.pause(), TimerState::Paused(elapsed, duration));

        let state = TimerState::Paused(elapsed, duration);
        assert_eq!(state.clone().pause(), state);

        let state = TimerState::Stopped(duration);
        assert_eq!(state.clone().pause(), state);
    }

    #[test]
    fn test_reset() {
        let duration = Duration::from_secs(10);
        let elapsed = Duration::from_secs(5);
        let state = TimerState::Paused(elapsed, duration);
        assert_eq!(state.reset(), TimerState::Stopped(duration));

        let state = TimerState::Stopped(duration);
        assert_eq!(state.clone().reset(), state);

        let state = TimerState::Running(elapsed, duration);
        assert_eq!(state.clone().reset(), state);
    }

    #[test]
    fn test_tick() {
        let duration = Duration::from_secs(10);
        let elapsed = Duration::from_secs(5);
        let state = TimerState::Running(elapsed, duration);
        assert_eq!(
            state.tick(),
            TimerState::Running(elapsed + Duration::from_secs(1), duration)
        );

        let elapsed = Duration::from_secs(9);
        let state = TimerState::Running(elapsed, duration);
        assert_eq!(state.tick(), TimerState::Stopped(duration));

        let state = TimerState::Paused(elapsed, duration);
        assert_eq!(state.clone().tick(), state);

        let state = TimerState::Stopped(duration);
        assert_eq!(state.clone().tick(), state);
    }
}
