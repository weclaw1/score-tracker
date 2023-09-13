use adw::prelude::*;
use relm4::prelude::*;
use relm4_icons::icon_name;

use crate::{score_page::ScorePage, timer_page::TimerPage};

#[derive(Debug)]
pub enum Page {
    Score,
    Timer,
}

pub struct App {
    page: Page,
    score_page: Controller<ScorePage>,
    timer_page: Controller<TimerPage>,
}

#[derive(Debug)]
pub enum AppMsg {
    SetPage(String),
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = Page;
    type Input = AppMsg;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_default_width: 360,
            set_default_height: 760,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_hexpand: true,

                adw::HeaderBar {
                    #[wrap(Some)]
                    #[name="view_title"]
                    set_title_widget = &adw::ViewSwitcherTitle {
                        set_stack: Some(&stack),
                        set_title: "Score Tracker",
                    },
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_vexpand: true,

                    #[name="stack"]
                    adw::ViewStack {
                        connect_visible_child_notify[sender] => move |view_stack| {
                            if let Some(name) = view_stack.visible_child_name() {
                                sender.input(AppMsg::SetPage(name.to_string()))
                            }
                        },
                        set_vexpand: true,

                        add_titled[Some("score"), "Score"] = model.score_page.widget() {} -> {
                            set_icon_name: Some(icon_name::TABLE),
                        },
                        add_titled[Some("timer"), "Timer"] = model.timer_page.widget() {} -> {
                            set_icon_name: Some(icon_name::HOURGLASS),
                        },
                    },

                    #[name = "view_bar"]
                    adw::ViewSwitcherBar {
                        set_stack: Some(&stack),
                    }
                }
            },
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::SetPage(name) => {
                self.page = match name.as_str() {
                    "score" => Page::Score,
                    "timer" => Page::Timer,
                    _ => unreachable!(),
                }
            }
        }
    }

    // Initialize the component.
    fn init(
        page: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let score_page = ScorePage::builder()
            .launch(vec!["Player 1".to_string(), "Player 2".to_string()])
            .detach();

        let timer_page = TimerPage::builder()
            .launch(())
            .detach();

        let model = App { page, score_page, timer_page };

        // Insert the code generation of the view! macro here
        let widgets = view_output!();

        widgets
        .view_title
        .bind_property("title-visible", &widgets.view_bar, "reveal")
        .build();

        ComponentParts { model, widgets }
    }
}
