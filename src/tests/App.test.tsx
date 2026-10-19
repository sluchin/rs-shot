import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { invoke } from "@tauri-apps/api/core";
import App from "../App";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

const mockedInvoke = vi.mocked(invoke);

describe("App", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("正常系: 最初は, ウィンドウを選ぶように案内すること", () => {
    render(<App />);
    expect(screen.getByRole("status")).toHaveTextContent(
      "次の画面で選んでください",
    );
  });

  it("正常系: 撮ったら, 保存したパスを表示すること", async () => {
    mockedInvoke.mockResolvedValue("/home/u/Pictures/Screenshots/rs-shot.png");
    render(<App />);
    await userEvent.click(
      screen.getByRole("button", { name: "ウィンドウを撮る" }),
    );
    expect(mockedInvoke).toHaveBeenCalledWith("capture_window");
    expect(
      await screen.findByText(/保存しました: .*rs-shot\.png/),
    ).toBeInTheDocument();
  });

  it("正常系: 取り消したら, 取り消したことを表示すること", async () => {
    mockedInvoke.mockResolvedValue(null);
    render(<App />);
    await userEvent.click(
      screen.getByRole("button", { name: "ウィンドウを撮る" }),
    );
    expect(await screen.findByText("取り消しました.")).toBeInTheDocument();
  });

  it("異常系: 失敗したら, エラーの内容を表示すること", async () => {
    mockedInvoke.mockRejectedValue("Screenshot portal failed");
    render(<App />);
    await userEvent.click(
      screen.getByRole("button", { name: "ウィンドウを撮る" }),
    );
    expect(
      await screen.findByText("撮れませんでした: Screenshot portal failed"),
    ).toBeInTheDocument();
  });

  it("正常系: 撮っている間は, ボタンを押せないこと", async () => {
    let finish: (value: string | null) => void = () => {};
    mockedInvoke.mockReturnValue(new Promise((resolve) => (finish = resolve)));
    render(<App />);
    const button = screen.getByRole("button", { name: "ウィンドウを撮る" });
    await userEvent.click(button);
    expect(button).toBeDisabled();
    expect(screen.getByRole("status")).toHaveTextContent("選んでいます");
    finish(null);
    expect(await screen.findByText("取り消しました.")).toBeInTheDocument();
  });
});
