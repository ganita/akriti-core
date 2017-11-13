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

use std::any::Any;

use super::super::{Layout, PresentationLayout, ConcreteLayout};
use ::draw::{Drawable, BoundingBox, MeasureMode};
use ::platform::Context;
use ::paint::{Canvas, Point, MathRuler, Rect};
use ::props::{DisplayStyle, MathSize};

pub struct MmultiscriptLayout {
    pub(crate) base_layout: Box<Layout>,
    pub(crate) prescript_layout: Vec<(Box<Layout>, Box<Layout>)>,
    pub(crate) postscript_layout: Vec<(Box<Layout>, Box<Layout>)>,
    pub(crate) subscript_shift: f32,
    pub(crate) superscript_shift: f32,

    pub(crate) presentation_layout: PresentationLayout,
}

impl Layout for MmultiscriptLayout {
    fn layout<'a>(&'a self, context: &Context) -> Box<Drawable + 'a> {
        let mut wrapper = self.presentation_layout.layout(context);

        wrapper.wrap(MultiscriptDrawable {
            base: self.base_layout.layout(context),
            base_pos: Point::new(0f32, 0f32),
            prescripts: self.prescript_layout.iter().map(|&(ref superscript, ref subscript)| {
                MultiscriptDrawableChild {
                    subscript: subscript.layout(context),
                    subscript_pos: Point::new(0f32, 0f32),
                    superscript: superscript.layout(context),
                    superscript_pos: Point::new(0f32, 0f32),
                }
            }).collect(),
            postscripts: self.postscript_layout.iter().map(|&(ref superscript, ref subscript)| {
                MultiscriptDrawableChild {
                    subscript: subscript.layout(context),
                    subscript_pos: Point::new(0f32, 0f32),
                    superscript: superscript.layout(context),
                    superscript_pos: Point::new(0f32, 0f32),
                }
            }).collect(),
            superscript_shift: self.superscript_shift,
            subscript_shift: self.subscript_shift,
            display_style: self.presentation_layout.display_style,
            base_size: self.presentation_layout.script_level.get_font_size(context, &MathSize::NORMAL),
            bounding_box: BoundingBox::default(),
        });

        wrapper.calculate(context, &MeasureMode::Wrap, &MeasureMode::Wrap);

        Box::new(wrapper)
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
}


struct MultiscriptDrawable<'a> {
    base: Box<Drawable + 'a>,
    base_pos: Point,

    prescripts: Vec<MultiscriptDrawableChild<'a>>,
    postscripts: Vec<MultiscriptDrawableChild<'a>>,

    superscript_shift: f32,
    subscript_shift: f32,
    display_style: DisplayStyle,
    base_size: f32,

    bounding_box: BoundingBox,
}

struct MultiscriptDrawableChild<'a> {
    subscript: Box<Drawable + 'a>,
    subscript_pos: Point,
    superscript: Box<Drawable + 'a>,
    superscript_pos: Point,
}

struct ScriptPosition {
    base_baseline_pos: f32,
    superscript_baseline_pos: f32,
    subscript_baseline_pos: f32
}

impl ScriptPosition {
    fn max(&self, rhs: &ScriptPosition) -> ScriptPosition {
        ScriptPosition {
            base_baseline_pos: self.base_baseline_pos.max(rhs.base_baseline_pos),
            superscript_baseline_pos: self.superscript_baseline_pos.max(rhs.superscript_baseline_pos),
            subscript_baseline_pos: self.subscript_baseline_pos.max(rhs.subscript_baseline_pos),
        }
    }
}

impl<'a> MultiscriptDrawable<'a> {
    fn find_script_y_pos(&self, child: &MultiscriptDrawableChild<'a>, ruler: &MathRuler) -> ScriptPosition {

        let base = &self.base;
        let subscript = &child.superscript;
        let superscript = &child.subscript;

        let has_subscript = subscript.bounding_box().width() > 0f32 || subscript.bounding_box().height() > 0f32;
        let has_superscript = superscript.bounding_box().width() > 0f32 || superscript.bounding_box().height() > 0f32;

        // We will fix the position of superscript first
        let superscript_y_pos = 0f32;

        let base_y_pos = if has_superscript {
            // Recommended shift between baseline of base and baseline of superscript
            let superscript_shift = if self.superscript_shift.is_nan() {
                if self.display_style {
                    ruler.superscript_shift_up()
                } else {
                    ruler.superscript_shift_up_cramped()
                }
            } else {
                self.superscript_shift
            };

            let mut base_y_pos = superscript.bounding_box().baseline_pos() +
                superscript_shift -
                base.bounding_box().baseline_pos();

            // Minimum shift between superscript bottom and baseline of base
            let superscript_bottom_min = ruler.superscript_bottom_min();

            base_y_pos = base_y_pos.max(
                superscript.bounding_box().height() +
                    superscript_bottom_min -
                    base.bounding_box().baseline_pos()
            );

            // Maximum drop of baseline of superscript from top of base
            let superscript_baseline_drop_max = ruler.superscript_baseline_drop_max();

            base_y_pos = base_y_pos.max(
                superscript.bounding_box().baseline_pos() - superscript_baseline_drop_max
            );

            if has_subscript {
                // Maximum shift between superscript bottom and baseline of base when subscript is present
                let superscript_bottom_max_with_subscript = ruler.superscript_bottom_max_with_subscript();
                base_y_pos = base_y_pos.min(
                    superscript.bounding_box().height() +
                        superscript_bottom_max_with_subscript -
                        base.bounding_box().baseline_pos()
                );
            }

            base_y_pos
        } else {
            0f32
        };

        let subscript_y_pos = if has_subscript {
            // Recommended shift between baseline of base and baseline of subscript
            let subscript_shift = if self.subscript_shift.is_nan() {
                ruler.subscript_shift_down()
            } else {
                self.subscript_shift
            };

            let mut subscript_y_pos = base_y_pos + base.bounding_box().baseline_pos() +
                subscript_shift -
                subscript.bounding_box().baseline_pos();

            // Minimum shift between baseline of subscript and bottom of base
            let subscript_baseline_drop_min = ruler.subscript_baseline_drop_min();

            subscript_y_pos = subscript_y_pos.max(
                base_y_pos + base.bounding_box().height() +
                    subscript_baseline_drop_min -
                    subscript.bounding_box().baseline_pos()
            );

            // Maximum shift between top of subscript and baseline of base
            let subscript_top_max = ruler.subscript_top_max();

            subscript_y_pos = subscript_y_pos.max(
                base_y_pos +
                    base.bounding_box().baseline_pos() -
                    subscript_top_max
            );

            if has_superscript {
                // Minimum gap between bottom of superscript and top of subscript
                let sub_superscript_gap_min = ruler.sub_superscript_gap_min();

                subscript_y_pos.max(
                    superscript_y_pos +
                        superscript.bounding_box().height() +
                        sub_superscript_gap_min
                );
            }

            subscript_y_pos
        } else {
            base_y_pos + base.bounding_box().height()
        };

        return ScriptPosition {
            base_baseline_pos: base_y_pos + base.bounding_box().baseline_pos(),
            superscript_baseline_pos: superscript_y_pos + superscript.bounding_box().baseline_pos(),
            subscript_baseline_pos: subscript_y_pos + subscript.bounding_box().baseline_pos(),
        }
    }

