mod bean;
mod data;
mod font;
mod page;
mod page_render;

use data::*;
use page::*;
use std::collections::VecDeque;

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
                show_confirmation_dialog: false,
                allowed_to_close: false,
            }))
        }),
    )?;

    Ok(())
}

struct MyApp {
    app_data: AppData,
    app_state: AppState,
    show_confirmation_dialog: bool,
    allowed_to_close: bool,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct AppData {
    selected_page: Page,
    sidebar_collapsed: bool,
    workspaces: Vec<String>,
    current_workspace: usize,
}

#[derive(Default)]
struct AppState {
    ctx: egui::Context,
    messages: VecDeque<String>,
    current_message: Option<String>,

    table_data: TableData,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render_sidebar(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_main_content(ui);
        });

        // 处理关闭事件
        if ctx.input(|i| i.viewport().close_requested()) {
            if self.allowed_to_close {
                // 允许关闭，不需要做任何事
            } else {
                // 取消关闭并显示确认对话框
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                self.show_confirmation_dialog = true;
            }
        }

        // 显示确认对话框
        if self.show_confirmation_dialog {
            let modal = egui::Modal::new("exit_confirmation".into()).show(ctx, |ui| {
                ui.set_width(300.0);

                ui.horizontal(|ui| {
                    ui.heading("是否保存数据?");

                    ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                        if ui.button("x").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = false;
                        }
                    })
                });

                ui.add_space(32.0);

                let mut show_confirmation_dialog = self.show_confirmation_dialog;
                let mut allowed_to_close = self.allowed_to_close;
                let mut changed = false;
                Sides::default().show(
                    ui,
                    |ui| {
                        if ui.button("不保存").clicked() {
                            show_confirmation_dialog = false;
                            allowed_to_close = true;
                            changed = true;
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    },
                    |ui| {
                        if ui.button("保存并退出").clicked() {
                            save_data(self);
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = true;
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    },
                );

                if changed {
                    self.show_confirmation_dialog = show_confirmation_dialog;
                    self.allowed_to_close = allowed_to_close;
                }
            });

            // 防止用户通过ESC键关闭模态窗口
            if modal.should_close() {
                self.show_confirmation_dialog = false;
                self.allowed_to_close = false;
            }
        }
    }

    fn save(&mut self, _storage: &mut dyn Storage) {
        set_value(_storage, APP_DATA_KEY, &self.app_data);
    }
}

impl MyApp {
    pub fn render_sidebar(&mut self, ctx: &egui::Context) {
        self.render_message();
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
        self.render_top_bar(ui);

        // 在渲染页面时传递用户信息
        let page = self.app_data.selected_page.clone();
        page.render(ui, self);
    }

    fn render_top_bar(&mut self, ui: &mut egui::Ui) {
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

            ui.separator();

            self.render_workspace_state(ui);

            ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                if ui.button("加载数据").clicked() {
                    read_data(self);
                }
                if ui.button("保存数据").clicked() {
                    save_data(self);
                }
                if ui.button("清除数据").clicked() {
                    clear_data(self);
                }
            });
        });
    }

    fn render_message(&mut self) {
        if self.app_state.current_message.is_none() {
            self.app_state.current_message = self.app_state.messages.pop_front();
        }
        if let Some(message) = &self.app_state.current_message {
            let modal = egui::Modal::new("message".into()).show(&self.app_state.ctx, |ui| {
                ui.with_layout(
                    Layout::centered_and_justified(Direction::LeftToRight),
                    |ui| {
                        ui.vertical_centered(|ui| {
                            TopBottomPanel::top("message_top").show_inside(ui, |ui| {
                                ui.with_layout(
                                    Layout::centered_and_justified(Direction::LeftToRight),
                                    |ui| {
                                        ui.label("通知");
                                    },
                                );
                            });
                            CentralPanel::default().show_inside(ui, |ui| {
                                ui.with_layout(
                                    Layout::centered_and_justified(Direction::LeftToRight),
                                    |ui| {
                                        ui.label(message);
                                    },
                                );
                            });
                            TopBottomPanel::bottom("message_bottom").show_inside(ui, |ui| {
                                ui.with_layout(
                                    Layout::centered_and_justified(Direction::LeftToRight),
                                    |ui| {
                                        if ui.button("关闭").clicked() {
                                            ui.close();
                                        }
                                    },
                                );
                            });
                        });
                    },
                );
            });
            if modal.should_close() {
                self.app_state.current_message = None;
            }
        }
    }

    fn render_workspace_state(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("工作区：");
            ComboBox::new(Id::new("workspace_path"), "")
                .selected_text(if self.app_data.workspaces.is_empty() {
                    "请添加工作区"
                } else {
                    &self.app_data.workspaces[self.app_data.current_workspace]
                })
                .show_ui(ui, |ui| {
                    for (i, path) in self.app_data.workspaces.clone().iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut self.app_data.current_workspace, i, path);
                            // 将按钮移到行的最右边
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.button("-").clicked() {
                                        self.remove_workspace(i);
                                    }
                                },
                            );
                        });
                    }
                });
        });
        if ui.button("+").clicked() {
            self.add_workspace();
        }
    }

    fn add_workspace(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .set_title("选择ModTool文件夹")
            .add_filter("ModTool", &[""])
            .pick_folder()
        {
            if path.file_name().unwrap().ne("ModTool") {
                self.app_state
                    .messages
                    .push_back("请选择ModTool文件夹".to_string());
                return;
            }

            let path_str = path.display().to_string();
            // 检查是否已存在该工作区
            if !self.app_data.workspaces.contains(&path_str) {
                self.app_data.workspaces.push(path_str);
                // 选中新添加的工作区
                self.app_data.current_workspace = self.app_data.workspaces.len() - 1;
            } else {
                self.app_state
                    .messages
                    .push_back("工作区已存在".to_string());
            }
        }
    }

    // 移除当前选中的工作区
    fn remove_workspace(&mut self, index: usize) {
        if !self.app_data.workspaces.is_empty() {
            // 移除当前选中的工作区
            self.app_data.workspaces.remove(index);

            // 调整当前选中索引
            if self.app_data.current_workspace >= self.app_data.workspaces.len()
                && !self.app_data.workspaces.is_empty()
            {
                self.app_data.current_workspace = self.app_data.workspaces.len() - 1;
            } else if self.app_data.workspaces.is_empty() {
                self.app_data.current_workspace = 0;
            }
        }
    }

    pub fn get_current_workspace(&self) -> Option<&str> {
        self.app_data
            .workspaces
            .get(self.app_data.current_workspace)
            .map(|s| s.as_str())
    }
}
