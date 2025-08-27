use eframe::egui::{self, Color32, Rounding, Style, Visuals};

pub struct AppTheme;

impl AppTheme {
    pub fn setup_dark_theme(ctx: &egui::Context) {
        let mut style = Style::default();
        style.visuals = Visuals::dark();
        
        // 自定义颜色
        style.visuals.window_fill = Color32::from_rgb(32, 32, 32);
        style.visuals.panel_fill = Color32::from_rgb(40, 40, 40);
        style.visuals.extreme_bg_color = Color32::from_rgb(24, 24, 24);
        
        // 圆角设置
        style.visuals.window_rounding = Rounding::same(8.0);
        style.visuals.menu_rounding = Rounding::same(6.0);
        
        ctx.set_style(style);
    }
    
    pub fn setup_light_theme(ctx: &egui::Context) {
        let mut style = Style::default();
        style.visuals = Visuals::light();
        
        // 自定义颜色
        style.visuals.window_fill = Color32::from_rgb(248, 248, 248);
        style.visuals.panel_fill = Color32::from_rgb(240, 240, 240);
        
        // 圆角设置
        style.visuals.window_rounding = Rounding::same(8.0);
        style.visuals.menu_rounding = Rounding::same(6.0);
        
        ctx.set_style(style);
    }
}

pub mod colors {
    use eframe::egui::Color32;
    
    pub const PRIMARY: Color32 = Color32::from_rgb(0, 123, 255);
    pub const SUCCESS: Color32 = Color32::from_rgb(40, 167, 69);
    pub const WARNING: Color32 = Color32::from_rgb(255, 193, 7);
    pub const DANGER: Color32 = Color32::from_rgb(220, 53, 69);
    pub const INFO: Color32 = Color32::from_rgb(23, 162, 184);
}

pub mod spacing {
    pub const SMALL: f32 = 4.0;
    pub const MEDIUM: f32 = 8.0;
    pub const LARGE: f32 = 16.0;
    pub const XLARGE: f32 = 24.0;
}
