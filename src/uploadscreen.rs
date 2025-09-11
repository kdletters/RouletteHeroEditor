use eframe::egui::*;
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};
// 注意：实际使用时需要根据Steam SDK的文档来导入正确的模块
// 这里假设Steam SDK已经通过steamworks crate集成
use steamworks::*;

const APP_ID: u32 = 3371510;

pub struct UploadScreen {
    // 是否为新MOD
    pub is_new_mod: bool,

    // MOD信息
    pub mod_name: String,
    pub mod_description: String,
    pub mod_id: String,
    pub thumbnail_path: String,
    pub folder_path: String,

    // UI状态
    pub show_confirm_dialog: bool,

    // 上传状态
    upload_progress: Arc<RwLock<UploadProgress>>,

    // 缩略图纹理（用于显示）
    thumbnail_texture: Option<TextureHandle>,

    // Steam SDK初始化状态
    steam_initialized: bool,
    steam_error: Option<String>,

    steam_client: Option<Client>,
}

struct UploadProgress {
    uploading: bool,
    progress: Option<UpdateWatchHandle>,
    status_text: String,
}

impl Default for UploadScreen {
    fn default() -> Self {
        Self {
            is_new_mod: true,
            mod_name: String::new(),
            mod_description: String::new(),
            mod_id: String::new(),
            thumbnail_path: String::new(),
            folder_path: String::new(),
            show_confirm_dialog: false,
            upload_progress: Arc::new(RwLock::new(UploadProgress {
                uploading: false,
                progress: None,
                status_text: "准备上传".to_string(),
            })),
            thumbnail_texture: None,
            steam_initialized: false,
            steam_error: None,
            steam_client: None,
        }
    }
}

