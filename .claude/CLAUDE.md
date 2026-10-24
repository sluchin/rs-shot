# プロジェクト設定

rs-shot は, ウィンドウを選んで, スクリーンショットを撮って保存するアプリ. Tauri v2 (バックエンド: Rust, フロントエンド: React 19 / TypeScript / Vite) で作る. 公開リポジトリ.

## コマンド

すべて, リポジトリのルートで実行する (`package.json` の scripts).

- 開発用に起動: `npm run tauri:dev`
- ビルド: `npm run build` (フロントエンド). `npm run tauri:build` (deb と AppImage)
- 確認: `npm run check` (prettier と `cargo fmt` の整形の確認, eslint, clippy (`-D warnings`), vitest, cargo test. CI と同じ)
- 整形: `npm run format`
- カバレッジ: `npm run coverage` (フロントエンド (vitest) とバックエンド (cargo tarpaulin --engine llvm). どちらも 100% を下回ると失敗する. CI の `coverage` のジョブも同じ)

## プロジェクト構成

- `src/`: フロントエンド. `App.tsx` (ボタンと, 結果の表示). テストは `tests/` (`src/` と同じ構成. `src/` は `@/` で参照する).
- `src-tauri/src/`: バックエンド.
  - `lib.rs`: Tauri の起動と, IPC コマンド `capture_window` (ウィンドウを隠す, ポータルで撮る, ウィンドウを戻す, 保存先へ移す).
  - `portal.rs`: xdg-desktop-portal のスクリーンショットのポータル (ashpd). 利用者がウィンドウを選ぶ (`interactive`, 対象は `Window`).
  - `capture.rs`: ファイルの名前, 保存先 (`~/Pictures/Screenshots`), URI からパスへの変換, 保存先への移動. Tauri とポータルに依存しない関数にして, テストする.
- `src-tauri/tests/`: Rust のテスト (`capture_test.rs`).
- `.github/workflows/ci.yml`: `main` への push と pull request で, `npm run check` を実行する.

## コードの規約

- ドキュメントコメントとコメントは, 日本語. 句読点は「, 」と「.」. ログとエラーの文字列, 識別子は英語.
- Rust の公開する関数には `///` を付けて, `# Arguments`, `# Returns`, `# Panics` を書く. IPC コマンドは `Result<T, String>` を返す. 実行時のコードでは `unwrap()` / `expect()` を使わない (`run()` の起動の失敗だけは除く).
- ロジックは, Tauri とポータルに依存しない関数 (`capture.rs`) に書く. 実際の画面とポータルが要る部分 (`portal.rs`, `capture_window`, `run`) だけを, `#[cfg(not(tarpaulin_include))]` でカバレッジから外す.
- フロントエンドは, 関数コンポーネント. `any` を使わない. 公開する型と関数には TSDoc を付ける. バックエンドは `invoke` で呼ぶ.

## テストの規約

- **Rust のテストは, `src-tauri/tests/` にファイルを分けて置く** (本番のソースの中に `#[cfg(test)] mod tests` を書かない). テストが呼ぶ関数は `pub` にする. 名前は `test_<対象の関数名>_<内容>`. ファイルを使うテストは `tempfile` を使う.
- フロントエンドは vitest. `@tauri-apps/api/core` の `invoke` を `vi.mock` でモックする. `it` の説明は「正常系: ...」「異常系: ...」「境界: ...」. 各テストの前に `vi.clearAllMocks()`.
- 機能を足したり直したりしたら, テストを足す. カバレッジは 100% を保つ (フロントエンドもバックエンドも).
- 実際の撮影 (ポータル, デスクトップの撮影の画面) は, 単体テストでは確認できないので, `npm run tauri:dev` で, 手で確認する.

## 開発ルール

- コードを直したら, `npm run build` と `npm run check` を実行して, エラーと警告がなくなるまで直す (整形で失敗したら `npm run format`). 変更が Markdown だけのときは, 実行しない.
- 依存を変えたときだけ, `package-lock.json` と `Cargo.lock` を更新する. バージョン (`package.json`, `Cargo.toml`, `tauri.conf.json`) は, 勝手に変えない.
- 修正が終わったら, `git diff --stat` を確認して, 変更点と理由を簡潔に報告する. 依頼されない限り, `git commit` / `git push` はしない.

## コミットガイドライン

- コミットメッセージは英語. 件名は Conventional Commits (`type(scope): description`. description は命令形). type は `feat` `fix` `docs` `style` `refactor` `test` `build` `ci` `chore`. scope は `backend` (`src-tauri/` だけ), `frontend` (`src/` だけ), `deps`. 省略できる.
- 件名のあとに空行を 1 行入れて, 本文を `- ` で始まる箇条書きで書く.
- コミットメッセージに `Co-Authored-By` などの帰属の行を付けない.
- rebase したあとの push は `git push --force-with-lease --force-if-includes origin <ブランチ名>` (`--force` は使わない). 依頼されたときだけ push する.
