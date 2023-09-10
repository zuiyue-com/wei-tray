use systray::Application;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    wei_env::bin_init("wei-tray");
    let instance = single_instance::SingleInstance::new("wei-tray")?;
    if !instance.is_single() { 
        std::process::exit(1);
    };

    let mut app = Application::new().unwrap();

    let mut ico = "./wei.ico";
    let mut path = std::env::current_dir()?;
    path.push("./src/main.rs");
    if path.exists() {
        ico = "../wei/res/wei.ico";
    }

    let ico_path = std::path::Path::new(ico);

    app.set_icon_from_file(&ico_path.to_string_lossy()).unwrap();
    app.add_menu_item(&"启动主界面".to_string(), |_| {
        wei_run::kill("wei-ui").unwrap();
        wei_run::run_async("wei-ui", vec![]).unwrap();
        Ok::<_, systray::Error>(())
    }).unwrap();
    app.add_menu_item(&"退出".to_string(), |window| {
        wei_env::stop();
        window.quit();
        Ok::<_, systray::Error>(())
    }).unwrap();
    app.wait_for_message().unwrap();
    wei_run::kill("wei")?;
    wei_run::kill("wei-ui")?;
    
    Ok(())
}

