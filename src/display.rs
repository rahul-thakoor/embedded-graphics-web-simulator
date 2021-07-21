use core::marker::PhantomData;
use crate::output_settings::OutputSettings;
use embedded_graphics::{
    primitives,
    // drawable::Pixel,
    geometry::Size,
    pixelcolor::{PixelColor, Rgb888},
    prelude::*,
    // DrawTarget,
};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlCanvasElement;

/// WebSimulator display.
pub struct WebSimulatorDisplay<C> {
    size: Size,
    canvas: HtmlCanvasElement,
    output_settings: OutputSettings,
    _color_type: PhantomData<C>,
}

impl<C> WebSimulatorDisplay<C>
where
    C: PixelColor + Into<Rgb888>
{
    /// Creates a new display.
    ///
    /// This appends a `<canvas>` element with size corresponding to scale and pixel spacing used
    /// The display is filled with black.
    pub fn new(size: (u32, u32), output_settings: &OutputSettings) -> Self {
        // source:https://github.com/jamwaffles/embedded-graphics/blob/master/simulator/src/output_settings.rs#L27
        let width = size.0 * output_settings.scale + (size.0 - 1) * output_settings.pixel_spacing;
        // source:https://github.com/jamwaffles/embedded-graphics/blob/master/simulator/src/output_settings.rs#L28
        let height = size.1 * output_settings.scale + (size.1 - 1) * output_settings.pixel_spacing;
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.create_element("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        canvas.set_width(width);
        canvas.set_height(height);
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.set_fill_style(&JsValue::from_str("black"));
        context.fill_rect(0.0, 0.0, width as f64, height as f64);
        let body = document.body().expect("document should have a body");

        body.append_child(&canvas)
            .expect("couldn't append canvas to body");

        WebSimulatorDisplay {
            size: Size::new(width, height),
            canvas,
            output_settings: output_settings.clone(),
            _color_type: PhantomData,
        }
    }

    fn draw_pixel(&mut self, pixel: Pixel<C>) -> Result<(), core::convert::Infallible> {
        let Pixel(coord, color) = pixel;

        let context = self
            .canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        let color_rgb888 = color.into();

        let css_color = format!(
            "rgb({},{},{})",
            color_rgb888.r(),
            color_rgb888.g(),
            color_rgb888.b()
        );
        context.set_fill_style(&JsValue::from_str(&css_color));
        // source: https://github.com/jamwaffles/embedded-graphics/blob/master/simulator/src/output_settings.rs#L40
        let pitch = (self.output_settings.scale + self.output_settings.pixel_spacing) as i32;
        context.fill_rect(
            (coord.x * pitch) as f64,
            (coord.y * pitch) as f64,
            self.output_settings.scale as f64,
            self.output_settings.scale as f64,
        );

        Ok(())
    }
}

impl<C> OriginDimensions for WebSimulatorDisplay<C>
where
    C: PixelColor + Into<Rgb888>
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
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let bounding_box =
            primitives::Rectangle::new(Point::new(0, 0), self.size);
        for pixel in pixels.into_iter() {
            if bounding_box.contains(pixel.0) {
                self.draw_pixel(pixel)?;
                // self.draw_point((coord.x as i16, coord.y as i16), color.into_storage())?;
            }
        }
        Ok(())
    }

}
