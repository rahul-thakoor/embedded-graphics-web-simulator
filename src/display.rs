use crate::output_settings::OutputSettings;
use core::marker::PhantomData;
use embedded_graphics::{
    geometry::Size,
    pixelcolor::{PixelColor, Rgb888},
    prelude::*,
    primitives::{self},
};
use std::{convert::TryInto, error::Error};
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement, ImageData};

/// WebSimulator display.
pub struct WebSimulatorDisplay<C> {
    size: Size,
    canvas_size: Size,
    canvas: HtmlCanvasElement,
    output_settings: OutputSettings,
    backing: Vec<u8>,
    context: CanvasRenderingContext2d,
    _color_type: PhantomData<C>,
}

impl<C> WebSimulatorDisplay<C>
where
    C: PixelColor + Into<Rgb888>,
{
    /// Creates a new display.
    ///
    /// This appends a `<canvas>` element with size corresponding to scale and pixel spacing used
    /// The display is filled with black.
    pub fn new(
        size: (u32, u32),
        output_settings: &OutputSettings,
        parent: Option<&Element>,
    ) -> Self {
        // source: https://github.com/embedded-graphics/simulator/blob/master/src/output_settings.rs
        let canvas_width =
            size.0 * output_settings.scale + (size.0 - 1) * output_settings.pixel_spacing;
        // source: https://github.com/embedded-graphics/simulator/blob/master/src/output_settings.rs
        let canvas_height =
            size.1 * output_settings.scale + (size.1 - 1) * output_settings.pixel_spacing;

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.create_element("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        canvas.set_width(canvas_width);
        canvas.set_height(canvas_height);
        parent
            .unwrap_or(
                &document
                    .body()
                    .expect("document doesn't have a body and no alternative parent was supplied")
                    .dyn_into::<web_sys::Element>()
                    .map_err(|_| ())
                    .unwrap(),
            )
            .append_child(&canvas)
            .expect("couldn't append canvas to parent");
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        // source: https://github.com/embedded-graphics/simulator/blob/master/src/output_settings.rs#L39

        WebSimulatorDisplay {
            size: Size::new(size.0, size.1),
            canvas_size: Size::new(canvas_width, canvas_height),
            backing: vec![0; (4 * canvas_width * canvas_height) as usize],
            canvas,
            context,
            output_settings: output_settings.clone(),
            _color_type: PhantomData,
        }
    }
}
impl<C> OriginDimensions for WebSimulatorDisplay<C>
where
    C: PixelColor + Into<Rgb888>,
{
    fn size(&self) -> Size {
        self.size
    }
}

impl<C> DrawTarget for WebSimulatorDisplay<C>
where
    C: PixelColor + Into<Rgb888>,
{
    type Color = C;
    type Error = Box<dyn Error>;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let canvas_width = self.canvas_size.width as usize;
        let backing = self.backing.as_mut_slice();

        let scale = self.output_settings.scale as usize;

        // source: https://github.com/embedded-graphics/simulator/blob/master/src/output_settings.rs#L39
        let pitch = scale + self.output_settings.pixel_spacing as usize;

        let bounding_box = primitives::Rectangle::new(Point::new(0, 0), self.canvas_size);
        for pixel in pixels.into_iter() {
            let point = pixel.0;
            if bounding_box.contains(point) {
                let rgb: Rgb888 = pixel.1.into();
                let rgb_slice = &[rgb.r(), rgb.g(), rgb.b(), 255];
                let py = point.y as usize;
                let px = point.x as usize;

                let x_offset = px * 4 * pitch;
                for y in 0..scale {
                    let y_offset = py * 4 * canvas_width * pitch + y * 4 * canvas_width;
                    for x in 0..scale {
                        let pixel_offset = y_offset + x_offset + x * 4;
                        backing[pixel_offset..pixel_offset + 4].copy_from_slice(rgb_slice);
                    }
                }
            }
        }

        let image_data = ImageData::new_with_u8_clamped_array(
            Clamped(backing),
            self.canvas_size.width.try_into().unwrap(),
        )
        .expect("could not create ImageData");
        self.context
            .put_image_data(&image_data, 0., 0.)
            .expect("coult not put ImageData");

        Ok(())
    }
}
