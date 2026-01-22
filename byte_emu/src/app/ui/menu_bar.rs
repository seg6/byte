use eframe::egui;
use crate::app::{ByteEmuApp, FileProcessorMessage, State};

impl ByteEmuApp {
    pub fn show_menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.ui_menu_bar(ui);
        });
    }

    fn ui_menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::MenuBar::new().ui(ui, |ui| {
            self.ui_file_button(ui);
            self.ui_tools_button(ui);

            if ui.button("About").clicked() {
                self.state.is_about_open = !self.state.is_about_open;
            }
        });
    }

    fn ui_file_button(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("File", |ui| {
            use FileProcessorMessage::*;

            if ui.button("Load binary program").clicked() {
                self.file_processor
                    .read(|name, data| BinaryFile((name, data)));
                ui.close();
            }
            if ui.button("Load source file").clicked() {
                self.file_processor
                    .read(|name, data| SourceFile((name, data)));
                ui.close();
            }

            ui.separator();

            if ui.button("Reset GUI state").clicked() {
                ui.ctx().memory_mut(|mem| *mem = Default::default());
                ui.close();
            }
            if ui.button("Reset everything").clicked() {
                self.state = State::default();
                ui.ctx().memory_mut(|mem| *mem = Default::default());
                ui.close();
            }
        });
    }

    fn ui_tools_button(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Tools", |ui| {
            if ui.button("Code Editor").clicked() {
                self.state.is_code_editor_open = !self.state.is_code_editor_open;
                ui.close();
            }

            if ui.button("Emulator Controls").clicked() {
                self.state.is_emu_controls_open = !self.state.is_emu_controls_open;
                ui.close();
            }

            if ui.button("Memory Monitor").clicked() {
                self.state.is_memory_monitor_open = !self.state.is_memory_monitor_open;
                ui.close();
            }
        });
    }
}
