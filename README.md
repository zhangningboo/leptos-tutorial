## [教程](https://book.leptos.dev/)

## 项目启动
```shell
$ cargo add leptos --features=csr
$ rustup target add wasm32-unknown-unknown
$ trunk serve --open
```

## [开发配置](https://book.leptos.dev/getting_started/leptos_dx.html)
```shell
$ cargo add console_error_panic_hook
# main.rs
# console_error_panic_hook::set_once();
$ cargo install leptosfmt
# leptosfmt ./**/*.rs
$ # tool: https://github.com/leptos-rs/awesome-leptos
```