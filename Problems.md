
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