impl UploadScreen {
    pub fn ui(&mut self, ctx: &Context) -> bool {
        let mut close_window = false;

        // 当界面第一次显示时初始化Steam SDK
        if !self.steam_initialized && self.steam_error.is_none() {
            self.initialize_steam();
        }

        // 创建模态窗口
        let modal = Modal::new("upload_screen_modal".into()).show(ctx, |ui| {
            ui.set_width(500.0);
            ui.set_height(400.0);

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.heading("《轮盘英雄》 创意工坊上传工具");

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.button("❌").clicked() {
                            close_window = true;
                        }
                    });
                });

                // 显示Steam状态
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Steam状态:").strong());
                    if self.steam_initialized {
                        ui.label(RichText::new("已连接").color(Color32::GREEN));
                    } else if let Some(error) = &self.steam_error {
                        ui.label(RichText::new(error).color(Color32::RED));
                    } else {
                        ui.label(RichText::new("初始化中...").color(Color32::YELLOW));
                    }
                });

                ui.separator();

                // MOD类型选择
                ui.horizontal(|ui| {
                    ui.label("MOD类型:");
                    ui.checkbox(&mut self.is_new_mod, "新MOD");
                    ui.add_space(20.0);
                    ui.label("MOD ID:");
                    ui.add_enabled_ui(!self.is_new_mod, |ui| {
                        ui.text_edit_singleline(&mut self.mod_id);
                    });
                });

                ui.separator();

                // MOD信息输入
                ui.horizontal(|ui| {
                    ui.label("MOD名称:");
                    ui.text_edit_singleline(&mut self.mod_name);
                });

                ui.horizontal(|ui| {
                    ui.label("MOD描述:");
                    ui.add_sized(
                        [ui.available_width(), 80.0],
                        TextEdit::multiline(&mut self.mod_description),
                    );
                });

                // 缩略图选择
                ui.horizontal(|ui| {
                    ui.label("缩略图:");
                    ui.text_edit_singleline(&mut self.thumbnail_path);
                    if ui.button("浏览...").clicked() {
                        self.browse_thumbnail(ui);
                    }
                });

                // 显示缩略图预览
                if let Some(texture) = &self.thumbnail_texture {
                    ui.horizontal(|ui| {
                        ui.label("预览:");
                        ui.add(Image::new(texture).max_width(100.0));
                    });
                }

                // 文件夹选择
                ui.horizontal(|ui| {
                    ui.label("MOD文件夹:");
                    ui.text_edit_singleline(&mut self.folder_path);
                    if ui.button("浏览...").clicked() {
                        self.browse_folder();
                    }
                });

                ui.separator();

                // 上传进度和状态
                let upload_progress = self.upload_progress.clone();
                ui.label(&upload_progress.read().unwrap().status_text);
                let mut progress = 0.0;
                let mut upload_message = "".to_string();
                if let Some(handle) = &upload_progress.read().unwrap().progress {
                    let (status, uploaded, total) = handle.progress();
                    progress = uploaded as f32 / total as f32;
                    upload_message = match status {
                        UpdateStatus::Invalid => "无效操作",
                        UpdateStatus::PreparingConfig => "准备配置中",
                        UpdateStatus::PreparingContent => "准备内容中",
                        UpdateStatus::UploadingContent => "上传内容中",
                        UpdateStatus::UploadingPreviewFile => "上传预览文件中",
                        UpdateStatus::CommittingChanges => "提交修改中",
                    }
                    .to_string();
                }
                ui.add(ProgressBar::new(progress));
                if progress > 0.0 {
                    ui.label(format!("{}: {}%", upload_message, progress * 100.0));
                }

                ui.separator();

                // 上传按钮 - 放在界面正下方
                ui.vertical_centered(|ui| {
                    ui.add_enabled_ui(self.steam_initialized, |ui| {
                        if ui.button("测试连接").clicked() {
                            let message = self.test_steam_connection();
                            // 显示测试结果消息（在实际应用中可能需要添加一个消息显示区域）
                            self.upload_progress.write().unwrap().status_text = message;
                        }

                        ui.add_space(10.0); // 添加一些间距

                        if ui.button("上传").clicked() {
                            self.validate_and_upload();
                        }
                    });
                });

                // 确认对话框
                if self.show_confirm_dialog {
                    self.show_confirmation_dialog(ui);
                }
            });
        });

        // 处理ESC键关闭
        if modal.should_close() {
            close_window = true;
        }

        close_window
    }

    fn initialize_steam(&mut self) {
        // 初始化Steam SDK
        // 注意：这需要在实际项目中集成steamworks crate并正确初始化

        match steamworks::Client::init_app(APP_ID) {
            Ok(client) => {
                // 初始化成功
                self.steam_initialized = true;
                self.upload_progress.write().unwrap().status_text =
                    "Steam初始化成功，准备上传".to_string();
                self.steam_client = Some(client.clone());
                // 可以启动一个线程来运行Steam回调
                std::thread::spawn(move || {
                    loop {
                        client.run_callbacks();
                        std::thread::sleep(std::time::Duration::from_millis(16));
                    }
                });
            }
            Err(e) => {
                // 初始化失败
                self.steam_error = Some("Steam初始化失败".to_string());
                self.upload_progress.write().unwrap().status_text =
                    format!("Steam初始化失败: {}", e);
            }
        }
    }

    fn browse_thumbnail(&mut self, ui: &mut Ui) {
        // 使用rfd打开文件选择对话框
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("图片文件", &["jpg", "jpeg", "png"])
            .set_title("选择模组缩略图")
            .pick_file()
        {
            self.thumbnail_path = path.display().to_string();
            // 加载缩略图纹理
            self.load_thumbnail_texture(ui);
        }
    }

    fn load_thumbnail_texture(&mut self, ui: &mut Ui) {
        // 尝试加载图片文件并创建纹理
        if let Ok(image_data) = std::fs::read(&self.thumbnail_path) {
            if let Ok(image) = ::image::load_from_memory(&image_data) {
                let rgba_image = image.to_rgba8();
                let pixels = rgba_image.as_raw();
                let image_buffer = ColorImage::from_rgba_unmultiplied(
                    [rgba_image.width() as usize, rgba_image.height() as usize],
                    pixels,
                );

                self.thumbnail_texture = Some(ui.ctx().load_texture(
                    "thumbnail_texture",
                    image_buffer,
                    TextureOptions::default(),
                ));
            }
        }
    }

    fn browse_folder(&mut self) {
        // 使用rfd打开文件夹选择对话框
        if let Some(path) = rfd::FileDialog::new()
            .set_title("选择模组文件夹")
            .pick_folder()
        {
            self.folder_path = path.display().to_string();
        }
    }

    fn validate_and_upload(&mut self) {
        // 验证是否有正在编辑的MOD
        if self.mod_name.trim().is_empty() || self.mod_description.trim().is_empty() {
            self.upload_progress.write().unwrap().status_text =
                "没有正在编辑的MOD可以上传".to_string();
            return;
        }

        // 验证输入
        if self.thumbnail_path.trim().is_empty() || !Path::new(&self.thumbnail_path).exists() {
            self.upload_progress.write().unwrap().status_text = "请选择有效的缩略图".to_string();
            return;
        }

        if self.folder_path.trim().is_empty() || !Path::new(&self.folder_path).exists() {
            self.upload_progress.write().unwrap().status_text = "请选择有效的MOD文件夹".to_string();
            return;
        }

        // 显示确认对话框
        self.show_confirm_dialog = true;
    }

    fn show_confirmation_dialog(&mut self, ui: &mut Ui) {
        // 创建确认对话框
        let modal = Modal::new("confirm_upload_dialog".into()).show(ui.ctx(), |ui| {
            ui.set_width(400.0);
            ui.set_height(300.0);

            ui.vertical(|ui| {
                ui.heading("确认上传");
                ui.separator();

                ScrollArea::vertical().show(ui, |ui| {
                    ui.label(format!("MOD名称: {}", self.mod_name));
                    ui.add_space(5.0);
                    ui.label(format!("MOD描述: {}", self.mod_description));
                    ui.add_space(5.0);
                    ui.label(format!("缩略图: {}", self.thumbnail_path));
                    ui.add_space(5.0);
                    ui.label(format!("MOD文件夹: {}", self.folder_path));
                });

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.button("取消").clicked() {
                            self.show_confirm_dialog = false;
                        }

                        if ui.button("确认上传").clicked() {
                            self.show_confirm_dialog = false;
                            self.upload_to_workshop();
                        }
                    });
                });
            });
        });

        // 处理ESC键关闭
        if modal.should_close() {
            self.show_confirm_dialog = false;
        }
    }

    fn upload_to_workshop(&mut self) {
        let upload_progress = self.upload_progress.clone();
        let mod_name = self.mod_name.clone();
        let mod_description = self.mod_description.clone();
        let mod_id = self.mod_id.clone();
        let thumbnail_path = self.thumbnail_path.clone();
        let folder_path = self.folder_path.clone();
        // 示例代码（需要根据实际Steam SDK调整）：
        if let Some(client) = self.steam_client.clone() {
            client.ugc().create_item(
                APP_ID.into(),
                FileType::Community,
                move |result| match result {
                    Ok((file_id, _)) => {
                        // 第二个参数是用户需要接受《Steam 创意工坊法律协议》（true），还是不需要（false）？ 参见 创意工坊法律协议，了解更多信息。
                        let upload_progress2 = upload_progress.clone();
                        upload_progress.write().unwrap().status_text =
                            "创建创意工坊物品成功,开始上传".to_string();
                        let handle = client.ugc().start_item_update(APP_ID.into(), file_id);
                        let watch_handle = handle
                            .title(&mod_name)
                            .description(&mod_description)
                            .metadata(&format!("{{mod_id = {}}}", mod_id))
                            .content_path(Path::new(&folder_path))
                            .preview_path(Path::new(&thumbnail_path))
                            .submit(None, move |result| match result {
                                Ok((file_id, _)) => {
                                    // 第二个参数是用户需要接受《Steam 创意工坊法律协议》（true），还是不需要（false）？ 参见 创意工坊法律协议，了解更多信息。
                                    upload_progress2.write().unwrap().status_text =
                                        "上传创意工坊物品成功".to_string();
                                    client.friends().activate_game_overlay_to_web_page(&format!(
                                        "steam://url/CommunityFilePage/{}",
                                        file_id.0
                                    ));
                                }
                                Err(e) => {
                                    upload_progress2.write().unwrap().status_text =
                                        format!("上传失败: {}", e);
                                }
                            });
                        upload_progress.write().unwrap().progress = Some(watch_handle);
                    }
                    Err(e) => {
                        upload_progress.write().unwrap().status_text =
                            format!("创建创意工坊物品失败: {}", e);
                    }
                },
            );

            self.upload_progress.write().unwrap().status_text =
                "上传请求已提交，处理中...".to_string();
        }
    }

    // 添加测试连接的方法
    fn test_steam_connection(&self) -> String {
        // 测试Steam连接状态
        // 注意：这需要在实际项目中集成steamworks crate并正确实现

        if self.steam_initialized
            && let Some(client) = &self.steam_client
        {
            // 获取用户名等信息
            let friends = client.friends();
            let username = friends.name();
            format!("Steam 已登录，当前用户: {}", username)
        } else {
            "Steam 未运行！".to_string()
        }
    }

    pub fn reset(&mut self) {
        self.is_new_mod = true;
        self.mod_name.clear();
        self.mod_description.clear();
        self.mod_id.clear();
        self.thumbnail_path.clear();
        self.folder_path.clear();
        self.show_confirm_dialog = false;
        let mut upload_progress = self.upload_progress.write().unwrap();
        upload_progress.uploading = false;
        upload_progress.progress = None;
        upload_progress.status_text = "准备上传".to_string();
        self.thumbnail_texture = None;
        // 重置Steam状态
        self.steam_initialized = false;
        self.steam_error = None;
    }
}
