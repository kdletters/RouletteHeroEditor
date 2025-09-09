use super::*;

// 设置自定义字体以支持中文显示
pub fn setup_custom_fonts(ctx: &egui::Context) {
    // 从默认字体开始（我们将添加而不是替换它们）
    let mut fonts = egui::FontDefinitions::default();

    // 尝试加载系统微软雅黑字体文件
    let font_data = load_system_font_data("msyh.ttc")
        .or_else(|| load_system_font_data("msyh.ttf"))
        .or_else(|| load_system_font_data("simsun.ttc"))
        .unwrap();

    fonts.font_data.insert(
        "system_font".to_owned(),
        std::sync::Arc::new(egui::FontData::from_owned(font_data)),
    );

    // 将自定义字体添加到比例字体族中（用于用户界面）
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "system_font".to_owned());

    // 将自定义字体添加到等宽字体族中（用于代码）
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .insert(0, "system_font".to_owned());

    // 应用字体设置
    ctx.set_fonts(fonts);
}


// 加载系统字体数据
fn load_system_font_data(font_file: &str) -> Option<Vec<u8>> {
    // Windows字体路径
    let windows_font_path = format!("C:\\Windows\\Fonts\\{}", font_file);
    if std::path::Path::new(&windows_font_path).exists() {
        return std::fs::read(windows_font_path).ok();
    }
    None
}