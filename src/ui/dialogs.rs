use crate::app::HouseholdApp;
use crate::data::models::*;
use crate::data::validation::Validate;
use eframe::egui::{self, *};

impl HouseholdApp {
    pub fn render_dialogs(&mut self, ctx: &egui::Context) {
        if self.ui_state.show_add_dialog {
            self.render_add_dialog(ctx);
        }
        
        if self.ui_state.show_edit_dialog {
            self.render_edit_dialog(ctx);
        }
    }

    fn render_add_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("新增户籍")
            .id(egui::Id::new("add_household_dialog"))
            .collapsible(false)
            .resizable(true)
            .default_size([600.0, 500.0])
            .show(ctx, |ui| {
                self.render_household_form(ui, true);
            });
    }

    fn render_edit_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("编辑户籍")
            .id(egui::Id::new("edit_household_dialog"))
            .collapsible(false)
            .resizable(true)
            .default_size([600.0, 500.0])
            .show(ctx, |ui| {
                self.render_household_form(ui, false);
            });
    }

    pub fn render_household_form(&mut self, ui: &mut Ui, is_add: bool) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical(|ui| {
                // 基本信息表单
                self.render_basic_info_form(ui);
                
                ui.add_space(10.0);
                
                // 家庭成员表单
                self.render_members_form(ui);
                
                ui.add_space(10.0);
                
                // 按钮
                self.render_form_buttons(ui, is_add);
            });
        });
    }
    
    fn render_basic_info_form(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("基本信息");
                
                ui.horizontal(|ui| {
                    ui.label("户主姓名:");
                    ui.add(improved_text_edit_singleline(&mut self.ui_state.edit_form.head_name, "dialog_head_name")
                        .desired_width(200.0));
                });
                
                ui.horizontal(|ui| {
                    ui.label("身份证号:");
                    ui.add(egui::TextEdit::singleline(&mut self.ui_state.edit_form.id_number)
                        .id_source("dialog_id_number")
                        .desired_width(200.0));
                });
                
                ui.horizontal(|ui| {
                    ui.label("户口类型:");
                    egui::ComboBox::from_id_salt("dialog_household_type")
                        .selected_text(self.ui_state.edit_form.household_type.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.ui_state.edit_form.household_type, HouseholdType::Urban, "城镇户口");
                            ui.selectable_value(&mut self.ui_state.edit_form.household_type, HouseholdType::Rural, "农村户口");
                        });
                });
                
                ui.horizontal(|ui| {
                    ui.label("联系电话:");
                    ui.add(egui::TextEdit::singleline(&mut self.ui_state.edit_form.phone)
                        .id_source("dialog_phone")
                        .desired_width(200.0));
                });
                
                ui.horizontal(|ui| {
                    ui.label("家庭地址:");
                    ui.add(egui::TextEdit::multiline(&mut self.ui_state.edit_form.address)
                        .id_source("dialog_address")
                        .desired_width(300.0)
                        .desired_rows(3));
                });
            });
        });
    }
    
    fn render_members_form(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.heading("家庭成员");
                    
                    if ui.button("添加成员").clicked() {
                        self.ui_state.edit_form.members.push(MemberForm::default());
                    }
                });
                
                let mut to_remove = None;
                let members_len = self.ui_state.edit_form.members.len();
                
                // 分离可变借用
                let members = &mut self.ui_state.edit_form.members;
                for (i, member) in members.iter_mut().enumerate() {
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.heading(&format!("成员 {}", i + 1));
                                
                                if members_len > 1 && ui.button("删除").clicked() {
                                    to_remove = Some(i);
                                }
                            });
                            
                            // 直接在这里渲染成员表单，避免调用self方法
                            render_member_form_inline(ui, member, i);
                        });
                    });
                    ui.add_space(5.0);
                }
                
                if let Some(index) = to_remove {
                    self.ui_state.edit_form.members.remove(index);
                }
            });
        });
    }
    

    
    fn render_form_buttons(&mut self, ui: &mut Ui, is_add: bool) {
        ui.horizontal(|ui| {
            if ui.button("保存").clicked() {
                println!("保存按钮被点击");
                match self.ui_state.edit_form.validate() {
                    Ok(()) => {
                        println!("表单验证通过");
                        if is_add {
                            println!("执行新增操作");
                            if let Some(household) = self.ui_state.edit_form.to_household(None) {
                                println!("成功创建Household对象");
                                if let Err(e) = self.add_household(household) {
                                    eprintln!("Failed to add household: {}", e);
                                    self.ui_state.error_message = format!("添加失败: {}", e);
                                    self.ui_state.show_error_dialog = true;
                                } else {
                                    println!("成功添加户籍");
                                    self.ui_state.show_add_dialog = false;
                                    self.ui_state.edit_form.clear();
                                }
                            } else {
                                println!("无法创建Household对象");
                                self.ui_state.error_message = "无法创建户籍对象".to_string();
                                self.ui_state.show_error_dialog = true;
                            }
                        } else {
                            if let Some(selected) = self.ui_state.selected_household {
                                if let Ok(Some(existing_household)) = self.get_household(selected) {
                                    if let Some(updated_household) = self.ui_state.edit_form.to_household(Some(existing_household.id)) {
                                        if let Err(e) = self.update_household(updated_household) {
                                            eprintln!("Failed to update household: {}", e);
                                        } else {
                                            self.ui_state.show_edit_dialog = false;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        println!("表单验证失败: {}", error);
                        // 显示错误对话框
                        self.ui_state.error_message = error;
                        self.ui_state.show_error_dialog = true;
                    }
                }
            }
            
            if ui.button("取消").clicked() {
                if is_add {
                    self.ui_state.show_add_dialog = false;
                } else {
                    self.ui_state.show_edit_dialog = false;
                }
                self.ui_state.edit_form.clear();
            }
        });
    }
}

// 独立的成员表单渲染函数，避免借用检查器问题
fn render_member_form_inline(ui: &mut Ui, member: &mut MemberForm, index: usize) {
    use crate::data::models::*;
    
    ui.horizontal(|ui| {
        ui.label("姓名:");
        ui.add(egui::TextEdit::singleline(&mut member.name)
            .id_source(format!("dialog_member_name_{}", index))
            .desired_width(120.0));
        
        ui.label("关系:");
        egui::ComboBox::from_id_salt(format!("dialog_member_relationship_{}", index))
            .selected_text(member.relationship.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut member.relationship, Relationship::Head, "户主");
                ui.selectable_value(&mut member.relationship, Relationship::Spouse, "配偶");
                ui.selectable_value(&mut member.relationship, Relationship::Child, "子女");
                ui.selectable_value(&mut member.relationship, Relationship::Parent, "父母");
                ui.selectable_value(&mut member.relationship, Relationship::Other, "其他");
            });
    });
    
    ui.horizontal(|ui| {
        ui.label("身份证号:");
        ui.add(egui::TextEdit::singleline(&mut member.id_number)
            .id_source(format!("dialog_member_id_number_{}", index))
            .desired_width(160.0));
        
        ui.label("性别:");
        egui::ComboBox::from_id_salt(format!("dialog_member_gender_{}", index))
            .selected_text(member.gender.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut member.gender, Gender::Male, "男");
                ui.selectable_value(&mut member.gender, Gender::Female, "女");
            });
    });
    
    ui.horizontal(|ui| {
        ui.label("出生年份:");
        ui.add(egui::DragValue::new(&mut member.birth_year).range(1900..=2024));
        
        ui.label("月:");
        ui.add(egui::DragValue::new(&mut member.birth_month).range(1..=12));
        
        ui.label("日:");
        ui.add(egui::DragValue::new(&mut member.birth_day).range(1..=31));
    });
    
    ui.horizontal(|ui| {
        ui.label("学历:");
        egui::ComboBox::from_id_salt(format!("dialog_member_education_{}", index))
            .selected_text(member.education.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut member.education, Education::Primary, "小学");
                ui.selectable_value(&mut member.education, Education::MiddleSchool, "初中");
                ui.selectable_value(&mut member.education, Education::HighSchool, "高中");
                ui.selectable_value(&mut member.education, Education::College, "大专");
                ui.selectable_value(&mut member.education, Education::University, "本科");
                ui.selectable_value(&mut member.education, Education::Graduate, "研究生");
                ui.selectable_value(&mut member.education, Education::Other, "其他");
            });
        
        ui.label("职业:");
        ui.add(egui::TextEdit::singleline(&mut member.occupation)
            .id_source(format!("dialog_member_occupation_{}", index))
            .desired_width(120.0));
    });
}

// 辅助函数：创建改进的单行文本输入框
fn improved_text_edit_singleline(text: &mut String, id: impl std::hash::Hash) -> egui::TextEdit {
    egui::TextEdit::singleline(text)
        .id(egui::Id::new(id))
        .clip_text(false)
        .cursor_at_end(false)
}
