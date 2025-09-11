use crate::{MyApp, egui};

pub fn render_welcome_screen(app: &mut MyApp, ui: &mut egui::Ui) {
    // 获取可用空间并居中显示
    let available_size = ui.available_size();
    let content_width = 600.0;
    let content_height = 200.0;
    
    // 计算居中位置
    let margin_x = (available_size.x - content_width) / 2.0;
    let margin_y = (available_size.y - content_height) / 2.0;
    
    ui.add_space(margin_y);
    
    ui.vertical_centered(|ui| {
        ui.set_width(content_width);
        
        // 标题
        ui.label(egui::RichText::new("欢迎使用《轮盘英雄》MOD工具").size(24.0));
        ui.add_space(20.0);
        ui.label( egui::RichText::new("请选择或创建mod").size(16.0));

    });
}
