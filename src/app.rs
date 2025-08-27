use crate::data::models::*;
use crate::data::manager::HouseholdManager;
use eframe::egui;
use uuid::Uuid;

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
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut app = Self {
            household_manager: HouseholdManager::new()?,
            ui_state: UiState::default(),
        };
        
        // 只在数据库为空时添加示例数据
        if app.household_manager.is_empty()? {
            #[cfg(debug_assertions)]
            println!("数据库为空，添加示例数据");
            app.household_manager.add_sample_data()?;
        } else {
            #[cfg(debug_assertions)]
            println!("数据库已有数据，跳过示例数据初始化");
        }
        app.update_filtered_households()?;
        
        Ok(app)
    }
    
    pub fn update_filtered_households(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.ui_state.filtered_households = self.household_manager.search(&self.ui_state.search_query)?;
        Ok(())
    }
    
    pub fn get_households(&mut self) -> Result<Vec<Household>, Box<dyn std::error::Error>> {
        self.household_manager.get_households()
    }
    
    pub fn get_household(&mut self, index: usize) -> Result<Option<Household>, Box<dyn std::error::Error>> {
        self.household_manager.get_household(index)
    }
    
    pub fn add_household(&mut self, household: Household) -> Result<(), Box<dyn std::error::Error>> {
        self.household_manager.add_household(household)?;
        self.update_filtered_households()?;
        Ok(())
    }
    
    pub fn update_household(&mut self, household: Household) -> Result<(), Box<dyn std::error::Error>> {
        self.household_manager.update_household(household)?;
        self.update_filtered_households()?;
        Ok(())
    }
    
    pub fn remove_household(&mut self, household_id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
        self.household_manager.remove_household(household_id)?;
        self.update_filtered_households()?;
        Ok(())
    }
}

impl eframe::App for HouseholdApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render_ui(ctx);
    }
}
