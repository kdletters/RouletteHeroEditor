mod bean;
mod font;
mod page;
mod page_render;

use page::*;

pub use eframe::egui::*;
pub use eframe::*;
pub use serde::{Deserialize, Serialize};

const APP_NAME: &str = "RouletteHeroEditor";
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_DATA_KEY: &str = "app_data";

const SIDEBAR_PAGES: &[Page; 5] = &[
    Page::Relic,
    Page::Element,
    Page::Enemy,
    Page::L10n,
    Page::RaceAttribute,
];

fn main() -> eframe::Result {
    let mut logger_builder = env_logger::builder();
    #[cfg(debug_assertions)]
    logger_builder.filter_level(log::LevelFilter::Debug);
    #[cfg(not(debug_assertions))]
    logger_builder.filter_level(log::LevelFilter::Error);

    logger_builder.build();
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = ViewportBuilder::default()
        .with_inner_size([1280.0, 720.0])
        .with_title(format!("{} - V{}", APP_NAME, APP_VERSION));

    eframe::run_native(
        APP_NAME,
        native_options,
        Box::new(|cc| {
            font::setup_custom_fonts(&cc.egui_ctx);
            let app_data = if let Some(storage) = cc.storage
                && let Some(app_data) = get_value::<AppData>(storage, APP_DATA_KEY)
            {
                app_data
            } else {
                AppData::default()
            };
            let mut app_state = AppState::default();
            app_state.ctx = cc.egui_ctx.clone();

            Ok(Box::new(MyApp {
                app_data,
                app_state,
            }))
        }),
    )?;

    Ok(())
}

struct MyApp {
    app_data: AppData,
    app_state: AppState,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct AppData {
    selected_page: Page,
    sidebar_collapsed: bool,
}

#[derive(Default)]
struct AppState {
    ctx: egui::Context,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render_sidebar(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_main_content(ui);
        });
    }

    fn save(&mut self, _storage: &mut dyn Storage) {
        set_value(_storage, APP_DATA_KEY, &self.app_data);
    }
}

impl MyApp {
    pub fn render_sidebar(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .show_animated(ctx, !self.app_data.sidebar_collapsed, |ui| {
                ui.set_width(200.0);
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    ui.heading(APP_NAME);
                });

                ui.add_space(20.0);

                ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                    for page in SIDEBAR_PAGES {
                        let info = page.get_info();
                        if ui
                            .selectable_label(
                                self.app_data.selected_page == *page,
                                format!("{}", info.title),
                            )
                            .clicked()
                        {
                            self.app_data.selected_page = page.clone();
                        }
                    }
                });
            });
    }

    // 渲染主内容区域
    pub fn render_main_content(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui
                .button(if self.app_data.sidebar_collapsed {
                    "⏵"
                } else {
                    "⏴"
                })
                .clicked()
            {
                self.app_data.sidebar_collapsed = !self.app_data.sidebar_collapsed;
            }

            // 添加主题切换按钮
            let theme_btn_res = ui.button(match self.app_state.ctx.theme() {
                Theme::Dark => "☀ Light",
                Theme::Light => "🌙 Dark",
            });
            if theme_btn_res.clicked() {
                match self.app_state.ctx.theme() {
                    Theme::Dark => self.app_state.ctx.set_theme(Theme::Light),
                    Theme::Light => self.app_state.ctx.set_theme(Theme::Dark),
                }
            }
        });

        // 在渲染页面时传递用户信息
        let page = self.app_data.selected_page.clone();
        page.render(ui, self);
    }
}
