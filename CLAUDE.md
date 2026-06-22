# special-relativity-web

特殊相対論・共変電磁気学（過去光円錐形式）の可視化 Web アプリ。Rust → WebAssembly で計算し、TypeScript + webpack でフロントエンドを組み、GitHub Pages で公開している。sogebu org のプロジェクト。

## 背景論文
- [Covariant Electromagnetism in Past-Light-Cone Formalism](https://www.arxiv.org/abs/2408.05481v3)
- [Seeing through the light cone: Visualizing electromagnetic fields in special relativity](https://arxiv.org/abs/2505.20596)

## リポジトリ情報
- リモート: `sogebu/special-relativity-web` (public, GitHub)
- ブランチ: `main`（開発）/ `dev`（GitHub Pages デプロイ先、`npm run deploy` が `gh-pages` で push）
- 公開 URL: https://sogebu.github.io/special-relativity-web/dev
- 開発: sogebu org のプロジェクト（contributors は GitHub の commit 履歴 / org メンバー参照）。

## 構造

Rust workspace（wasm 計算本体）と TypeScript フロントエンド（webpack）の 2 層。

```
special-relativity-web/
├── Cargo.toml / Cargo.lock     # Rust workspace（root crate special-relativity-rs + crates/*）
├── src/                        # root crate のソース（app / charge_set / key / player / lib）
├── crates/                     # 分割された Rust crate（責務はソース参照）
│   ├── rmath/                  #   数学ユーティリティ
│   ├── shape/                  #   形状・ジオメトリ
│   ├── color/                  #   配色
│   └── backend/                #   描画 backend
├── index.ts / index.html       # Web エントリポイント
├── webpack.config.ts           # webpack 設定
├── package.json                # npm scripts（build / serve / deploy）
├── tsconfig.json
├── CLAUDE.md / SESSION.md / DESIGN.md   # 作業マニュアル / 作業状態 / 設計判断
└── README.md                   # 外部訪問者向けの玄関（ビルド手順の正本もここ）
```

各 crate の責務境界・アーキテクチャの設計判断は [`DESIGN.md`](DESIGN.md)、実装詳細は各ソース。

## ビルド・実行

前提: Rust toolchain + [`wasm-pack`](https://rustwasm.github.io/wasm-pack/) + Node.js。

**ビルド・開発サーバー・GitHub Pages デプロイのコマンドは [`README.md`](README.md) §How to build が正本**（外部 contributor 向けの玄関に集約）。要点だけ再掲すると、開発は `wasm-pack build` → `npm run serve`、本番は `npm run build`、デプロイは `npm run deploy`（`dev` ブランチへ）。

## How to Resume
1. [`SESSION.md`](SESSION.md) を読む → 現在の作業状態
2. 設計判断が要れば [`DESIGN.md`](DESIGN.md)
3. ソース構造は本ファイル §構造、ビルドは README §How to build
4. 変更後は commit + push（共有リポなので push 前に `git pull` で同期。force push 禁止）

## 規約・注意
- 全リポ共通の作業規約（odakin 環境）は `~/Claude/CONVENTIONS.md`（claude-config）。
- **共有 OSS リポ**なので collaborator の流儀・既存構成を尊重する。大きな構造変更は owner と調整してから。
