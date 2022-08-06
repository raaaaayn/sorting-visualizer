# Sorting Visualiser

[![CI][build-badge]][build-url]

![image](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)
![image](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=WebAssembly&logoColor=white)
![image](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

_Sorting Algorithm visualiser built using WebAssembly and React 👀_

[![merge-sort]](https://user-images.githubusercontent.com/12910777/183264929-33554bc5-1201-4c24-ac6f-a5a275f2696e.mp4)


<!-- ## [Demo 💥](https://mkosir.github.io/react-parallax-tilt) -->

## Why wasm?

- Rust 💕
- Runs **blazingly fast** 🚀
- Lightweight

## Development

Make sure to have the rust toolchain [installed](https://rustup.rs/)

##### Install wasm-pack:

```shields
$ cargo install wasm-pack
```

#### 1. Build the wasm:

```sh
$ wasm-pack build --target web --out-dir ui/pkg
```

#### 2. Start React app:

```sh
cd ui && npm i && npm start
```

**Start coding!** 🎉

## Contributing

All contributions are welcome!

[npm-url]: https://www.npmjs.com/package/react-parallax-tilt
[npm-badge]: https://img.shields.io/npm/v/react-parallax-tilt.svg
[size-badge]: https://badgen.net/bundlephobia/minzip/react-parallax-tilt
[downloads-badge]: https://img.shields.io/npm/dm/react-parallax-tilt.svg?color=blue
[build-badge]: https://github.com/mkosir/react-parallax-tilt/actions/workflows/build.yml/badge.svg
[build-url]: https://github.com/mkosir/react-parallax-tilt/actions/workflows/build.yml
[coverage-badge]: https://codecov.io/gh/mkosir/react-parallax-tilt/branch/main/graph/badge.svg
[coverage-url]: https://codecov.io/gh/mkosir/react-parallax-tilt
[issues-badge]: https://img.shields.io/github/issues/mkosir/react-parallax-tilt
[issues-url]: https://github.com/mkosir/react-parallax-tilt/issues
[semantic-badge]: https://img.shields.io/badge/%20%20%F0%9F%93%A6%F0%9F%9A%80-semantic--release-e10079.svg
[semantic-url]: https://github.com/semantic-release/semantic-release
[typescript-badge]: https://badges.frapsoft.com/typescript/code/typescript.svg?v=101
[typescript-url]: https://github.com/microsoft/TypeScript
