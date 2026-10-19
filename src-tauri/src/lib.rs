//! rs-shot: ウィンドウを選んで, スクリーンショットを撮って保存するアプリ.

pub mod capture;
mod portal;

use std::time::Duration;

/// 撮る前に, このアプリのウィンドウを隠してから待つ時間 (画面から消えるのを待つ).
#[cfg(not(tarpaulin_include))]
const HIDE_DELAY: Duration = Duration::from_millis(300);

/// ウィンドウを選んで撮り, ピクチャの `Screenshots` に保存します.
///
/// 撮る間は, このアプリのウィンドウを隠します (選ぶ画面の邪魔にならないように).
///
/// # Arguments
///
/// * `window` - このアプリのウィンドウ.
///
/// # Returns
///
/// 保存したファイルのパス. 利用者が取り消したときは, `None`. 失敗したら, [`Err`] (表示用の文字列).
#[cfg(not(tarpaulin_include))]
#[tauri::command]
async fn capture_window(window: tauri::WebviewWindow) -> Result<Option<String>, String> {
    window.hide().map_err(|e| e.to_string())?;
    tokio::time::sleep(HIDE_DELAY).await;
    let uri = portal::request_window().await;
    window.show().map_err(|e| e.to_string())?;
    let Some(uri) = uri? else {
        return Ok(None);
    };
    let source = capture::uri_to_path(&uri)?;
    let dir = capture::save_dir(dirs::picture_dir(), dirs::home_dir())?;
    let name = capture::file_name(&chrono::Local::now());
    let saved = capture::store(&source, &dir, &name)?;
    Ok(Some(saved.display().to_string()))
}

/// アプリを起動します.
///
/// # Panics
///
/// Tauri の起動に失敗したとき.
#[cfg(not(tarpaulin_include))]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![capture_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
