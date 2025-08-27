use crate::data::models::*;
use crate::data::manager::HouseholdManager;

use eframe::egui;

pub struct HouseholdApp {
    pub household_manager: HouseholdManager,
    pub ui_state: UiState,
}

#[derive(Default)]
pub struct UiState {
    pub selected_household: Option<usize>,
    pub show_add_dialog: bool,
    pub show_edit_dialog: bool,
    pub show_error_dialog: bool,
    pub error_message: String,
    pub edit_form: HouseholdForm,
    pub search_query: String,
    pub filtered_households: Vec<usize>,
}

impl HouseholdApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self {
            household_manager: HouseholdManager::new(),
            ui_state: UiState::default(),
        };
        
        // 添加示例数据
        app.household_manager.add_sample_data();
        app.update_filtered_households();
        
        app
    }
    
    pub fn update_filtered_households(&mut self) {
        self.ui_state.filtered_households = self.household_manager.search(&self.ui_state.search_query);
    }
    
    pub fn get_households(&self) -> &Vec<Household> {
        self.household_manager.get_households()
    }
    
    pub fn get_household(&self, index: usize) -> Option<&Household> {
        self.household_manager.get_household(index)
    }
    
    pub fn add_household(&mut self, household: Household) {
        self.household_manager.add_household(household);
        self.update_filtered_households();
    }
    
    pub fn update_household(&mut self, index: usize, household: Household) {
        self.household_manager.update_household(index, household);
        self.update_filtered_households();
    }
    
    pub fn remove_household(&mut self, index: usize) {
        self.household_manager.remove_household(index);
        if let Some(selected) = self.ui_state.selected_household {
            if selected >= index {
                self.ui_state.selected_household = if selected == index {
                    None
                } else {
                    Some(selected - 1)
                };
            }
        }
        self.update_filtered_households();
    }
}

impl eframe::App for HouseholdApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render_ui(ctx);
    }
}
