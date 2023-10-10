use std::{str::FromStr, time::Duration};

use adw::prelude::*;
use relm4::prelude::*;
use relm4_icons::icon_name;

use crate::{fl, score_page::ScorePage, timer_page::TimerPage};

const INITIAL_PLAYERS: usize = 2;
const INITIAL_SCORE_ROWS: usize = 1;

#[derive(Debug)]
pub enum Page {
    Score,
    Timer,
}

impl Page {
    pub fn to_translated_string(&self) -> String {
        match self {
            Page::Score => fl!("score").to_owned(),
            Page::Timer => fl!("timer").to_owned(),
        }
    }
}

impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Page::Score => write!(f, "score"),
            Page::Timer => write!(f, "timer"),
        }
    }
}

impl FromStr for Page {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "score" => Ok(Page::Score),
            "timer" => Ok(Page::Timer),
            _ => Err(anyhow::anyhow!("Invalid page name")),
        }
    }
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
                        #[watch]
                        set_title: &model.page.to_translated_string(),
                        #[chain(build())]
                        bind_property: ("title-visible", &view_bar, "reveal"),
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

                        add_titled[Some("score"), fl!("score")] = model.score_page.widget() {} -> {
                            set_icon_name: Some(icon_name::TABLE),
                        },
                        add_titled[Some("timer"), fl!("timer")] = model.timer_page.widget() {} -> {
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
                self.page = Page::from_str(name.as_str()).unwrap();
            }
        }
    }

    fn init(
        page: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let score_page = ScorePage::builder()
            .launch((INITIAL_PLAYERS, INITIAL_SCORE_ROWS))
            .detach();

        let timer_page = TimerPage::builder().launch(Duration::from_secs(0)).detach();

        let model = App {
            page,
            score_page,
            timer_page,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
