mod bean;
mod data;
mod font;
mod page;
mod page_render;
mod uploadscreen;
mod welcome_screen;

use data::*;
use page::*;
use std::collections::VecDeque;
use std::fs;
use std::path::Path;
use welcome_screen::render_welcome_screen;

use crate::bean::ModData;
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

#[derive(Debug, PartialEq, Clone)]
enum ShowState {
    None,
    Welcome,
    Mod,
    Upload,
}

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
                show_state: ShowState::Welcome,
                upload_screen: uploadscreen::UploadScreen::default(),
            }))
        }),
    )?;

    Ok(())
}

struct MyApp {
    app_data: AppData,
    app_state: AppState,
    show_state: ShowState,
    upload_screen: uploadscreen::UploadScreen,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
struct AppData {
    selected_page: Option<Page>,
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
    show_confirmation_dialog: bool,
    allowed_to_close: bool,
    scroll_to_row: Option<usize>,

    mod_creation_data: ModCreationData, // 添加MOD对话框相关状态
    show_create_dialog: bool,           // 显示创建MOD的对话框
}

#[derive(Default)]
struct ModCreationData {
    mod_name: String,
    mod_description: String,
    modify_units: bool,
    modify_tapes: bool,
    modify_enemies: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render_sidebar(ctx);
        self.render_create_mod_dialog();

        // 渲染上传界面弹窗
        if self.show_state == ShowState::Upload {
            if self.upload_screen.ui(ctx) {
                self.show_state = ShowState::None;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_main_content(ui);
        });

        // 处理关闭事件
        if ctx.input(|i| i.viewport().close_requested()) {
            if self.app_state.allowed_to_close {
                // 允许关闭，不需要做任何事
            } else {
                // 取消关闭并显示确认对话框
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                self.app_state.show_confirmation_dialog = true;
            }
        }

        // 显示确认对话框
        if self.app_state.show_confirmation_dialog {
            let modal = egui::Modal::new("exit_confirmation".into()).show(ctx, |ui| {
                ui.set_width(300.0);

                ui.horizontal(|ui| {
                    ui.heading("是否保存数据?");

                    ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                        if ui.button("x").clicked() {
                            self.app_state.show_confirmation_dialog = false;
                            self.app_state.allowed_to_close = false;
                        }
                    })
                });

                ui.add_space(32.0);

                let mut show_confirmation_dialog = self.app_state.show_confirmation_dialog;
                let mut allowed_to_close = self.app_state.allowed_to_close;
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
                            self.app_state.show_confirmation_dialog = false;
                            self.app_state.allowed_to_close = true;
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    },
                );

