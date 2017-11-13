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
use std::cell::RefCell;

use super::super::{Element, ElementType, GeneralLayout, InheritedProps, StyleProps, Family, InstanceId,
                   Presentation, PresentationPrivate, SpecifiedPresentationProps,
                   PropertyCalculator, Property, EmptyComputeCtx, Mrow, Mo, Token};
use ::platform::Context;
use ::layout::{Layout};

#[allow(const_err)]
const PROP_OPEN: Property<String, Mfenced, EmptyComputeCtx> = Property::Specified {
    default: || String::from("("),
    reader: |i| i.mfenced_open()
};

#[allow(const_err)]
const PROP_CLOSE: Property<String, Mfenced, EmptyComputeCtx> = Property::Specified {
    default: || String::from(")"),
    reader: |i| i.mfenced_close(),
};

#[allow(const_err)]
const PROP_SEPARATORS: Property<String, Mfenced, EmptyComputeCtx> = Property::Specified {
    default: || String::from(","),
    reader: |i| i.mfenced_separators(),
};

pub struct Mfenced {
    open: Option<String>,
    close: Option<String>,
    separators: Option<String>,

    placeholders: RefCell<Mrow>,

    presentation_props: SpecifiedPresentationProps,
    instance_id: InstanceId,
}

impl Mfenced {
    pub fn new() -> Mfenced {
        let mut mrow = Mrow::new();

        // Opening fence placeholder
        mrow.with_child(Box::new(Mo::new(String::new())));

        // Content placeholder
        mrow.with_child(Box::new(Mrow::new()));

        // Closing fence placeholder
        mrow.with_child(Box::new(Mo::new(String::new())));

        Mfenced {
            open: None,
            close: None,
            separators: None,
            placeholders: RefCell::new(mrow),

            presentation_props: SpecifiedPresentationProps::default(),
            instance_id: InstanceId::new(),
        }
    }

    pub fn with_child<'a>(&'a mut self, child: Box<Element>) -> &'a mut Mfenced {
        // Scope for mutable borrow of self
        {
            let content = &mut self.placeholders.get_mut().children_mut()[1];
            let content: &mut Mrow = content.as_any_mut().downcast_mut::<Mrow>().unwrap();

            if content.children().len() > 0 {
                // Separator placeholder
                content.with_child(Box::new(Mo::new(String::new())));
            }
            content.with_child(child);
        }
        self
    }
    
    pub fn with_open<'a>(&'a mut self, fence: Option<String>) -> &'a Mfenced {
        self.open = fence;
        self
    }
    
    pub fn open(&self) -> Option<&String> {
        self.open.as_ref()
    }

    pub fn with_close<'a>(&'a mut self, fence: Option<String>) -> &'a Mfenced {
        self.close = fence;
        self
    }

    pub fn close(&self) -> Option<&String> {
        self.close.as_ref()
    }

    pub fn with_separators<'a>(&'a mut self, separators: Option<String>) -> &'a Mfenced {
        self.separators = separators;
        self
    }

    pub fn separators(&self) -> Option<&String> {
        self.separators.as_ref()
    }
}

impl Element for Mfenced {
    fn layout<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
                  style: &Option<&StyleProps>) -> Box<Layout> {
        let mut calculator = PropertyCalculator::new(
            context, self, family, inherited, style.clone());

        let presentation_layout = self.layout_presentation(&mut calculator);
        let open = calculator.calculate(&PROP_OPEN, self.open.as_ref());
        let close = calculator.calculate(&PROP_CLOSE, self.close.as_ref());

        let separators: Vec<char> = calculator.calculate(&PROP_SEPARATORS, self.separators.as_ref())
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();

        let inherited_fork = calculator.make_fork().copy();
        let new_family = family.add(self);

        // Scope for mutating placeholders based on calculated props
        {
            let mut placeholders = self.placeholders.borrow_mut();

            // Updated math background since it is not inherited
            placeholders.with_math_background(Some(presentation_layout.math_background));

            let placeholders: &mut Mrow = placeholders.as_any_mut().downcast_mut::<Mrow>().unwrap();
            let children = placeholders.children_mut();

            // Scope for opening fence operator mutable borrow
            {
                let open_fence_op: &mut Mo = children[0].as_any_mut().downcast_mut::<Mo>()
                    .expect("Cannot find opening fence placeholder");
                open_fence_op.with_text(open);
            }

            // Scope for closing fence operator mutable borrow
            {
                let close_fence_op: &mut Mo = children[2].as_any_mut().downcast_mut::<Mo>()
                    .expect("Cannot find closing fence placeholder");
                close_fence_op.with_text(close);
            }

            // Scope for content mutable borrow
            {
                let content: &mut Mrow = children[1].as_any_mut().downcast_mut::<Mrow>()
                    .expect("Cannot find contents");

                let mut latest_separator = separators[0];
                let mut separator_index = 0usize;
                for (index, child) in content.children_mut().iter_mut().enumerate() {
                    if index % 2 != 0 {
                        let separator: &mut Mo = child.as_any_mut().downcast_mut::<Mo>()
                            .expect(&format!("Expected operator at index {}", index));
                        latest_separator = *separators.get(separator_index).unwrap_or(&latest_separator);
                        separator.with_text(latest_separator.to_string());
                        separator_index += 1;
                    }
                }
            }
        }

        let row = self.placeholders.borrow();
        let layout = row.layout(context, &new_family, &inherited_fork, style);

        return layout;
    }

    fn type_info(&self) -> ElementType {
        ElementType::GeneralLayout(GeneralLayout::Mfenced)
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }

    fn instance_id(&self) -> &InstanceId {
        &self.instance_id
    }
}

impl PresentationPrivate<Mfenced> for Mfenced {
    fn get_specified_presentation_props(&self) -> &SpecifiedPresentationProps {
        &self.presentation_props
    }

    fn get_specified_presentation_props_mut(&mut self) -> &mut SpecifiedPresentationProps {
        &mut self.presentation_props
    }
}

impl Presentation<Mfenced> for Mfenced {}


#[cfg(test)]
mod test {
    use ::test::skia::Snapshot;
    use super::*;
    use elements::*;
    use ::props::*;


    #[test]
    fn it_works() {
        let snap = Snapshot::default();

        let mut fenced = Mfenced::new();
        fenced.with_child(Box::new(Mi::new(String::from("x"))));
        snap.snap_element(&fenced, "mfenced_one_element");

        fenced.with_child(Box::new(Mi::new(String::from("y"))));
        snap.snap_element(&fenced, "mfenced_two_elements");
        
        fenced.with_open(Some(String::from("{")));
        snap.snap_element(&fenced, "mfenced_curly_braces_open");

        fenced.with_close(Some(String::from("}")));
        snap.snap_element(&fenced, "mfenced_curly_braces_close");

        fenced.with_separators(Some(String::from(".")));
        snap.snap_element(&fenced, "mfenced_dot_separator");

        fenced.with_child(Box::new(Mn::new(String::from("3"))));
        snap.snap_element(&fenced, "mfenced_repeats_separator");

        fenced.with_separators(Some(String::from(",.")));
        snap.snap_element(&fenced, "mfenced_multiple_separator");

        fenced.with_child(Box::new(Mn::new(String::from("2"))));
        snap.snap_element(&fenced, "mfenced_repeats_last_separator");

        fenced.with_math_color(Some(Color::RGB(0, 255, 0)));
        snap.snap_element(&fenced, "mfenced_green_color");

        fenced.with_math_background(Some(Color::RGB(0, 0, 0)));
        snap.snap_element(&fenced, "mfenced_black_background");
    }
}