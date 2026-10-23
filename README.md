# rs-shot

ウィンドウを選んで, スクリーンショットを撮って保存する, 小さなデスクトップアプリです. Tauri v2 (バックエンド: Rust, フロントエンド: React / TypeScript / Vite) で作っています.

## 使い方

1. `ウィンドウを撮る` を押します.
2. デスクトップの撮影の画面が出るので, 撮るウィンドウを選びます (GNOME では, 撮影の画面で「ウィンドウ」を選んで, ウィンドウをクリックします).
3. 画像は, PNG で, `~/Pictures/Screenshots/rs-shot-YYYYMMDD-HHMMSS.png` に保存されます (ピクチャのディレクトリは, `xdg-user-dirs` の設定に従います. 同じ名前があれば, `-1` などの番号を付けます).

撮る間は, rs-shot のウィンドウを隠します. 撮影の画面で取り消すと, 何も保存しません.

## 仕組みと, 動く環境

- Linux (Wayland と X11) で, [xdg-desktop-portal](https://flatpak.github.io/xdg-desktop-portal/) のスクリーンショットのポータルを使います (Rust の [ashpd](https://crates.io/crates/ashpd)). Wayland では, アプリが, ほかのウィンドウの画像を, 直接は読めないためです.
- ポータルの実装 (`xdg-desktop-portal-gnome` など) が, 入っている必要があります. 撮影の画面の見た目と, ウィンドウを選べるかどうかは, デスクトップによって違います.
- 開発している環境: Ubuntu (GNOME Shell 50, Wayland). Windows と macOS には, まだ対応していません.

## 開発

必要なもの: Node.js 20 以上, Rust (stable), Tauri の Linux の依存 (`libwebkit2gtk-4.1-dev` など. [Tauri の前提条件](https://tauri.app/start/prerequisites/)).

```bash
npm install
npm run tauri:dev     # 開発用に起動する
npm run check         # 整形の確認, eslint, clippy, vitest, cargo test
npm run coverage      # カバレッジ (フロントエンドとバックエンド. 100% を下回ると失敗する)
npm run tauri:build   # deb と AppImage を作る
```

## ライセンス

[GPL-3.0-or-later](LICENSE)
