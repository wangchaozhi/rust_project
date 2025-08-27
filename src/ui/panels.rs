use crate::app::HouseholdApp;
use crate::data::models::*;
use eframe::egui::{self, *};

impl HouseholdApp {
    pub fn render_menu_bar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading(RichText::new("户籍管理系统").size(20.0));
            
            ui.separator();
            
            if ui.button(RichText::new("新增户籍").size(14.0)).clicked() {
                self.ui_state.edit_form.clear();
                self.ui_state.show_add_dialog = true;
            }
            
            if ui.button(RichText::new("编辑户籍").size(14.0)).clicked() {
                if let Some(selected) = self.ui_state.selected_household {
                    if let Some(household) = self.get_household(selected) {
                        self.ui_state.edit_form = HouseholdForm::from_household(household);
                        self.ui_state.show_edit_dialog = true;
                    }
                }
            }
            
            if ui.button(RichText::new("删除户籍").size(14.0)).clicked() {
                if let Some(selected) = self.ui_state.selected_household {
                    self.remove_household(selected);
                }
            }
            
            ui.separator();
            
            ui.label("搜索:");
            if ui.text_edit_singleline(&mut self.ui_state.search_query).changed() {
                self.update_filtered_households();
            }
            
            ui.separator();
            
            // 显示统计信息
            let stats = self.household_manager.get_statistics();
            ui.label(format!("总户数: {}", stats.total_households));
            ui.label(format!("总人数: {}", stats.total_members));
        });
    }

    pub fn render_household_list_panel(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.heading(RichText::new("户籍列表").size(16.0));
            ui.separator();
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                for &index in &self.ui_state.filtered_households {
                    if let Some(household) = self.get_household(index) {
                        let is_selected = self.ui_state.selected_household == Some(index);
                        
                        let response = ui.selectable_label(
                            is_selected,
                            format!("{} - {}", household.head_name, household.household_type)
                        );
                        
                        if response.clicked() {
                            self.ui_state.selected_household = Some(index);
                        }
                        
                        ui.separator();
                    }
                }
            });
        });
    }

    pub fn render_household_details_panel(&mut self, ui: &mut Ui) {
        if let Some(selected) = self.ui_state.selected_household {
            if let Some(household) = self.get_household(selected) {
                ui.vertical(|ui| {
                    ui.heading(RichText::new("户籍详细信息").size(18.0));
                    ui.separator();
                    
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        self.render_basic_info(ui, household);
                        ui.add_space(10.0);
                        self.render_members_info(ui, household);
                    });
                });
            }
        } else {
            ui.vertical_centered(|ui| {
                ui.add_space(200.0);
                ui.heading(RichText::new("请选择一个户籍查看详细信息").size(16.0));
            });
        }
    }
    
    fn render_basic_info(&self, ui: &mut Ui, household: &Household) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading(RichText::new("基本信息").size(16.0));
                
                ui.horizontal(|ui| {
                    ui.label(RichText::new("户主姓名:").size(14.0));
                    ui.label(RichText::new(&household.head_name).size(14.0));
                });
                
                ui.horizontal(|ui| {
                    ui.label(RichText::new("身份证号:").size(14.0));
                    ui.label(RichText::new(&household.id_number).size(14.0));
                });
                
                ui.horizontal(|ui| {
                    ui.label(RichText::new("户口类型:").size(14.0));
                    ui.label(RichText::new(&household.household_type.to_string()).size(14.0));
                });
                
                ui.horizontal(|ui| {
                    ui.label(RichText::new("联系电话:").size(14.0));
                    ui.label(RichText::new(&household.phone).size(14.0));
                });
                
                ui.horizontal(|ui| {
                    ui.label(RichText::new("家庭地址:").size(14.0));
                    ui.label(RichText::new(&household.address).size(14.0));
                });
                
                ui.horizontal(|ui| {
                    ui.label(RichText::new("登记日期:").size(14.0));
                    ui.label(RichText::new(&household.registration_date.format("%Y-%m-%d %H:%M:%S").to_string()).size(14.0));
                });
            });
        });
    }
    
    fn render_members_info(&self, ui: &mut Ui, household: &Household) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading(RichText::new("家庭成员").size(16.0));
                
                for (i, member) in household.members.iter().enumerate() {
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.heading(RichText::new(&format!("成员 {}", i + 1)).size(14.0));
                            
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("姓名:").size(12.0));
                                ui.label(RichText::new(&member.name).size(12.0));
                                
                                ui.separator();
                                
                                ui.label(RichText::new("关系:").size(12.0));
                                ui.label(RichText::new(&member.relationship.to_string()).size(12.0));
                                
                                ui.separator();
                                
                                ui.label(RichText::new("性别:").size(12.0));
                                ui.label(RichText::new(&member.gender.to_string()).size(12.0));
                            });
                            
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("身份证号:").size(12.0));
                                ui.label(RichText::new(&member.id_number).size(12.0));
                            });
                            
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("出生日期:").size(12.0));
                                ui.label(RichText::new(&member.birth_date.format("%Y-%m-%d").to_string()).size(12.0));
                                
                                ui.separator();
                                
                                ui.label(RichText::new("学历:").size(12.0));
                                ui.label(RichText::new(&member.education.to_string()).size(12.0));
                            });
                            
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("职业:").size(12.0));
                                ui.label(RichText::new(&member.occupation).size(12.0));
                            });
                        });
                    });
                    ui.add_space(5.0);
                }
            });
        });
    }
}
