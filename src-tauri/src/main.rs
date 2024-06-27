#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, fs, path::PathBuf, sync::Arc};

use serde::{Deserialize, Serialize};
use tauri::{Manager, Window, WindowEvent};

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Mod {
	#[serde(default, rename(deserialize = "UniqueID"))]
	uniqueid: String,
	#[serde(default, rename(deserialize = "Name"))]
	name: String,
	#[serde(default, rename(deserialize = "Author"))]
	author: String,
	#[serde(default, rename(deserialize = "Version"))]
	version: String,
	#[serde(default, rename(deserialize = "Description"))]
	description: String
}

#[derive(Clone, Serialize)]
struct ModsEvent {
	mods: Vec<Mod>
}

#[derive(Clone, Serialize)]
enum FileEvent {
	#[serde(rename = "hovered")]
	Hovered,
	#[serde(rename = "drop")]
	Drop,
	#[serde(rename = "cancelled")]
	Cancelled
}

struct Config {
	folder: String
}

fn main() {
	let config = Arc::new(Config {
		folder: String::new()
	});

  	tauri::Builder::default()
		.setup(move |app| {
			let window = app.get_window("main").unwrap();

			let p = app.path_resolver();
			p.app_data_dir();

			// app.state()

			Ok(())
		})
		.on_window_event(|event| match event.event() {
			WindowEvent::FileDrop(files) => {
				let window = event.window();

				match files {
					tauri::FileDropEvent::Hovered(_) => window.emit("file", FileEvent::Hovered).unwrap(),
					tauri::FileDropEvent::Dropped(_) => window.emit("file", FileEvent::Drop).unwrap(),
					_ => window.emit("file", FileEvent::Cancelled).unwrap()
				}
			},
			_ => {}
		})
		.manage(Config {
			folder: String::new()
		})
		.invoke_handler(tauri::generate_handler![get_mods])
    	.run(tauri::generate_context!())
    	.expect("Failed to start Stardew Valley Mod Manager");
}

#[tauri::command]
fn get_mods(window: Window) {
	window.emit("mods", ModsEvent {
		mods: mods(PathBuf::from(r"C:\Users\The Yule\Documents\mods")) // window.app_handle().path_resolver().app_data_dir().unwrap()
	}).unwrap();
}

fn mods(path: PathBuf) -> Vec<Mod> {
	let mut mods = vec![];

	if let Ok(dir) = fs::read_dir(path) {
		for entry in dir.into_iter() {
			if entry.is_ok() {
				let file = entry.unwrap();
				let mut path = file.path();
				
				if path.is_dir() {
					path.push("manifest.json");

					match read_json::<Mod>(path) {
						Ok(m) => mods.push(m),
						Err(error) => println!("Mod error: {:?}", error)
					}
				}
			}
		}
	}

	return mods;
}

pub fn read_json<T: for<'a> Deserialize<'a>>(path: PathBuf) -> Result<T, Box<dyn Error>> {
	// Read file and remove BOM (\u{feff})
	let data = fs::read_to_string(path)?.replace('\u{feff}', "");
	let json = serde_json::from_str(data.as_str())?;

    Ok(json)
}