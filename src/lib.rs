// use systray::Application;

// pub fn start() -> Result<(), Box<dyn std::error::Error>> {
//     wei_env::bin_init("wei-tray");
//     let instance = single_instance::SingleInstance::new("wei-tray")?;
//     if !instance.is_single() { 
//         std::process::exit(1);
//     };

//     let mut app = Application::new().unwrap();

//     let mut ico = "./wei.ico";
//     let mut path = std::env::current_dir()?;
//     path.push("./src/main.rs");
//     if path.exists() {
//         ico = "../wei/res/wei.ico";
//     }

//     let ico_path = std::path::Path::new(ico);

//     app.set_icon_from_file(&ico_path.to_string_lossy()).unwrap();
//     app.add_menu_item(&"启动主界面".to_string(), |_| {
//         wei_run::kill("wei-ui").unwrap();
//         wei_run::run_async("wei-ui", vec![]).unwrap();
//         Ok::<_, systray::Error>(())
//     }).unwrap();
//     app.add_menu_item(&"退出".to_string(), |window| {
//         wei_env::stop();
//         window.quit();
//         Ok::<_, systray::Error>(())
//     }).unwrap();
//     app.wait_for_message().unwrap();

//     wei_run::kill("wei")?;
//     wei_run::kill("wei-ui")?;
    
//     Ok(())
// }

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, SystemTrayMenuItem};
use tauri::Manager;

pub fn start() -> Result<(), Box<dyn std::error::Error>> {
  let quit = CustomMenuItem::new("exit".to_string(), "退出");
  let hide = CustomMenuItem::new("hide".to_string(), "隐藏");
  let tray_menu = SystemTrayMenu::new()
    .add_item(quit)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(hide);

  tauri::Builder::default()
    .system_tray(SystemTray::new().with_menu(tray_menu))
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick {
        position: _,
        size: _,
        ..
      } => {
        let window = app.get_window("main").unwrap();
        window.show().unwrap();
      }
      SystemTrayEvent::RightClick {
        position: _,
        size: _,
        ..
      } => {
        println!("system tray received a right click");
      }
      SystemTrayEvent::DoubleClick {
        position: _,
        size: _,
        ..
      } => {
        let window = app.get_window("main").unwrap();
        window.show().unwrap();
      }
      SystemTrayEvent::MenuItemClick { id, .. } => {
        match id.as_str() {
          "exit" => {
            wei_env::stop();
            std::process::exit(0);
          }
          "hide" => {
            let window = app.get_window("main").unwrap();
            window.hide().unwrap();
          }
          _ => {}
        }
      }
      _ => {}
    })
    .run(tauri::generate_context!("./tauri.conf.json"))
    .expect("error while running tauri application");

    Ok(())
}
