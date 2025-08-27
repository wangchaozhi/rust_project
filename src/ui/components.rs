use crate::app::HouseholdApp;
use eframe::egui;

impl HouseholdApp {
    pub fn render_ui(&mut self, ctx: &egui::Context) {
        // 顶部菜单栏
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.render_menu_bar(ui);
        });

        // 左侧面板 - 户籍列表
        egui::SidePanel::left("household_list")
            .min_width(300.0)
            .max_width(500.0)
            .show(ctx, |ui| {
                self.render_household_list_panel(ui);
            });

        // 中央面板 - 详细信息
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_household_details_panel(ui);
        });

        // 对话框
        self.render_dialogs(ctx);
        
        // 错误提示对话框
        self.render_error_dialog(ctx);
    }
    
    fn render_error_dialog(&mut self, ctx: &egui::Context) {
        if self.ui_state.show_error_dialog {
            egui::Window::new("错误提示")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.add_space(10.0);
                        
                        ui.horizontal(|ui| {
                            // 添加错误图标
                            ui.colored_label(egui::Color32::RED, "⚠");
                            ui.label(&self.ui_state.error_message);
                        });
                        
                        ui.add_space(15.0);
                        
                        ui.horizontal(|ui| {
                            ui.add_space(ui.available_width() - 60.0);
                            if ui.button("确定").clicked() {
                                self.ui_state.show_error_dialog = false;
                                self.ui_state.error_message.clear();
                            }
                        });
                    });
                });
        }
    }
}
