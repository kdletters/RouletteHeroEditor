use crate::MyApp;
use crate::bean::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use crate::luban::load_tables;

#[derive(Default)]
pub struct TableData {
    pub relics: Vec<Relics>,
    pub elements: Vec<Element>,
    pub enemies: Vec<Enemy>,
    pub l10n: Vec<Localization>,
    pub race_attributes: Vec<RaceAttribute>,
}

fn get_data_path(app: &mut MyApp) -> Option<PathBuf> {
    app.get_current_workspace().and_then(|workspace| {
        let path = Path::new(workspace)
            .join("Config")
            .join("Config")
            .join("Datas");
        if path.exists() { Some(path) } else { None }
    })
}

/// 将json数据读取到编辑器没
pub fn read_data(app: &mut MyApp) {
    if let Some(path) = get_data_path(app) {
        read_luban_bytes(path);
        /*
        read_json(
            path.join("element_mod.json"),
            &mut app.app_state.table_data.elements,
            &mut app.app_state.messages,
        );
        read_json(
            path.join("localization_mod.json"),
            &mut app.app_state.table_data.l10n,
            &mut app.app_state.messages,
        );
        read_json(
            path.join("raceattribute_mod.json"),
            &mut app.app_state.table_data.race_attributes,
            &mut app.app_state.messages,
        );
        read_json(
            path.join("enemy_mod.json"),
            &mut app.app_state.table_data.enemies,
            &mut app.app_state.messages,
        );
        read_json(
            path.join("relics_mod.json"),
            &mut app.app_state.table_data.relics,
            &mut app.app_state.messages,
        );
        
         */
    }
}

/// 将编辑器数据保存为json
pub fn save_data(app: &mut MyApp) {
    if let Some(path) = get_data_path(app) {
        write_json(
            path.join("element_mod.json"),
            &mut app.app_state.table_data.elements,
            &mut app.app_state.messages,
        );
        write_json(
            path.join("localization_mod.json"),
            &mut app.app_state.table_data.l10n,
            &mut app.app_state.messages,
        );
        write_json(
            path.join("raceattribute_mod.json"),
            &mut app.app_state.table_data.race_attributes,
            &mut app.app_state.messages,
        );
        write_json(
            path.join("enemy_mod.json"),
            &mut app.app_state.table_data.enemies,
            &mut app.app_state.messages,
        );
        write_json(
            path.join("relics_mod.json"),
            &mut app.app_state.table_data.relics,
            &mut app.app_state.messages,
        );
    }
}

/// 清除编辑器内数据
pub fn clear_data(app: &mut MyApp) {
    app.app_state.table_data = Default::default();
}

fn read_json<T>(path: PathBuf, vec: &mut Vec<T>, messages: &mut VecDeque<String>)
where
    T: for<'de> Deserialize<'de>,
{
    if path.exists() {
        match std::fs::read_to_string(path) {
            Ok(json) => match serde_json::from_str::<Vec<T>>(&json) {
                Ok(data) => *vec = data,
                Err(e) => messages.push_back(format!("{}", e)),
            },
            Err(e) => messages.push_back(format!("{}", e)),
        }
    }
    else {

    }
}

fn write_json<T>(path: PathBuf, vec: &mut Vec<T>, messages: &mut VecDeque<String>)
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    let mut file = match std::fs::File::create(path) {
        Ok(file) => file,
        Err(e) => {
            messages.push_back(format!("{}", e));
            return;
        }
    };
    if let Err(e) = serde_json::to_writer_pretty(&mut file, vec) {
        messages.push_back(format!("{}", e));
    }
}

fn read_luban_bytes(path: PathBuf) {
    let table_path = path.join("Config_Mod");
    let tables = load_tables(table_path.to_str().unwrap());
    let _relics = tables.tbrelics;
    let _element = tables.tbelement;
    let _enemy = tables.tbenemy;
    let _race  = tables.tbraceattribute;
    let _l10n = tables.tblocalization;
}