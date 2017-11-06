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


use ::props::*;
use ::layout::{MiLayout, Layout};
use ::draw::*;
use ::platform::*;

pub struct Mi {
    text: String,
    math_variant: Option<MathVariant>,
    dir: Option<Directionality>,
    math_size: Option<MathSize>,

    math_color: Option<Color>,
    math_background: Option<Color>,

    element: Option<MiLayout>,
}

impl Mi {
    pub fn new(text: String) -> Mi {
        Mi {
            text,
            math_variant: None,
            dir: None,
            math_size: None,

            math_color: None,
            math_background: None,

            element: Some(MiLayout::new (
                String::from("Test"),
                MathVariant::Normal,
                64.,
                Directionality::LTR,
                Color::RGB(0, 0, 0),
                Color::transparent()
            )),
        }
    }

    pub fn with_text(mut self, text: String) -> Mi {
        self.text = text;
        self
    }

    pub fn with_dir(mut self, dir: Option<Directionality>) -> Mi {
        self.dir = dir;
        self
    }

    pub fn with_math_size(mut self, math_size: Option<MathSize>) -> Mi {
        self.math_size = math_size;
        self
    }

    pub fn with_math_color(mut self, math_color: Option<Color>) -> Mi {
        self.math_color = math_color;
        self
    }

    pub fn with_math_background(mut self, math_background: Option<Color>) -> Mi {
        self.math_background = math_background;
        self
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn get_dir(&self) -> Option<&Directionality> {
        self.dir.as_ref()
    }

    pub fn get_math_size(&self) -> Option<&MathSize> {
        self.math_size.as_ref()
    }

    pub fn get_math_color(&self) -> Option<&Color> {
        self.math_color.as_ref()
    }

    pub fn get_math_background(&self) -> Option<&Color> {
        self.math_background.as_ref()
    }

    pub fn layout<'a>(&'a mut self, context: &Context) -> Box<Drawable + 'a> {
        self.element.as_ref().unwrap().layout(context)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_has_good_api() {
        let mi = Mi::new(String::from("Hello world!"))
            .with_dir(Some(Directionality::RTL))
            .with_math_color(Some(Color::RGB(0, 0, 0)));
    }
}