// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};

/*
   Sample API response:
   {"status":"success","result":"k02yl"}
*/
#[derive(serde::Deserialize, serde::Serialize)]
struct Response {
    status: Status,
    result: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Status {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
}

#[derive(serde::Deserialize)]
struct AdditionalOptions {
    endpoint: String,
}

const DEFAULT_ENDPOINT: &str = "https://interclip.app";

fn create_clip(
    url: &str,
    options: Option<AdditionalOptions>,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let options = match options {
        Some(options) => options,
        None => AdditionalOptions {
            endpoint: DEFAULT_ENDPOINT.to_string(),
        },
    };

    let res = client
        .post(format!("{}/api/set", options.endpoint))
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

fn retrieve_clip(
    code: &str,
    options: Option<AdditionalOptions>,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let options = match options {
        Some(options) => options,
        None => AdditionalOptions {
            endpoint: DEFAULT_ENDPOINT.to_string(),
        },
    };

    let res = client
        .post(format!("{}/api/get", options.endpoint))
        .form(&[("code", code)])
        .send()?;

    let response = match res.error_for_status() {
        Ok(resp) => resp,
        Err(err) => {
            if err.status() == Some(reqwest::StatusCode::NOT_FOUND) {
                return Err("Clip not found".into());
            }

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
        return Err(resp.result.into());
    }

    Ok(resp.result)
}

#[tauri::command]
fn create_clip_cmd(url: &str, options: Option<AdditionalOptions>) -> Response {
    let clip = match create_clip(url, options) {
        Ok(clip) => Response { status: Status::Success, result: clip },
        Err(err) => {
            return Response { status: Status::Error, result: format!("Error creating clip: {}", err) };
        }
    };
    return clip;
}

#[tauri::command]
fn retrieve_clip_cmd(code: &str, options: Option<AdditionalOptions>) -> Response {
    let clip = match retrieve_clip(code, options) {
        Ok(url) => Response { status: Status::Success, result: url },
        Err(err) => {
            return Response { status: Status::Error, result: format!("Error retrieving clip: {}", err) };
        }
    };
    return clip;
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![create_clip_cmd, retrieve_clip_cmd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
