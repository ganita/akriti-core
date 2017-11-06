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

use ::props::{
    MathVariant,
    Directionality,
    Color,
    Length,
    LineBreak,
    LineBreakStyle,
    IndentAlign,
    IndentAlignFirstLast,
    IndentShiftFirstLast,
    IdRef
};
use super::{Element, ConcreteElement, TokenElement};
use ::draw::{PaddingBox, Drawable, Symbol, MeasureMode};
use ::platform::Context;

pub struct MoElement {
    lspace: f32,
    rspace: f32,
    stretchy: bool,
    symmetric: bool,
    max_size: f32,
    min_size: f32,
    large_op: bool,
    movable_limits: bool,
    accent: bool,

    linebreak: LineBreak,
    line_leading: f32,
    linebreak_style: LineBreakStyle,
    linebreak_mult_char: String,

    indent_align: IndentAlign,
    indent_shift: f32,
    indent_target: Option<IdRef>,
    indent_align_first: IndentAlignFirstLast,
    indent_shift_first: IndentShiftFirstLast,
    indent_align_last: IndentAlignFirstLast,
    indent_shift_last: IndentShiftFirstLast,

    token_element: TokenElement,
}

impl Element for MoElement {
    fn layout<'a>(&'a self, context: &Context) -> Box<Drawable + 'a> {
        Box::new(ConcreteElement::layout(self, context))
    }
}

impl<'a> ConcreteElement<'a, PaddingBox<'a, MoElement, Symbol<'a, MoElement>>> for MoElement {
    fn layout(&'a self, context: &Context) -> PaddingBox<'a, MoElement, Symbol<'a, MoElement>> {
        let mut padding_box = PaddingBox::new(
            self,
            |s| s.lspace,
            |s| s.rspace,
            |_| 0.,
            |_| 0.,
        );

        let symbol = Symbol::new(
            self,
            |s| &s.token_element.text,
            |s| &s.token_element.math_variant,
            |s| s.symmetric,
            |s| s.token_element.math_size,
            |s| s.max_size,
            |s| s.min_size,
            |s| &s.token_element.dir,
            |s| &s.token_element.presentation_element.math_color,
        );

        padding_box.wrap(symbol);

        padding_box.calculate(context, &MeasureMode::Wrap, &MeasureMode::Wrap);

        padding_box
    }
}

impl MoElement {
    pub fn new(
        text: String,
        math_variant: MathVariant,
        math_size: f32,
        dir: Directionality,
        math_color: Color,

        lspace: f32,
        rspace: f32,
        stretchy: bool,
        symmetric: bool,
        max_size: f32,
        min_size: f32,
        large_op: bool,
        movable_limits: bool,
        accent: bool,

        linebreak: LineBreak,
        line_leading: f32,
        linebreak_style: LineBreakStyle,
        linebreak_mult_char: String,

        indent_align: IndentAlign,
        indent_shift: f32,
        indent_target: Option<IdRef>,
        indent_align_first: IndentAlignFirstLast,
        indent_shift_first: IndentShiftFirstLast,
        indent_align_last: IndentAlignFirstLast,
        indent_shift_last: IndentShiftFirstLast,

        math_background: Color,
    ) -> MoElement {
        MoElement {
            lspace,
            rspace,
            stretchy,
            symmetric,
            max_size,
            min_size,
            large_op,
            movable_limits,
            accent,
            linebreak,
            line_leading,
            linebreak_style,
            linebreak_mult_char,
            indent_align,
            indent_shift,
            indent_target,
            indent_align_first,
            indent_shift_first,
            indent_align_last,
            indent_shift_last,
            token_element: TokenElement::new(
                text,
                math_variant,
                math_size,
                dir,
                math_color,
                math_background,
            )
        }
    }
}