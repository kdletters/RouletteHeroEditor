use crate::MyApp;
use crate::bean::*;

#[derive(Default)]
pub struct TableData {
    pub relics: Vec<Relics>,
    pub elements: Vec<Element>,
    pub enemies: Vec<Enemy>,
    pub l10n: Vec<Localization>,
    pub race_attributes: Vec<RaceAttribute>,
}

/// 将json数据读取到编辑器没
pub fn read_data(app: &mut MyApp) {}

/// 将编辑器数据保存为json
pub fn save_data(app: &mut MyApp) {}

/// 清除编辑器内数据
pub fn clear_data(app: &mut MyApp) {}
