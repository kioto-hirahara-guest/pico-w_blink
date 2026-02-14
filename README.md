# pico-w_blink

Raspberry Pi Pico WのLED点滅をRustで実装

使用前に以下のコマンドを実行しておくこと。

```
mkdir lib
cd lib
git clone https://github.com/embassy-rs/embassy.git
cd ..
```

probe-rsコマンドをインストールする。

```
cargo install probe-rs-tools
```

ビルド、インストールは以下のコマンドを実行する。

```
cargo run --release --bin wifi_blinky
```

proxyの設定は、`.cargo/config.toml`で以下の行を追加する。

```:
[http]
proxy = "http://xxx:yyyy"
```
