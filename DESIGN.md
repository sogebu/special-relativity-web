# special-relativity-web — DESIGN

設計判断とその理由。CLAUDE.md には「何がどうなっているか」、ここには「なぜそうしたか」。

> このファイルは 2026-06-23 に claude-config 規約（CLAUDE / SESSION / DESIGN の三層）に沿って新設した。**設計判断の rationale の多くは元の作者（sogebu org）の知識で、判明し次第ここに追記する**（推測で埋めない）。

## 観測される構成（rationale は今後メンテナが追記）

- **Rust → WebAssembly で計算、TypeScript + webpack でフロントエンド**: 相対論・電磁場の計算を Rust（wasm）に、UI/描画の glue を TS に置く 2 層構成。
- **Rust workspace + crate 分割**（root crate `special-relativity-rs` + `crates/{rmath, shape, color, backend}`）: 数学・形状・配色・描画 backend を独立 crate に切り出している。各分割の判断根拠（境界の引き方・依存方向）は要追記。
- **GitHub Pages を `dev` ブランチへデプロイ**（`gh-pages -d dist -e dev`）: 公開先 https://sogebu.github.io/special-relativity-web/dev 。

## 未文書化（メンテナ確認待ち）

- 各 crate の責務境界の設計意図
- WebAssembly 採用の経緯（性能要件・Rust 数値計算の再利用 等）
- 背景論文 2 本（過去光円錐形式の共変電磁気学 / 光円錐を通して見る可視化）と本実装の対応関係
