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

pub mod ruler;
pub mod canvas;
pub mod platform;

use std::path::Path;

use ::paint::{Point};
use ::layout::Layout;
use ::platform::Context;
use ::paint::Canvas as AkritiCanvas;
use ::draw::{Drawable, MeasureMode};

use self::platform::Platform;
use self::canvas::Canvas;

pub struct Snapshot {
    context: Context,
}

impl Snapshot {
    pub fn new(font: &str, snaps_dir: &str, font_size: f32) -> Snapshot {
        Snapshot {
            context: Context::new(Box::new(Platform::new(font)), font_size),
        }
    }

    pub fn snap_layout(&self, layout: &Layout, name: &str) {
        let layout = layout.layout(&self.context);

        let canvas: Canvas = self.context.platform().as_any().downcast_ref::<Platform>().unwrap()
            .new_canvas(layout.bounding_box().width(), layout.bounding_box().height());

        layout.draw(&canvas, &Point::new(0., 0.));

        canvas.as_any().downcast_ref::<Canvas>().unwrap().snapshot(Path::new("target")
            .join(format!("{}.png", name)).as_ref()).expect("Cannot snap");
    }

    pub fn snap_drawable(&self, drawable: &mut Drawable, width_mode: &MeasureMode,
                         height_mode: &MeasureMode, name: &str) {
        drawable.calculate(&self.context, width_mode, height_mode);

        let canvas: Canvas = self.context.platform().as_any().downcast_ref::<Platform>().unwrap()
            .new_canvas(drawable.bounding_box().width(), drawable.bounding_box().height());

        drawable.draw(&canvas, &Point::new(0., 0.));

        canvas.as_any().downcast_ref::<Canvas>().unwrap().snapshot(Path::new("target")
            .join(format!("{}.png", name)).as_ref()).expect("Cannot snap");
    }
}

impl Default for Snapshot {
    fn default() -> Snapshot {
        let root = env!("CARGO_MANIFEST_DIR");
        Snapshot::new(
            &format!("{}/src/test/assets/STIX2Math.otf", root),
            &format!("{}/target", root),
            64.
        )
    }
}