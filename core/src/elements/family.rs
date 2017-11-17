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


use super::{Element};

pub struct Family<'a> {
    grand_parent: Option<&'a Family<'a>>,
    parent: Option<&'a Element>
}

impl<'a> Family<'a> {
    pub fn new() -> Family<'a> {
        Family {
            grand_parent: None,
            parent: None,
        }
    }

    pub fn add(&'a self, element: &'a Element) -> Family<'a> {
        Family {
            grand_parent: Some(self),
            parent: Some(element),
        }
    }

    pub fn parent(&'a self) -> Option<&'a Element> {
        self.parent.clone()
    }

    pub fn grand_parent(&'a self) -> Option<&'a Family<'a>> {
        self.grand_parent.clone()
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;

    use super::*;
    use ::platform::Context;
    use ::elements::{InstanceId, InheritedProps, StyleProps, ElementType};
    use ::layout::Layout;

    struct MockElement {
        instance_id: InstanceId,
        child: MockElement2
    }

    impl MockElement {
        fn new() -> MockElement {
            MockElement {
                instance_id: InstanceId::new(),
                child: MockElement2 {}
            }
        }
    }

    impl Element for MockElement {
        fn layout<'a>(&self, _: &Context, _: &Family<'a>, _: &InheritedProps,
                  _: &Option<&StyleProps>) -> Box<Layout> {
            unimplemented!()
        }

        fn type_info(&self) -> ElementType {
            unimplemented!()
        }

        fn as_any(&self) -> &Any {
            unimplemented!()
        }

        fn instance_id(&self) -> &InstanceId {
            &self.instance_id
        }

        fn as_any_mut(&mut self) -> &mut Any {
            unimplemented!()
        }
    }

    trait Test {
        fn test_layout<'a>(&self, parent: &Family<'a>);
    }

    impl Test for MockElement {
        fn test_layout<'a>(&self, parent: &Family<'a>) {
            self.child.test_layout(&parent.add(self));
        }
    }

    struct MockElement2 {}

    impl Test for MockElement2 {
        fn test_layout<'a>(&self, parent: &Family<'a>) {
            assert_eq!(parent.parent().is_some(), true);
        }
    }

    #[test]
    fn it_works() {
        let hierarchy = Family::new();
        assert_eq!(hierarchy.parent().is_none(), true);
        assert_eq!(hierarchy.grand_parent().is_none(), true);

        let element = MockElement::new();
        let new = hierarchy.add(&element);
        assert_eq!(new.parent().unwrap().instance_id(), element.instance_id());
        assert_eq!(new.grand_parent().unwrap().parent().is_none(), true);

        let element1 = MockElement::new();
        let new1 = new.add(&element1);
        assert_eq!(new1.parent().unwrap().instance_id(), element1.instance_id());
        assert_eq!(new1.grand_parent().unwrap().parent().unwrap().instance_id(), element.instance_id());

        element1.test_layout(&Family::new());
    }
}