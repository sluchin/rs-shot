import { useState, type ReactElement } from "react";
import { invoke } from "@tauri-apps/api/core";

/**
 * 撮影の状態.
 *
 * - `idle`: まだ撮っていない.
 * - `busy`: 撮っている (ウィンドウを選んでいる).
 * - `saved`: 保存した (`message` は保存したパス).
 * - `cancelled`: 利用者が取り消した.
 * - `error`: 失敗した (`message` はエラーの内容).
 */
export type CaptureStatus =
  | { kind: "idle" }
  | { kind: "busy" }
  | { kind: "saved"; message: string }
  | { kind: "cancelled" }
  | { kind: "error"; message: string };

/**
 * ウィンドウを選んで撮るボタンと, 結果を表示する画面.
 *
 * @returns 画面.
 */
export default function App(): ReactElement {
  const [status, setStatus] = useState<CaptureStatus>({ kind: "idle" });

  /** バックエンドに, ウィンドウを選んで撮るように頼む. */
  const capture = async (): Promise<void> => {
    setStatus({ kind: "busy" });
    try {
      const saved = await invoke<string | null>("capture_window");
      setStatus(
        saved === null
          ? { kind: "cancelled" }
          : { kind: "saved", message: saved },
      );
    } catch (e) {
      setStatus({ kind: "error", message: String(e) });
    }
  };

  return (
    <main className="app">
      <button type="button" onClick={capture} disabled={status.kind === "busy"}>
        ウィンドウを撮る
      </button>
      <p className={`status status-${status.kind}`} role="status">
        {describe(status)}
      </p>
    </main>
  );
}

/**
 * 撮影の状態を, 表示する文にします.
 *
 * @param status - 撮影の状態.
 * @returns 表示する文.
 */
export function describe(status: CaptureStatus): string {
  switch (status.kind) {
    case "idle":
      return "撮るウィンドウを, 次の画面で選んでください.";
    case "busy":
      return "ウィンドウを選んでいます…";
    case "saved":
      return `保存しました: ${status.message}`;
    case "cancelled":
      return "取り消しました.";
    case "error":
      return `撮れませんでした: ${status.message}`;
  }
}
