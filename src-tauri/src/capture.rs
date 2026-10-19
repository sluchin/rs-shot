//! 撮ったスクリーンショットのファイルを, 保存先へ移すモジュール.
//!
//! ポータル (xdg-desktop-portal) は, 撮った画像を一時的な場所に置いて, その URI を返します.
//! ここでは, その URI をパスにして, 保存先のディレクトリへ, 撮った時刻の名前で移します.

use chrono::{DateTime, TimeZone};
use std::fs;
use std::path::{Path, PathBuf};
use url::Url;

/// 保存先のディレクトリの名前 (ピクチャのディレクトリの下).
pub const SAVE_DIR_NAME: &str = "Screenshots";

/// 同じ名前のファイルがあるときに, 番号を付けて試す最大の回数.
pub const MAX_SUFFIX: u32 = 999;

/// 撮った時刻から, 保存するファイルの名前を作ります.
///
/// # Arguments
///
/// * `time` - 撮った時刻.
///
/// # Returns
///
/// `rs-shot-YYYYMMDD-HHMMSS.png` の形の名前.
pub fn file_name<Tz: TimeZone>(time: &DateTime<Tz>) -> String
where
    Tz::Offset: std::fmt::Display,
{
    format!("rs-shot-{}.png", time.format("%Y%m%d-%H%M%S"))
}

/// 保存先のディレクトリを決めます.
///
/// # Arguments
///
/// * `pictures` - ピクチャのディレクトリ (分からないときは, `None`).
/// * `home` - ホームディレクトリ (分からないときは, `None`).
///
/// # Returns
///
/// ピクチャのディレクトリの下の `Screenshots`. ピクチャのディレクトリがなければ,
/// `~/Pictures/Screenshots`. どちらも分からなければ, [`Err`].
pub fn save_dir(pictures: Option<PathBuf>, home: Option<PathBuf>) -> Result<PathBuf, String> {
    pictures
        .or_else(|| home.map(|h| h.join("Pictures")))
        .map(|dir| dir.join(SAVE_DIR_NAME))
        .ok_or_else(|| "Could not find the pictures directory".to_string())
}

/// ポータルが返した URI を, ファイルのパスにします.
///
/// # Arguments
///
/// * `uri` - 撮った画像の URI (`file://...`).
///
/// # Returns
///
/// ファイルのパス. `file` の URI でなければ, [`Err`].
pub fn uri_to_path(uri: &str) -> Result<PathBuf, String> {
    let url = Url::parse(uri).map_err(|e| format!("Invalid URI {uri}: {e}"))?;
    url.to_file_path()
        .map_err(|()| format!("Not a file URI: {uri}"))
}

/// 保存先で, まだ使われていないパスを選びます.
///
/// # Arguments
///
/// * `dir` - 保存先のディレクトリ.
/// * `name` - ファイルの名前 (`.png` で終わる).
///
/// # Returns
///
/// `dir/name`. すでにあれば, `-1` から順に番号を付けたもの. 番号が尽きたら, [`Err`].
pub fn unique_path(dir: &Path, name: &str) -> Result<PathBuf, String> {
    let first = dir.join(name);
    if !first.exists() {
        return Ok(first);
    }
    let stem = name.strip_suffix(".png").unwrap_or(name);
    (1..=MAX_SUFFIX)
        .map(|n| dir.join(format!("{stem}-{n}.png")))
        .find(|path| !path.exists())
        .ok_or_else(|| format!("Too many files named {name} in {}", dir.display()))
}

/// 撮った画像を, 保存先へ移します (保存先のディレクトリがなければ, 作ります).
///
/// 名前の変更で移せない (別のファイルシステムの) ときは, コピーしてから, 元のファイルを消します.
///
/// # Arguments
///
/// * `source` - 撮った画像のパス.
/// * `dir` - 保存先のディレクトリ.
/// * `name` - ファイルの名前.
///
/// # Returns
///
/// 保存したパス. 失敗したら, [`Err`].
pub fn store(source: &Path, dir: &Path, name: &str) -> Result<PathBuf, String> {
    fs::create_dir_all(dir).map_err(|e| format!("Could not create {}: {e}", dir.display()))?;
    let dest = unique_path(dir, name)?;
    if fs::rename(source, &dest).is_err() {
        fs::copy(source, &dest).map_err(|e| format!("Could not save {}: {e}", dest.display()))?;
        fs::remove_file(source)
            .map_err(|e| format!("Could not remove {}: {e}", source.display()))?;
    }
    Ok(dest)
}