    fn find_max_script_y_pos(&self, scripts: &Vec<MultiscriptDrawableChild<'a>>, ruler: &MathRuler) -> ScriptPosition {
        let mut script_pos = ScriptPosition {
            base_baseline_pos: 0.0,
            superscript_baseline_pos: 0.0,
            subscript_baseline_pos: 0.0,
        };

        for prescript in scripts.iter() {
            let current_script_pos =
                self.find_script_y_pos(prescript, ruler);

            script_pos = script_pos.max(&current_script_pos);
        }

        return script_pos;
    }

    fn draw_scripts(&self, scripts: &Vec<MultiscriptDrawableChild<'a>>, canvas: &Canvas, pen_pos: &Point) {
        for script in scripts.iter() {
            script.superscript.draw(canvas, &(pen_pos+&script.superscript_pos));
            script.subscript.draw(canvas, &(pen_pos+&script.subscript_pos));
        }
    }

    fn set_script_positions(scripts: &mut Vec<MultiscriptDrawableChild<'a>>,
                            script_pos: &ScriptPosition, current_pen_x: f32) -> Rect {
        let mut pen_x = current_pen_x;
        let mut y_max = 0f32;
        for script in scripts.iter_mut() {
            let width = script.subscript.bounding_box().width()
                .max(script.superscript.bounding_box().width());

            script.subscript_pos = Point::new(
                pen_x+width-script.subscript.bounding_box().width(),
                script_pos.subscript_baseline_pos-script.subscript.bounding_box().baseline_pos());

            script.superscript_pos = Point::new(
                pen_x+width-script.superscript.bounding_box().width(),
                script_pos.superscript_baseline_pos-script.superscript.bounding_box().baseline_pos());

            pen_x += width;
            y_max = y_max.max(script_pos.subscript_baseline_pos+script.superscript.bounding_box().baseline())
        }

        return Rect::new(pen_x-current_pen_x, y_max);
    }
}

impl<'a> Drawable for MultiscriptDrawable<'a> {
    fn draw(&self, canvas: &Canvas, pen_pos: &Point) {
        self.draw_scripts(&self.prescripts, canvas, pen_pos);
        self.base.draw(canvas, &(pen_pos+&self.base_pos));
        self.draw_scripts(&self.postscripts, canvas, pen_pos);
    }

    fn calculate(&mut self, context: &Context, width_mode: &MeasureMode, height_mode: &MeasureMode) {
        self.base.calculate(context, width_mode, height_mode);

        let ruler = context.platform().get_math_ruler(self.base_size);

        let script_pos = self.find_max_script_y_pos(&self.prescripts, ruler)
            .max(&self.find_max_script_y_pos(&self.postscripts, ruler));

        let subscript_baseline_pos = script_pos.subscript_baseline_pos;

        let base_baseline_pos = script_pos.base_baseline_pos
            .max(self.base.bounding_box().baseline_pos());

        let superscript_baseline_pos = script_pos.superscript_baseline_pos;

        let mut pen_x = 0f32;

        let prescript_bounds = MultiscriptDrawable::set_script_positions(
            &mut self.prescripts, &script_pos, pen_x);
        pen_x += prescript_bounds.width();

        self.base_pos = Point::new(pen_x, base_baseline_pos-self.base.bounding_box().baseline_pos());
        pen_x += self.base.bounding_box().width();

        let postscript_bounds = MultiscriptDrawable::set_script_positions(
            &mut self.postscripts, &script_pos, pen_x);
        pen_x += postscript_bounds.width();

        let height = prescript_bounds.height()
            .max(postscript_bounds.height())
            .max(self.base.bounding_box().height());

        self.bounding_box = BoundingBox::new(
            Rect::new(pen_x, height),
            height - (base_baseline_pos),
            height - (self.base.bounding_box().axis_pos() - self.base.bounding_box().baseline_pos() + base_baseline_pos),
        );
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
}