
use crate::{MyApp, egui, APP_NAME, Layout, Direction, TopBottomPanel, CentralPanel, ComboBox, Id};
use crate::page::Page;


pub fn render_mod_screen(app: &mut MyApp, ctx: &egui::Context,ui: &mut egui::Ui) {
    
    // 在渲染页面时传递用户信息
    let page = app.app_data.selected_page.clone();
    page.render(ui, app);

}