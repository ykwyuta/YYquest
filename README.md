# 残響のアルカディア（仮）

ファミコン時代のコマンド式 RPG を目指す Rust プロジェクトです。最初の試遊版「辺境の残響」では、アレンが黄昏の辺境領ノクスの村を出て、森でリリスと出会い、北の石碑にたどり着きます。

## 遊び方

矢印キーまたは WASD で移動し、Enter、Space、Z で決定します。J でアレンの手記を開き、Esc または J で閉じます。ブラウザのタッチ端末では画面下のボタンを使えます。

村のかまどで体力を回復できます。街道を外れて草むらを歩くと魔物が出ます。戦闘では上下でコマンドを選びます。刻印魔法は大きなダメージを与え、手記に記された思い出を一つ失います。リリスが仲間になると精霊共鳴を使えます。

## ワールドマップ

タイトル画面、フィールド探索中、序章クリア画面で M を押すと、環状列島ヴェルデリアの全体図を開けます。矢印キーや WASD で島を選び、Enter、Space、Z で上陸します。ブラウザのタッチ端末では「地図」「決定」を使えます。

上陸すると、その島の港から 312×192 タイルのフィールドを歩けます。一画面に 24×14 タイルが表示されます。各島では魔導列車が線路を、馬車が街道を行き交います。五つの島それぞれに、港町を含む街・村が10か所、廃村・廃墟が3か所あります。それぞれ専用の複数タイルにまたがるアイコンで表示し、入口の目印まで歩くと名前と説明が表示されます。M または Esc で全体図へ戻れます。港では決定キーでも全体図へ戻り、次の島を選べます。各島の野生動物10種・魔獣5種は島ごとに異なります。動物115匹と魔獣20体が、沿岸部や内陸の生息地の近くを歩き回ります。魔獣に近づいて「戦う」が表示されたときは、決定キーで戦闘を始められます。

## Windows

Rust と MSVC ビルド環境で実行します。

```powershell
cargo run --locked --release --bin yyquest-win
```

## WebAssembly

Rust と Node.js を使います。ローカル配信の例は Python を使用します。

```powershell
rustup target add wasm32-unknown-unknown
node tools/build.mjs
python -m http.server 8765 --bind 127.0.0.1 --directory dist
```

[ローカルのゲーム画面](http://127.0.0.1:8765/)を開きます。タイトル画面の「キャッシュを消して再読み込み」で、ブラウザ版のゲームファイルを取り直せます。

## Cloudflare

[YYDungeon の公開手順](https://github.com/ykwyuta/YYDungeon#cloudflare-配信)と同様、Cloudflare Workers の Static Assets で `dist` を配信します。Workers の Git 連携ではリポジトリ直下をルートに設定し、デプロイコマンドを `npx wrangler deploy` にします。`wrangler.toml` に指定した `bash build.sh` が `dist` を生成します。Pages を使う場合はビルドコマンドを `bash build.sh`、出力先を `dist` にします。

## 実装と素材

ゲーム進行と 320×240 の画面描画は Windows と WebAssembly で共有します。Windows は Win32/GDI、ブラウザは Canvas 2D に描画します。キャラクターと敵の素材の出典は [assets/README.md](assets/README.md) を参照してください。

```powershell
cargo test --locked --lib
node tools/build.mjs
node tools/wasm-smoke.mjs
```
