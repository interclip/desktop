// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};

/*
    Sample API response:
    {"status":"success","result":"k02yl"}
 */
#[derive(serde::Deserialize)]
struct Response {
    status: Status,
    result: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Status {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error
}

fn create_clip (url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let res = client.post("https://interclip.app/api/set")
    .form(&[("url", url)])
    .send()?;


    let response = match res.error_for_status() {
        Ok(resp) => resp,
        Err(err) => {
            println!("Error: {}", err);
            return Err("Error creating clip".into());
        }
    };

    let resp = match response.json::<Response>() {
        Ok(resp) => resp,
        Err(err) => {
            return Err(format!("Server response parsing Error: {}", err).into());
        }
    };

    if resp.status == Status::Error {
        return Err("Error creating clip".into());
    }

    Ok(resp.result)
}

fn retrieve_clip (code: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let res = client.post("https://interclip.app/api/get")
    .form(&[("code", code)])
    .send()?;


    let response = match res.error_for_status() {
        Ok(resp) => resp,
        Err(err) => {
            println!("Error: {}", err);
            return Err("Error retrieving clip".into());
        }
    };

    let resp = match response.json::<Response>() {
        Ok(resp) => resp,
        Err(err) => {
            return Err(format!("Server response parsing Error: {}", err).into());
        }
    };

    if resp.status == Status::Error {
        return Err("Error retrieving clip".into());
    }

    Ok(resp.result)
}

#[tauri::command]
fn create_clip_cmd(url: &str) -> String {
    let clip = create_clip(url).unwrap();
    format!("Your clip is {}!", clip)
}

#[tauri::command]
fn retrieve_clip_cmd(code: &str) -> String {
    let url = retrieve_clip(code).unwrap();
    format!("Your URL is {}!", url)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_clip_cmd, retrieve_clip_cmd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
