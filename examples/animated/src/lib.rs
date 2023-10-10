use embedded_graphics::{draw_target::DrawTarget, prelude::*};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use embedded_graphics_web_simulator::{
    display::WebSimulatorDisplay, output_settings::OutputSettingsBuilder,
};

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::{Point, Primitive, WebColors},
    primitives::{Circle, PrimitiveStyle},
    Drawable,
};

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn text_container() -> web_sys::Element {
    document()
        .get_element_by_id("text")
        .expect("document should have our text container")
}

const NUM_ITER: i32 = 60;

// This function is automatically invoked after the wasm module is instantiated.
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let document = document();

    let body = document.body().expect("could not get document body");

    // for simplicity reasons, this example uses `cargo-run-wasm`, which doesn't allow
    // custom html - so it's augmented here inline. In a real project, you'd likely use `trunk` instead.
    body.set_inner_html(
        r#"
        <p id="text">A greeting from rust looks like...
        <div id="graphics"></div>
        "#,
    );
    let output_settings = OutputSettingsBuilder::new()
        .scale(3)
        .pixel_spacing(1)
        .build();
    let size = (2 * NUM_ITER) as u32;
    let mut img_display = WebSimulatorDisplay::new(
        (size, size),
        &output_settings,
        document.get_element_by_id("graphics").as_ref(),
    );

    // Here we want to call `requestAnimationFrame` in a loop, but only a fixed
    // number of times. After it's done we want all our resources cleaned up. To
    // achieve this we're using an `Rc`. The `Rc` will eventually store the
    // closure we want to execute on each frame, but to start out it contains
    // `None`.
    //
    // After the `Rc` is made we'll actually create the closure, and the closure
    // will reference one of the `Rc` instances. The other `Rc` reference is
    // used to store the closure, request the first frame, and then is dropped
    // by this function.
    //
    // Inside the closure we've got a persistent `Rc` reference, which we use
    // for all future iterations of the loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut i = 1;

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if i > NUM_ITER {
            text_container().set_text_content(Some("All done!"));

            // Drop our handle to this closure so that it will get cleaned
            // up once we return.
            let _ = f.borrow_mut().take();
            return;
        }

        // Set the body's text content to how many times this
        // requestAnimationFrame callback has fired.
        let text = format!("requestAnimationFrame has been called {} times.", i);
        text_container().set_text_content(Some(&text));

        img_display.clear(Rgb565::BLACK).expect("could not clear()");

        for j in 0..5 {
            Circle::new(
                Point::new(NUM_ITER - i - 2 * j, NUM_ITER - i - 2 * j),
                (2 * i + 4 * j) as u32,
            )
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::CSS_PINK, 1))
            .draw(&mut img_display)
            .expect("could not draw Circle");
        }
        img_display.flush().expect("could not flush buffer");

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
        i += 1;
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
