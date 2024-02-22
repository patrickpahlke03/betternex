#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::env;
use std::ops::{Add, Deref, DerefMut};
use std::process::{Child, Command};
use std::sync::Mutex;
use std::time::Duration;
use trainex::trainex::{Course, Grade, Trainex, TrainexFile};
use directories::UserDirs;
use native_dialog::FileDialog;
use futures::executor;
use tauri::{Manager, RunEvent, State, WindowEvent};
use trainex::file_manager::{save_file, save_files, save_files_zip, sync_files};


struct ChildGuard(Child);

impl Deref for ChildGuard {
    type Target = Child;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ChildGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        match self.0.kill() {
            Err(e) => println!("Could not kill child process: {}", e),
            Ok(_) => println!("Successfully killed child process"),
        }
    }
}

struct TrainexState {
    trainex: Trainex,
    gecko: Mutex<ChildGuard>,
    sync_process: Mutex<f64>
}

#[tauri::command]
async fn get_archive(state: tauri::State<'_, TrainexState>) -> Result<Vec<Course>, ()> {
    Ok(state.trainex.fetch_archive().await)
}

#[tauri::command]
async fn get_grades(state: tauri::State<'_, TrainexState>) -> Result<Vec<Grade>, ()> {
    Ok(state.trainex.fetch_grades().await)
}

#[tauri::command]
fn download_file(state: tauri::State<'_, TrainexState>, file: TrainexFile) -> Result<String, ()> {

    let location = if let Some(user_dirs) = UserDirs::new().unwrap().download_dir() {
        user_dirs.display().to_string()
    } else {
        stringify!("~/Desktop").to_string()
    };

    let mut answer = FileDialog::new()
            .set_location(location.as_str())
            .show_open_single_dir()
            .unwrap();

    let dir = match answer {
        Some(path) => path,
        None => return Ok("".to_string()),
    };

    let path = dir.display().to_string();

    executor::block_on(save_file(file, &state.trainex, Some(path.clone())));

    Ok(path.to_string())
}

#[tauri::command]
fn download_files(state: tauri::State<'_, TrainexState>, files: Vec<TrainexFile>) -> Result<String, ()> {
    let location = if let Some(user_dirs) = UserDirs::new().unwrap().download_dir() {
        user_dirs.display().to_string()
    } else {
        stringify!("~/Desktop").to_string()
    };

    let mut answer = FileDialog::new()
        .set_location(location.as_str())
        .show_open_single_dir()
        .unwrap();

    let dir = match answer {
        Some(path) => path,
        None => return Ok("".to_string()),
    };

    let path = dir.display().to_string();

    executor::block_on(save_files_zip(files, &state.trainex, path.clone()));

    Ok(path.to_string())
}

#[tauri::command]
async fn sync_archive(state: tauri::State<'_, TrainexState>, archive: Vec<Course>, path: String) -> Result<String, ()> {

    for (i, course) in archive.iter().enumerate() {
        {
            let mut num = state.sync_process.lock().unwrap();
            *num = (i as f64)/(archive.len()as f64)*100.0;
        }
        for child in &course.children {
            save_files(child.files.clone(), &state.trainex, path.clone()).await;
        }
        save_files(course.files.clone(), &state.trainex, path.clone()).await;
    }

    Ok(path.to_string())
}

#[tauri::command]
async fn get_base_sync_path(state: tauri::State<'_, TrainexState>) -> Result<String, ()> {
    Ok(env::var("ARCHIVE_PATH").unwrap_or("".to_string()))
}

#[tauri::command]
fn get_sync_process(state: tauri::State<'_, TrainexState>) -> f64 {
    *state.sync_process.lock().unwrap()
}

#[tokio::main]
async fn main() {
    let _ = fix_path_env::fix();
    let env_path = if let Some(user_dirs) = UserDirs::new().unwrap().document_dir() {
        user_dirs.display().to_string()
    } else {
        stringify!("~/Documents").to_string()
    };
    println!("{}", env_path.as_str());
    dotenv::from_path(format!("{}/BetterNex/.env", env_path)).ok();

    let mut gecko = ChildGuard(Command::new("geckodriver").spawn().unwrap());
    tokio::time::sleep(Duration::from_secs(1)).await;

    let trainex_user = env::var("TRAINEX_USER").expect("No TRAINEX_USER provided in .env");
    let trainex_pass = env::var("TRAINEX_PASS").expect("No TRAINEX_PASS provided in .env");
    let trainex = Trainex::new(trainex_user, trainex_pass).await.unwrap();

    let nav = trainex.navigation.clone();

    let tauri_app = tauri::Builder::default()
        .on_window_event(move |event| match event.event() {
            WindowEvent::Destroyed => {
                let state: State<TrainexState> = event.window().state();
                executor::block_on(state.trainex.client.clone().close()).unwrap();
                state.gecko.lock().unwrap().kill().expect("!kill");
            },
            _ => {}
        })
        .on_menu_event(|event| {
            println!("{}", event.menu_item_id());

            match event.menu_item_id() {

                "quit" => {
                    println!("test");
                }
                _ => {}
            }
        })
        .manage(TrainexState{trainex: trainex.clone(), gecko: Mutex::new(gecko), sync_process: Mutex::new(0.0)})
        .invoke_handler(tauri::generate_handler![get_archive, get_grades, download_file, download_files, sync_archive, get_sync_process, get_base_sync_path])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    tauri_app.run(move |_app_handle, _event| {
        match &_event {
            RunEvent::Exit => {
                let state: State<TrainexState> = _app_handle.state();
                executor::block_on(state.trainex.client.clone().close()).unwrap();
                state.gecko.lock().unwrap().kill().expect("!kill");
            },
            _ => {}
        }
    });
}
