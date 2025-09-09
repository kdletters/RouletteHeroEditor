use crate::bean::*;
use crate::*;
use egui_extras::{Column, TableBuilder};

pub fn render_race_attribute(ui: &mut egui::Ui, app: &mut MyApp) {
    TopBottomPanel::bottom("race_attribute_bottom").show_inside(ui, |ui| {
        if ui.button("+").clicked() {
            app.app_state
                .table_data
                .race_attributes
                .push(RaceAttribute::default());

            app.app_state.scroll_to_row = Some(0);
        }
    });

    CentralPanel::default().show_inside(ui, |ui| {
        egui::ScrollArea::horizontal().show(ui, |ui| {
            let mut table = TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::remainder())
                .column(Column::remainder())
                .column(Column::remainder())
                .column(Column::remainder())
                .column(Column::remainder());

            if let Some(row_index) = app.app_state.scroll_to_row.take() {
                table = table.scroll_to_row(row_index, None);
            }

            let mut need_delete = vec![];
            table
                .header(20., |mut header| {
                    header.col(|_ui| {});
                    header.col(|ui| {
                        ui.strong("id");
                    });
                    header.col(|ui| {
                        ui.strong("icon");
                    });
                    header.col(|ui| {
                        ui.strong("name_id");
                    });
                    header.col(|ui| {
                        ui.strong("desc_id");
                    });
                })
                .body(|mut body| {
                    for (index, race_attribute) in app
                        .app_state
                        .table_data
                        .race_attributes
                        .iter_mut()
                        .enumerate()
                    {
                        body.row(20., |mut row| {
                            row.col(|ui| {
                                if ui.button("删除").clicked() {
                                    need_delete.push(index);
                                }
                            });
                            row.col(|ui| {
                                ui.add(
                                    DragValue::new(&mut race_attribute.id)
                                        .speed(0)
                                        .update_while_editing(false),
                                );
                            });
                            row.col(|ui| {
                                ui.text_edit_singleline(&mut race_attribute.icon);
                            });
                            row.col(|ui| {
                                ui.add(
                                    DragValue::new(&mut race_attribute.name_id)
                                        .speed(0)
                                        .update_while_editing(false),
                                );
                            });
                            row.col(|ui| {
                                ui.add(
                                    DragValue::new(&mut race_attribute.desc_id)
                                        .speed(0)
                                        .update_while_editing(false),
                                );
                            });
                        });
                    }
                });

            for index in need_delete {
                app.app_state.table_data.race_attributes.remove(index);
            }
            app.app_state.table_data.race_attributes.sort();
        });
    });
}
