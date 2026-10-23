//! `rs_shot::capture` の結合テスト.

use chrono::{FixedOffset, TimeZone};
use rs_shot::capture::{
    file_name, save_dir, store, store_with, unique_path, uri_to_path, MAX_SUFFIX,
};
use std::fs;
use std::path::PathBuf;

/// 撮った時刻が, `rs-shot-YYYYMMDD-HHMMSS.png` の名前になること.
#[test]
fn test_file_name_formats_time() {
    let jst = FixedOffset::east_opt(9 * 3600).unwrap();
    let time = jst.with_ymd_and_hms(2026, 1, 2, 3, 4, 5).unwrap();
    assert_eq!(file_name(&time), "rs-shot-20260102-030405.png");
}

/// ピクチャのディレクトリがあれば, その下の `Screenshots` にすること.
#[test]
fn test_save_dir_uses_pictures() {
    let dir = save_dir(Some(PathBuf::from("/p")), Some(PathBuf::from("/h"))).unwrap();
    assert_eq!(dir, PathBuf::from("/p/Screenshots"));
}

/// ピクチャのディレクトリがなければ, `~/Pictures/Screenshots` にすること.
#[test]
fn test_save_dir_falls_back_to_home() {
    let dir = save_dir(None, Some(PathBuf::from("/h"))).unwrap();
    assert_eq!(dir, PathBuf::from("/h/Pictures/Screenshots"));
}

/// どちらも分からなければ, エラーにすること.
#[test]
fn test_save_dir_fails_without_dirs() {
    assert!(save_dir(None, None).is_err());
}

/// `file` の URI を, パスにすること (エスケープした文字も戻す).
#[test]
fn test_uri_to_path_decodes_file_uri() {
    let path = uri_to_path("file:///tmp/Screenshot%20from%20now.png").unwrap();
    assert_eq!(path, PathBuf::from("/tmp/Screenshot from now.png"));
}

/// URI として読めないものは, エラーにすること.
#[test]
fn test_uri_to_path_rejects_invalid_uri() {
    assert!(uri_to_path("not a uri").is_err());
}

/// `file` 以外の URI は, エラーにすること.
#[test]
fn test_uri_to_path_rejects_other_scheme() {
    assert!(uri_to_path("https://example.com/a.png").is_err());
}

/// 同じ名前がなければ, そのままの名前にすること.
#[test]
fn test_unique_path_keeps_free_name() {
    let dir = tempfile::tempdir().unwrap();
    assert_eq!(
        unique_path(dir.path(), "a.png").unwrap(),
        dir.path().join("a.png")
    );
}

/// 同じ名前があれば, `-1` から順に番号を付けること.
#[test]
fn test_unique_path_adds_suffix() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("a.png"), b"").unwrap();
    fs::write(dir.path().join("a-1.png"), b"").unwrap();
    assert_eq!(
        unique_path(dir.path(), "a.png").unwrap(),
        dir.path().join("a-2.png")
    );
}

/// 番号が尽きたら, エラーにすること.
#[test]
fn test_unique_path_fails_when_all_used() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("a.png"), b"").unwrap();
    for n in 1..=MAX_SUFFIX {
        fs::write(dir.path().join(format!("a-{n}.png")), b"").unwrap();
    }
    assert!(unique_path(dir.path(), "a.png").is_err());
}

/// 撮った画像を, 保存先のディレクトリを作って移すこと (元のファイルは残らない).
#[test]
fn test_store_moves_file() {
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("shot.png");
    fs::write(&source, b"png").unwrap();
    let dest_dir = dir.path().join("Pictures/Screenshots");

    let saved = store(&source, &dest_dir, "rs-shot.png").unwrap();

    assert_eq!(saved, dest_dir.join("rs-shot.png"));
    assert_eq!(fs::read(&saved).unwrap(), b"png");
    assert!(!source.exists());
}

/// 元のファイルがなければ, エラーにすること.
#[test]
fn test_store_fails_without_source() {
    let dir = tempfile::tempdir().unwrap();
    let result = store(&dir.path().join("none.png"), dir.path(), "rs-shot.png");
    assert!(result.is_err());
}

/// 保存先のディレクトリを作れなければ (同じ名前のファイルがある), エラーにすること.
#[test]
fn test_store_fails_when_dir_is_file() {
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("shot.png");
    fs::write(&source, b"png").unwrap();
    let blocker = dir.path().join("blocker");
    fs::write(&blocker, b"").unwrap();
    assert!(store(&source, &blocker.join("sub"), "rs-shot.png").is_err());
}

/// 名前の変更で移せない (別のファイルシステムの) ときは, コピーしてから, 元のファイルを消すこと.
#[test]
fn test_store_copies_when_rename_fails() {
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("shot.png");
    fs::write(&source, b"png").unwrap();
    let dest_dir = dir.path().join("out");

    let saved = store_with(&source, &dest_dir, "rs-shot.png", |_, _| {
        Err(std::io::Error::other("cross-device"))
    })
    .unwrap();

    assert_eq!(fs::read(&saved).unwrap(), b"png");
    assert!(!source.exists());
}
