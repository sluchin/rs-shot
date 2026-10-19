//! アプリケーションのエントリポイント.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// アプリケーションを起動します.
#[cfg(not(tarpaulin_include))]
fn main() {
    rs_shot::run();
}
