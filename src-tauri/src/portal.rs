//! xdg-desktop-portal のスクリーンショットのポータルを呼ぶモジュール.
//!
//! Wayland では, アプリが, ほかのウィンドウの画像を, 直接は読めません. ポータルに頼むと,
//! デスクトップ (GNOME など) が, 撮る対象 (ウィンドウ) を選ぶ画面を出して, 撮った画像の URI を返します.

use ashpd::desktop::screenshot::{AvailableTargets, Screenshot};
use ashpd::desktop::ResponseError;
use ashpd::Error;

/// ウィンドウを選んで撮るように, ポータルに頼みます.
///
/// # Returns
///
/// 撮った画像の URI. 利用者が取り消したときは, `None`. ポータルが失敗したら, [`Err`].
#[cfg(not(tarpaulin_include))]
pub async fn request_window() -> Result<Option<String>, String> {
    let result = Screenshot::request()
        .interactive(true)
        .modal(true)
        .target(AvailableTargets::Window)
        .send()
        .await
        .and_then(|request| request.response());
    match result {
        Ok(screenshot) => Ok(Some(screenshot.uri().as_str().to_string())),
        Err(Error::Response(ResponseError::Cancelled)) => Ok(None),
        Err(e) => Err(format!("Screenshot portal failed: {e}")),
    }
}
