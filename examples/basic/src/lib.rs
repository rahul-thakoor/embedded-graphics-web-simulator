use embedded_graphics_web_simulator::{
    display::WebSimulatorDisplay, output_settings::OutputSettingsBuilder,
};
use wasm_bindgen::prelude::*;
use web_sys::console;

use embedded_graphics::{
    egcircle, egtext,
    fonts::Font6x8,
    image::Image,
    pixelcolor::{BinaryColor, Rgb565},
    prelude::*,
    primitive_style, text_style,
};
use tinybmp::Bmp;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let output_settings = OutputSettingsBuilder::new().scale(3).build();
    let mut text_display = WebSimulatorDisplay::new((128, 64), &output_settings);
    let mut img_display = WebSimulatorDisplay::new((128, 128), &output_settings);

    // Show Font using a macro, source https://github.com/jamwaffles/embedded-graphics/blob/master/simulator/examples/text-fonts.rs#L64
    egtext!(
        text = "Hello, wasm world!",
        top_left = (10, 30),
        style = text_style!(font = Font6x8, text_color = BinaryColor::On)
    )
    .draw(&mut text_display)
    .unwrap_or_else(|_| console::log_1(&"Couldn't draw text".into()));

    // Load the BMP image
    let bmp = Bmp::from_slice(include_bytes!("./assets/rust-pride.bmp")).unwrap();
    let image: Image<Bmp, Rgb565> = Image::new(&bmp, Point::new(32, 32));
    image
        .draw(&mut img_display)
        .unwrap_or_else(|_| console::log_1(&"Couldn't draw image".into()));

    let circle = egcircle!(
        center = (64, 64),
        radius = 33,
        style = primitive_style!(stroke_color = BinaryColor::On, stroke_width = 1)
    );
    circle
        .draw(&mut img_display)
        .unwrap_or_else(|_| console::log_1(&"Couldn't draw circle".into()));
    Ok(())
}
