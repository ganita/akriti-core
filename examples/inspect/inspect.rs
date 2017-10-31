/*
 * Copyright 2017 Sreejith Krishnan R
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/


pub extern crate cairo;
pub extern crate gtk;
pub extern crate akriti_core;
pub extern crate akriti_measure;
pub extern crate freetype_sys;

mod canvas;
mod text_ruler;
mod math_ruler;
mod platform;

use gtk::prelude::*;
use gtk::DrawingArea;

use cairo::{Context, FontSlant, FontWeight};
use platform::GTKPlatform;
use canvas::CairoCanvas;
use akriti_core::paint::{Point};
use akriti_core::platform::Context as AkritiContext;
use akriti_core::elements::{MiElement, MrowElement};
use akriti_core::props::{MathVariant, Directionality, Color};
use akriti_core::elements::Element;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let platform = GTKPlatform::new();

    let context = AkritiContext::new(Box::new(platform), 64.);

    let mut mrow = MrowElement::new(Directionality::LTR, Color::transparent());
    mrow.add_element(
        Box::new(MiElement::new(
            String::from("i"),
            MathVariant::Normal,
            64.,
            Directionality::LTR,
            Color::RGB(0, 0, 0),
            Color::RGB(0, 255, 0),
        ))
    );
    mrow.add_element(
        Box::new(MiElement::new(
            String::from(" + "),
            MathVariant::Normal,
            64.,
            Directionality::LTR,
            Color::RGB(0, 0, 0),
            Color::RGB(0, 255, 0),
        ))
    );
    mrow.add_element(
        Box::new(MiElement::new(
            String::from("j"),
            MathVariant::Normal,
            64.,
            Directionality::LTR,
            Color::RGB(0, 0, 0),
            Color::RGB(0, 255, 0),
        ))
    );

    drawable(500, 500, move |_, cr| {
        cr.select_font_face("STIX Two Math", FontSlant::Normal, FontWeight::Normal);

        let drawing = mrow.layout(&context);

        let canvas = CairoCanvas::new(cr);
        cr.scale(1., 1.);

        drawing.draw(&canvas, &Point::new(0., 0.));

        Inhibit(false)
    });

    gtk::main();
}

pub fn drawable<F>(width: i32, height: i32, draw_fn: F)
        where F: Fn(&DrawingArea, &Context) -> Inhibit + 'static {
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    let drawing_area = Box::new(DrawingArea::new)();

    drawing_area.connect_draw(draw_fn);

    window.set_default_size(width, height);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    window.add(&drawing_area);
    window.set_keep_above(true);
    window.show_all();
}
