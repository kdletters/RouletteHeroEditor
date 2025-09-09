use super::*;
use crate::page_render::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum Page {
    #[default]
    Relic,
    Element,
    Enemy,
    L10n,
    RaceAttribute,
}

#[derive(Debug, Clone)]
pub struct PageInfo {
    pub title: &'static str,
}

impl Page {
    pub fn get_info(&self) -> PageInfo {
        match self {
            Page::Relic => PageInfo { title: "遗物" },
            Page::Element => PageInfo { title: "单位" },
            Page::Enemy => PageInfo { title: "敌人" },
            Page::L10n => PageInfo { title: "本地化" },
            Page::RaceAttribute => PageInfo { title: "种族" },
        }
    }

    pub fn render(&self, ui: &mut egui::Ui, app: &mut MyApp) {
        ui.heading(self.get_info().title);
        ui.separator();
        match self {
            Page::Relic => {
                ui.label("你好");
            }
            Page::Element => {
                ui.label("你好");
            }
            Page::Enemy => {
                ui.label("你好");
            }
            Page::L10n => {
                ui.label("你好");
            }
            Page::RaceAttribute => {
                render_race_attribute(ui, app);
            }
        }
    }
}
