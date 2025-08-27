use eframe;
use household_management::{HouseholdApp, config};

fn main() -> Result<(), eframe::Error> {
    let options = config::create_native_options();

    eframe::run_native(
        "户籍管理系统",
        options,
        Box::new(|cc| {
            // 设置自定义字体
            config::setup_custom_fonts(&cc.egui_ctx);
            
            match HouseholdApp::new(cc) {
                Ok(app) => Ok(Box::new(app)),
                Err(e) => {
                    eprintln!("Failed to initialize app: {}", e);
                    std::process::exit(1);
                }
            }
        }),
    )
}


