use systray::Application;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let instance = single_instance::SingleInstance::new("wei-tray")?;
    if !instance.is_single() { 
        std::process::exit(1);
    };

    let mut app = Application::new().unwrap();
    // let home_dir = wei_env::home_dir().unwrap();
    let ico_path = std::path::Path::new("../wei/res/wei.ico");

    app.set_icon_from_file(&ico_path.to_string_lossy()).unwrap();
    app.add_menu_item(&"打开主界面".to_string(), |_| {
        let _ = webbrowser::open("https://www.zuiyue.com");
        Ok::<_, systray::Error>(())
    }).unwrap();
    app.add_menu_item(&"退出".to_string(), |window| {
        wei_env::stop();
        window.quit();
        Ok::<_, systray::Error>(())
    }).unwrap();
    app.wait_for_message().unwrap();
    wei_run::kill("wei-ui")?;
}
