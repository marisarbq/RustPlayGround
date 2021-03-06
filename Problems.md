
## rust-analyzer failed to load workspace: Failed to find sysroot for Cargo.toml file

see https://github.com/rust-lang/rust-analyzer/issues/4172

> try `rustup component add rust-src --toolchain nightly`

## error: toolchain 'nightly-x86_64-unknown-linux-gnu' is not installed

```shell
rustup install nightly
```

## Do you have RUST_SRC_PATH set?
- check sets
```shell
echo $RUST_SRC_PATH
```
see https://github.com/rust-lang/rust-analyzer/issues/6264
- 如何设置 RUST_SRC_PATH （中文）
https://segmentfault.com/q/1010000010715376
```shell
# 查看安装版本位置
rustc --print sysroot # 我自己的 /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu
# /root/.rustup/toolchains/stable-x86_64-unknown-linux-gnu
# 
```

可以将下面写入.bashrc [如何设置环境变量 - Ubutun](https://blog.csdn.net/White_Idiot/article/details/78253004)
```bash
# Rust
export CARGO_HOME="$HOME/.cargo/"
export RUSTBINPATH="$HOME/.cargo/bin"
export RUST="$HOME/.rustup/toolchains/stable-x86_64-unknown-linux-gnu"
export RUST_SRC_PATH="$RUST/lib/rustlib/src/rust/library"
export RUST_SRC="$RUST/lib/rustlib/src/rust/src"
export RUST_BACKTRACE=1
export PATH=$PATH:$RUSTBINPATH
```
https://github.com/rust-lang/rust-analyzer/issues/6020 关于新旧版本src和lib的路径变化导致的问题

日本博主的解决方案
https://zenn.dev/yajamon/articles/be689814d242f8
```bash
ln -s /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src
ln -s /root/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library /root/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src
```

错误方法：
<!-- 最后在插件里面 选择这个就可以解决插件不去自动寻找路径，但是依然存在问题。
Rust-analyzer › Cargo: No Sysroot
[ok] Internal config for debugging, disables loading of sysroot crates.  -->

最后如果是版本问题，建议离线安装最新版本的插件。因为rust一直在更新，所以插件也必须同步保持最新才能避免更多问题。

最后记得重启Code Server!

## Code Server 如何安装vsix插件离线文件
https://stackoverflow.com/questions/64022855/how-do-you-install-an-extension-into-the-vs-code-server-thats-the-remote-end-of

## 关于Rust-analyzer
https://learnku.com/articles/32052?order_by=vote_count&

## 关于Yew 开发Web
> 试了下Yew，感觉还是开发体验没有想象那么好，不过静观其变吧。说不定在webgpu有起色对性能有追求的时候会机遇。

## 如果 trunk 运行报错很多内容
1. 尝试删除 trunk运行目录下的 dist缓存
2. 删除 工作空间的target构建目录
3. 检查是否有不支持wasm构建的 rust Cargo。比如 takio 就不支持


## Axum跨域问题
1. Router的 layer上加上一个组件
```toml
tower-http = { version = "0.3.0", features = ["cors"] }
```
```rust
use tower_http::cors::CorsLayer;

.layer(
    // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
    // for more details
    //
    // pay attention that for some request types like posting content-type: application/json
    // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
    // or see this issue https://github.com/tokio-rs/axum/issues/849
    CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET]),
);
```

## Stream split问题
添加Cargo包
```toml
futures = "0.3.21"
```

## 多线程安全引用 Arc机制
https://blog.csdn.net/quicmous/article/details/123053447

https://beachboyy.blog.csdn.net/article/details/113814516?spm=1001.2101.3001.6661.1&utm_medium=distribute.pc_relevant_t0.none-task-blog-2%7Edefault%7ECTRLIST%7Edefault-1-113814516-blog-123053447.pc_relevant_multi_platform_whitelistv2&depth_1-utm_source=distribute.pc_relevant_t0.none-task-blog-2%7Edefault%7ECTRLIST%7Edefault-1-113814516-blog-123053447.pc_relevant_multi_platform_whitelistv2&utm_relevant_index=1