                if changed {
                    self.app_state.show_confirmation_dialog = show_confirmation_dialog;
                    self.app_state.allowed_to_close = allowed_to_close;
                }
            });

            // 防止用户通过ESC键关闭模态窗口
            if modal.should_close() {
                self.app_state.show_confirmation_dialog = false;
                self.app_state.allowed_to_close = false;
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
                TopBottomPanel::top("top_panel").show_inside(ui, |ui| {
                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        ui.add_space(10.0);
                        ui.heading(APP_NAME);
                    });
                    ui.add_space(20.0);
                });

                CentralPanel::default().show_inside(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                        for page in SIDEBAR_PAGES {
                            let info = page.get_info();
                            if ui
                                .selectable_label(
                                    self.app_data.selected_page == Some(page.clone()),
                                    format!("{}", info.title),
                                )
                                .clicked()
                            {
                                self.app_data.selected_page = Some(page.clone());
                            }
                        }
                    });
                });

                TopBottomPanel::bottom("bottom_panel").show_inside(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                        ui.add_space(10.0); // 添加一些间距

                        // 第一个按钮
                        if ui
                            .add_sized([150.0, 30.0], egui::Button::new("创建MOD"))
                            .clicked()
                        {
                            // 在这里添加按钮1的点击逻辑
                            self.add_mod(ui);
                        }

                        // 第二个按钮
                        if ui
                            .add_sized([150.0, 30.0], egui::Button::new("上传MOD"))
                            .clicked()
                        {
                            // 检查是否有正在编辑的MOD
                            // 这里应该检查当前是否有正在编辑的MOD
                            // 现在使用一个简单的检查作为示例
                            let has_editing_mod = !self.app_data.workspaces.is_empty(); // 简单检查是否有工作区

                            if has_editing_mod {
                                // 在这里添加按钮2的点击逻辑
                                self.show_state = ShowState::Upload;
                                // 初始化上传界面数据
                                self.upload_screen.reset();
                                if let Some(workspace_path) = self.get_current_workspace() {
                                    self.upload_screen.folder_path = workspace_path.to_string();
                                }
                                self.upload_screen.mod_name = "当前MOD名称".to_string(); // 需要从当前编辑的MOD获取
                                self.upload_screen.mod_description = "当前MOD描述".to_string();
                            // 需要从当前编辑的MOD获取
                            } else {
                                // 显示提示信息
                                self.app_state
                                    .messages
                                    .push_back("没有正在编辑的MOD".to_string());
                                self.app_state.current_message =
                                    self.app_state.messages.pop_front();
                            }
                        }
                        ui.add_space(10.0); // 添加一些间距
                    });
                });
            });
    }

    // 渲染主内容区域
    pub fn render_main_content(&mut self, ui: &mut egui::Ui) {
        self.render_top_bar(ui);

        if let Some(page) = self.app_data.selected_page.clone() {
            page.render(ui, self);
        } else {
            render_welcome_screen(self, ui);
        }
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

    pub fn add_mod(&mut self, _ui: &mut egui::Ui) {
        // 重置表单数据
        self.app_state.mod_creation_data = ModCreationData {
            mod_name: String::new(),
            mod_description: String::new(),
            modify_units: true,
            modify_tapes: false,
            modify_enemies: false,
        };

        // 设置显示对话框标志
        self.app_state.show_create_dialog = true;
    }

    pub fn render_create_mod_dialog(&mut self) {
        if !self.app_state.show_create_dialog {
            return;
        }

        // 克隆需要在闭包中使用的数据，避免多重借用
        let mod_name = self.app_state.mod_creation_data.mod_name.clone();
        let mod_description = self.app_state.mod_creation_data.mod_description.clone();
        let modify_units = self.app_state.mod_creation_data.modify_units;
        let modify_tapes = self.app_state.mod_creation_data.modify_tapes;
        let modify_enemies = self.app_state.mod_creation_data.modify_enemies;

        let ctx = self.app_state.ctx.clone();

        let modal = egui::Modal::new("create_mod_dialog".into()).show(&ctx, |ui| {
            ui.set_width(400.0);

            ui.vertical(|ui| {
                ui.heading("创建新MOD");
                ui.separator();

                // MOD名字输入
                ui.horizontal(|ui| {
                    ui.label("MOD名称:");
                    let mut mod_name_clone = mod_name.clone();
                    ui.text_edit_singleline(&mut mod_name_clone);
                    // 更新原始数据
                    if mod_name != mod_name_clone {
                        self.app_state.mod_creation_data.mod_name = mod_name_clone;
                    }
                });

                // MOD描述输入
                ui.horizontal(|ui| {
                    ui.label("MOD描述:");
                    let mut mod_description_clone = mod_description.clone();
                    ui.add_sized(
                        [ui.available_width(), 80.0],
                        egui::TextEdit::multiline(&mut mod_description_clone),
                    );
                    // 更新原始数据
                    if mod_description != mod_description_clone {
                        self.app_state.mod_creation_data.mod_description = mod_description_clone;
                    }
                });

                ui.add_space(10.0);

                // Checkbox选项
                let mut modify_units_clone = modify_units;
                let mut modify_tapes_clone = modify_tapes;
                let mut modify_enemies_clone = modify_enemies;
                ui.checkbox(&mut modify_units_clone, "修改单位");
                ui.checkbox(&mut modify_tapes_clone, "修改卡带");
                ui.checkbox(&mut modify_enemies_clone, "修改敌人");

                // 更新原始数据
                if modify_units != modify_units_clone {
                    self.app_state.mod_creation_data.modify_units = modify_units_clone;
                }
                if modify_tapes != modify_tapes_clone {
                    self.app_state.mod_creation_data.modify_tapes = modify_tapes_clone;
                }
                if modify_enemies != modify_enemies_clone {
                    self.app_state.mod_creation_data.modify_enemies = modify_enemies_clone;
                }

                ui.add_space(20.0);

                // 按钮区域
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // 取消按钮
                        if ui.button("取消").clicked() {
                            self.app_state.show_create_dialog = false;
                        }

                        ui.add_space(10.0);

                        // 确认按钮
                        if ui.button("确认").clicked() {
                            // 验证输入
                            if mod_name.trim().is_empty() {
                                self.app_state
                                    .messages
                                    .push_back("MOD名称不能为空".to_string());
                                self.app_state.current_message =
                                    self.app_state.messages.pop_front();
                            } else if mod_description.trim().is_empty() {
                                self.app_state
                                    .messages
                                    .push_back("MOD描述不能为空".to_string());
                                self.app_state.current_message =
                                    self.app_state.messages.pop_front();
                            } else if !modify_units_clone
                                && !modify_tapes_clone
                                && !modify_enemies_clone
                            {
                                self.app_state
                                    .messages
                                    .push_back("至少选择一个修改选项".to_string());
                                self.app_state.current_message =
                                    self.app_state.messages.pop_front();
                            } else {
                                // 创建文件
                                // 获取当前工作区路径
                                if let Some(workspace_path) = self.get_current_workspace() {
                                    // 创建MOD根目录在与ModTool同级的ModDebug文件夹下
                                    let mod_debug_path = Path::new(workspace_path)
                                        .parent()
                                        .unwrap()
                                        .join("ModDebug");
                                    let mod_root_path = mod_debug_path.join(&mod_name);

                                    // 创建目录结构
                                    match fs::create_dir_all(&mod_root_path) {
                                        Ok(_) => {
                                            // 创建子目录
                                            let config_mod_path = mod_root_path.join("Config_Mod");
                                            let image_mod_path = mod_root_path.join("Image_Mod");
                                            let sound_mod_path = mod_root_path.join("Sound_Mod");

                                            let _ = fs::create_dir_all(&config_mod_path);
                                            let _ = fs::create_dir_all(&image_mod_path);
                                            let _ = fs::create_dir_all(&sound_mod_path);

                                            // 创建 moddata.json 文件
                                            let mod_data = ModData {
                                                name: mod_name.clone(),
                                                desc: mod_description.clone(),
                                                entity: modify_units_clone,
                                                enemy: modify_enemies_clone,
                                                version: "1.0.0".to_string(),
                                                card: modify_tapes_clone,
                                            };

                                            let mod_data_path = mod_root_path.join("moddata.json");
                                            match serde_json::to_string_pretty(&mod_data) {
                                                Ok(json_content) => {
                                                    match fs::write(&mod_data_path, json_content) {
                                                        Ok(_) => {
                                                            self.app_state.messages.push_back(
                                                                format!(
                                                                    "创建MOD: {} 成功",
                                                                    mod_name
                                                                ),
                                                            );
                                                            self.app_state.current_message =
                                                                self.app_state.messages.pop_front();
                                                            self.app_state.show_create_dialog =
                                                                false;
                                                        }
                                                        Err(e) => {
                                                            self.app_state.messages.push_back(
                                                                format!(
                                                                    "创建moddata.json文件失败: {}",
                                                                    e
                                                                ),
                                                            );
                                                            self.app_state.current_message =
                                                                self.app_state.messages.pop_front();
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    self.app_state.messages.push_back(format!(
                                                        "序列化MOD数据失败: {}",
                                                        e
                                                    ));
                                                    self.app_state.current_message =
                                                        self.app_state.messages.pop_front();
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            self.app_state
                                                .messages
                                                .push_back(format!("创建MOD目录失败: {}", e));
                                            self.app_state.current_message =
                                                self.app_state.messages.pop_front();
                                        }
                                    }
                                } else {
                                    self.app_state
                                        .messages
                                        .push_back("请先选择工作区".to_string());
                                    self.app_state.current_message =
                                        self.app_state.messages.pop_front();
                                }
                            }
                        }
                    });
                });
            });

            // 注意：由于modal变量在闭包内部创建，我们无法在这里直接访问它
            // ESC键关闭模态窗口的功能需要在闭包内部处理
        });
    }
}
