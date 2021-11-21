use crate::output_settings::OutputSettings;
use core::marker::PhantomData;
use embedded_graphics::{
    geometry::Size,
    pixelcolor::{PixelColor, Rgb888},
    prelude::*,
    primitives::{self, Rectangle},
};
use std::{convert::TryInto, error::Error};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, HtmlCanvasElement};

/// WebSimulator display.
pub struct WebSimulatorDisplay<C> {
    size: Size,
    canvas: HtmlCanvasElement,
    output_settings: OutputSettings,
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

        WebSimulatorDisplay {
            size: Size::new(width, height),
            canvas,
            output_settings: output_settings.clone(),
            _color_type: PhantomData,
        }
    }

    fn fill_rect(
        canvas: &HtmlCanvasElement,
        color: C,
        area: &Rectangle,
        scale: u32,
        pitch: u32,
    ) -> Result<(), Box<dyn Error>> {
        let context = canvas
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

        let width = area.size.width * scale;
        let height = area.size.height * scale;

        let scale: i32 = scale.try_into()?;
        let pitch: i32 = pitch.try_into()?;

        let origin = area.top_left;

        context.fill_rect(
            (origin.x * scale * pitch).try_into()?,
            (origin.y * scale * pitch).try_into()?,
            width.try_into()?,
            height.try_into()?,
        );
        Ok(())
    }

    fn draw_pixel(&mut self, pixel: Pixel<C>) -> Result<(), core::convert::Infallible> {
        let Pixel(coord, color) = pixel;
        let scale = self.output_settings.scale;

        // source: https://github.com/jamwaffles/embedded-graphics/blob/master/simulator/src/output_settings.rs#L40
        let pitch = scale + self.output_settings.pixel_spacing;

        Self::fill_rect(
            &self.canvas,
            color,
            &Rectangle::new(coord, Size::new(scale, scale)),
            scale,
            pitch,
        )
        .expect("numeric conversion failed");

        Ok(())
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
        let bounding_box = primitives::Rectangle::new(Point::new(0, 0), self.size);
        for pixel in pixels.into_iter() {
            if bounding_box.contains(pixel.0) {
                self.draw_pixel(pixel)?;
            }
        }
        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        Self::fill_rect(&self.canvas, color, area, self.output_settings.scale, 1)?;

        Ok(())
    }
}
