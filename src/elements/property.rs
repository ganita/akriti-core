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

use super::{InheritedProps, StyleProps, Element, Mstyle};
use ::platform::Context;

pub type InheritedPropReader<T> = fn(inherited: &InheritedProps) -> &T;
pub type ComputedPropComputer<T, U: Element> = fn(context: &Context, element: &U,
                                                  parent: &Option<&Element>) -> Option<T>;
pub type StylePropReader<T> = fn(style: &StyleProps) -> Option<&T>;

pub type DefaultProp<T> = fn() -> T;

pub enum Property<T: Clone, U: Element> {
    Inherited { reader: InheritedPropReader<T> },

    Computed { default: DefaultProp<T>, computer: ComputedPropComputer<T, U>, reader: StylePropReader<T> },

    Specified { default: DefaultProp<T>, reader: StylePropReader<T> },
}

impl<T: Clone, U: Element> Property<T, U> {
    pub fn calculate(&self, context: &Context, element: &U, specified: Option<&T>, parent: &Option<&Element>,
                     inherited: &InheritedProps, style: &Option<&StyleProps>) -> T {

        // Specified value always have the highest priority
        if let Some(specified) = specified {
            return specified.clone();
        }

        match *self {
            // Inherited props will always have a default value in InheritedProps struct.
            // If no value is specified in the element, we use the default value.
            Property::Inherited { ref reader, .. } => {
                return reader(inherited).clone();
            },

            // Computed props will have priority:
            // Specified value > Value in enclosing Mstyle > Computed value > Default value
            // Value that is available with highest priority will be used
            Property::Computed { ref default, ref computer, ref reader, .. } => {
                if let Some(parent) = *parent {
                    if parent.type_info().is_mstyle() {
                        let mstyle: &Mstyle = parent.as_any().downcast_ref::<Mstyle>().unwrap();
                        let style_props = mstyle.get_props();
                        if let Some(val) = reader(style_props) {
                            return val.clone();
                        }
                    }
                }

                if let Some(val) = computer(context, element, parent) {
                    return val;
                }

                return default();
            },

            // Specified props have priority :
            // Specified value > Value is style props (may or may not be direct) > Default value
            Property::Specified { ref default, ref reader, .. } => {
                if let Some(style) = *style {
                    if let Some(val) = reader(style) {
                        return val.clone();
                    }
                }

                return default();
            }
        }
    }
}