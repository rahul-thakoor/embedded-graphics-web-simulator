# Embedded Graphics Web Simulator

The web simulator is based on [embedded-graphics-simulator](https://docs.rs/embedded-graphics-simulator/0.2.0/embedded_graphics_simulator/).
This is a sample project demonstrating using a `no_std` rust-embedded library with Webassembly.

The Web Simulator allows you to use a browser to test embedded-graphics code and run graphics. There is no need to install SDL and its development libraries for _running_ the project. You can see [the demo here](https://rahul-thakoor.github.io/embedded-graphics-web-simulator/).

## For Development

```sh
npm install
```

### How to run in debug mode

```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
npm start
```

### How to build in release mode

```sh
# Builds the project and places it into the `dist` folder.
npm run build
```

### How it works

Embedded Graphics Web Simulator implements [`DrawTarget`](https://docs.rs/embedded-graphics/0.6.0/embedded_graphics/prelude/trait.DrawTarget.html) for the HTML `<canvas>` element.

Usage:

```rust
// define output settings
let output_settings = OutputSettingsBuilder::new().scale(3).build();
// create a Simulator Display
let mut img_display = WebSimulatorDisplay::new((128, 128), &output_settings);

// .. create content using embedded-graphics api

```

Examples:

The project creates two displays:

1. Displays a text
2. Loads a bmp image and displays it. Also draws a circle on the canvas

Modify the `src/lib.rs` file to change that.

See [more examples](https://github.com/jamwaffles/embedded-graphics/tree/master/simulator/examples) for some inspiration.

## Credits

This project is based on the [embedded-graphics](https://github.com/jamwaffles/embedded-graphics) library by @jamwaffles

Build with [wasm-pack](https://rustwasm.github.io/wasm-pack/)
