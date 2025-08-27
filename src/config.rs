use eframe::egui::{self, FontData, FontDefinitions, FontFamily};

pub struct AppConfig {
    pub window_title: &'static str,
    pub default_width: f32,
    pub default_height: f32,
    pub min_width: f32,
    pub min_height: f32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window_title: "户籍管理系统",
            default_width: 1200.0,
            default_height: 800.0,
            min_width: 800.0,
            min_height: 600.0,
        }
    }
}

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    
    // 加载自定义字体
    let font_data = include_bytes!("../font/字魂柳叶楷书.ttf");
    fonts.font_data.insert(
        "custom_font".to_owned(),
        FontData::from_static(font_data),
    );
    
    // 设置字体优先级
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "custom_font".to_owned());
    
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("custom_font".to_owned());
    
    ctx.set_fonts(fonts);
}

pub fn create_native_options() -> eframe::NativeOptions {
    let config = AppConfig::default();
    
    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([config.default_width, config.default_height])
            .with_title(config.window_title)
            .with_min_inner_size([config.min_width, config.min_height]),
        ..Default::default()
    }
}
