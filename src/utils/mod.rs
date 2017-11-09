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


use ::elements::{Element, Mrow, Family};


pub fn is_space_like(element: &Element) -> bool {
    let element_type = element.type_info();

    if element_type.is_mtext() || element_type.is_mspace() || element_type.is_maligngroup()
        || element_type.is_malignmark() {
        return true;
    }


    // mstyle, mphantom, or mpadded element, all of whose direct sub-expressions are space-
    // like are space like elements. Refer page 63, MathML3 spec
    if element_type.is_mstyle() {
        unimplemented!();
    }

    if element_type.is_mphantom() {
        unimplemented!();
    }

    if element_type.is_mpadded() {
        unimplemented!();
    }


    // maction element whose selected sub-expression exists and is space-like is space like
    // Refer page 63, MathML3 spec
    if element_type.is_maction() {
        unimplemented!();
    }

    // mrow all of whose direct sub-expressions are space-like
    if element_type.is_mrow() {
        let mrow: &Mrow = element.as_any().downcast_ref::<Mrow>().unwrap();
        let children = mrow.children();

        return children.iter().find(|c| !is_space_like(c.as_ref())).is_none();
    }

    return false;
}


pub fn get_enclosing_embellished_operator<'a>(family: &'a Family<'a>) -> Option<&'a Family> {
    let parent = family.parent();

    if parent.is_none() {
        return None;
    }

    let parent = parent.unwrap();
    let parent_type = parent.type_info();

    // msub, msup, msubsup, munder, mover, munderover,
    // mmultiscripts, mfrac, or semantics (Section 5.1), whose first argument exists and is an
    // embellished operator
    //
    // or an maction element whose selected sub-expression exists and is an embellished operator
    //
    // or one of the elements mstyle, mphantom, or mpadded, such that an mrow containing the
    // same arguments would be an embellished operator
    let is_embellished_operator = if parent_type.is_mrow() {
        let mrow: &Mrow = parent.as_any().downcast_ref::<Mrow>().unwrap();
        let children = mrow.children();
        let num_space_like = children.iter().fold(
            0, |acc, child| if is_space_like(child.as_ref()) { acc+1 } else { acc });

        children.len()-num_space_like == 1
    } else if parent_type.is_scrips_or_limits() {
        unimplemented!()
    } else if parent_type.is_mfrac() {
        unimplemented!()
    } else if parent_type.is_mstyle() || parent_type.is_mphantom() || parent_type.is_mpadded() {
        unimplemented!()
    } else if parent_type.is_maction() {
        unimplemented!()
    } else {
        false
    };

    if is_embellished_operator {
        return family.grand_parent()
            .and_then(|f| get_enclosing_embellished_operator(f))
            .or(Some(family));
    }

    return None;
}