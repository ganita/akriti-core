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

use super::{InheritedProps, StyleProps, Element, Mstyle, InheritedPropsCopier, Family};
use ::platform::Context;

pub type InheritedPropReader<T> = fn(inherited: &InheritedProps) -> &T;
pub type InheritedPropWriter<T> = fn(val: T, fork: &mut InheritedPropsCopier) -> &mut InheritedPropsCopier;

pub type ComputedPropComputer<T, U> = for<'a> fn(context: &Context, element: &U,
                                                  parent: &Family<'a>) -> Option<T>;
pub type StylePropReader<T> = fn(style: &StyleProps) -> Option<&T>;

pub type DefaultProp<T> = fn() -> T;

pub enum Property<T: Clone, U: Element> {
    Inherited { reader: InheritedPropReader<T>, writer: InheritedPropWriter<T> },

    Computed { default: DefaultProp<T>, computer: ComputedPropComputer<T, U>, reader: StylePropReader<T> },

    Specified { default: DefaultProp<T>, reader: StylePropReader<T> },
}

impl<T: Clone, U: Element> Property<T, U> {
    pub fn calculate<'a>(&self, context: &Context, element: &U, specified: Option<&T>, family: &Family<'a>,
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
                if let Some(parent) = family.parent() {
                    if parent.type_info().is_mstyle() {
                        let mstyle: &Mstyle = parent.as_any().downcast_ref::<Mstyle>().unwrap();
                        let style_props = mstyle.get_props();
                        if let Some(val) = reader(style_props) {
                            return val.clone();
                        }
                    }
                }

                if let Some(val) = computer(context, element, family) {
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

    pub fn is_inherited(&self) -> bool {
        if let Property::Inherited { .. } = *self { true } else { false }
    }

    pub fn is_computed(&self) -> bool {
        if let Property::Computed { .. } = *self { true } else { false }
    }

    pub fn is_specified(&self) -> bool {
        if let Property::Specified { .. } = *self { true } else { false }
    }
}


pub struct PropertyCalculator<'a, 'b, 'c, 'e, 'f, T: Element + 'b> {
    context: &'a Context,
    element: &'b T,
    family: &'c Family<'c>,
    inherited: &'e InheritedProps,
    style: Option<&'f StyleProps>,

    copier: InheritedPropsCopier,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, T: Element> PropertyCalculator<'a, 'b, 'c, 'e, 'f, T> {
    pub fn new(context: &'a Context, element: &'b T, family: &'c Family<'c>,
               inherited: &'e InheritedProps, style: Option<&'f StyleProps>
    ) -> PropertyCalculator<'a, 'b, 'c, 'e, 'f, T> {
        let copier = inherited.copier();

        PropertyCalculator {
            context,
            element,
            family,
            inherited,
            style,
            copier,
        }
    }

    pub fn calculate<U: Clone>(&mut self, property: &Property<U, T>, specified: Option<&U>) -> U {
        let val = property.calculate(self.context, self.element, specified, self.family,
                                     self.inherited, &self.style);

        if let Property::Inherited { writer, .. } = *property {
            writer(val.clone(), &mut self.copier);
        }

        val

    }

    pub fn make_fork(self) -> InheritedPropsCopier {
        self.copier
    }
}