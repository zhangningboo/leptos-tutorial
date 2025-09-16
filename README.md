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

## tailwindcss
- tailwindcss v4.1.13 20250912
```shell
$ npm install tailwindcss @tailwindcss/cli
$ npm view tailwindcss version
$ npm view @tailwindcss/cli version
$ npx tailwindcss -o styles/tailwindv4.1.13.css
$ npx @tailwindcss/cli -i styles/index.css -o styles/tailwindv4.1.13.css --watch  # 实时监测项目中的css改动，实时更新需要的css
```