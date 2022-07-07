# RustPlayGround
Learn Rust, Take The Future


### Env

> I use CodeServer or VsCode 

#### Vsix Plugins

Tools
- rust-analyzer @v0.4.1116 **Important!**
- Native Debug @v0.26.0
- Better TOML @v0.3.2
- Error Lens @v3.5.1
- CodeLLDB @v1.7.0

Theme
- One Dark Pro


#### Build Release
```bash
cargo build --package [cargoname: webserver] --release
```


#### crates: Rust Library
> 主要是rust示例仓库

#### front: Rust Yew Front Page
> 主要是rust Yew或者一些其他rust web前端框架的库，因为特殊性，和其他rust库分开了。
```bash
trunk serve --port 3003 --address 0.0.0.0
```