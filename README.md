# stop-sound-timer

Rust (Yew + Tauri) 製の簡単なタイマーアプリです。  
タイマー起動後、指定時間の経過後にPC上で鳴っている音楽を止めます。

## Usage

RustおよびCargoをインストールした環境で以下を実行します。

```bash
cargo install create-tauri-app
cargo tauri build
```

`target/release`以下に`stop-sound-timer.exe`が生成されます。