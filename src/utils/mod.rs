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


use ::elements::{Element, Mrow, Family, Mphantom, Mmultiscripts, Mo, Munderover};
use ::layout::{ElementGroup, Layout, MrowLayout, MoLayout, MmultiscriptLayout, MunderoverLayout, MfracLayout,
               MstyleLayout, MphatomLayout, MpaddedLayout, MactionLayout, MtextLayout, Maligngroup, Malignmark};


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
        let phantom: &Mphantom = element.as_any().downcast_ref::<Mphantom>().unwrap();
        return is_space_like(phantom.child().as_ref());
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


pub fn get_enclosing_embellished_operator<'a>(family: &'a Family<'a>, base_op: & Mo) -> Option<&'a Family<'a>> {
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
        if let Some(scripts) = parent.as_any().downcast_ref::<Mmultiscripts>() {
            let scripts: &Mmultiscripts = scripts;
            scripts.base().instance_id() == base_op.instance_id()
        } else if let Some(underover) = parent.as_any().downcast_ref::<Munderover>() {
            let underover: &Munderover = underover;
            underover.base().instance_id() == base_op.instance_id()
        } else {
            panic!("Unknown script type")
        }
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
            .and_then(|f| get_enclosing_embellished_operator(f, base_op))
            .or(Some(family));
    }

    return None;
}

// The following MathML elements are defined to be ‘space-like’:
// • an mtext, mspace, maligngroup, or malignmark element;
// • an mstyle, mphantom, or mpadded element, all of whose direct sub-expressions are space-
//   like;
// • an maction element whose selected sub-expression exists and is space-like;
// • an mrow all of whose direct sub-expressions are space-like.
pub fn is_space_like_layout(layout: &Box<Layout>) -> bool {
    if layout.as_any().is::<MtextLayout>() || layout.as_any().is::<Maligngroup>() || layout.as_any().is::<Malignmark>() {
        return true;
    }

    if let Some(_) = layout.as_any().downcast_ref::<MstyleLayout>() {
        unimplemented!()
    }

    if let Some(phantom) = layout.as_any().downcast_ref::<MphatomLayout>() {
        let phantom: &MphatomLayout = phantom;
        return is_space_like_layout(&phantom.child_layout);
    }

    if let Some(padded) = layout.as_any().downcast_ref::<MpaddedLayout>() {
        let padded: &MpaddedLayout = padded;
        return is_space_like_layout(&padded.child_layout);
    }

    if let Some(_) = layout.as_any().downcast_ref::<MactionLayout>() {
        unimplemented!();
    }

    if let Some(row) = layout.as_any().downcast_ref::<MrowLayout>() {
        let row: &MrowLayout = row;
        return row.children().iter()
            .find(|child| !is_space_like_layout(*child))
            .is_none();
    }

    return false;
}


// The precise definition of an ‘embellished operator’ is:
//
// • an mo element;
// • or one of the elements msub, msup, msubsup, munder, mover, munderover,
//   mmultiscripts, mfrac, or semantics (Section 5.1), whose first argument exists and is an
//   embellished operator;
// • or one of the elements mstyle, mphantom, or mpadded, such that an mrow containing the
//   same arguments would be an embellished operator;
// • or an maction element whose selected sub-expression exists and is an embellished operator;
// • or an mrow whose arguments consist (in any order) of one embellished operator and zero or
//   more space-like elements.
pub fn get_core_mo_layout(layout: &Box<Layout>) -> Option<&MoLayout> {
    if let Some(mo) = layout.as_any().downcast_ref::<MoLayout>() {
        return Some(mo);
    }

    if let Some(multiscript) = layout.as_any().downcast_ref::<MmultiscriptLayout>() {
        let multiscript: &MmultiscriptLayout = multiscript;
        return get_core_mo_layout(&multiscript.base_layout);
    }

    if let Some(underoverscript) = layout.as_any().downcast_ref::<MunderoverLayout>() {
        let underoverscript: &MunderoverLayout = underoverscript;
        return get_core_mo_layout(&underoverscript.base);
    }

    if let Some(frac) = layout.as_any().downcast_ref::<MfracLayout>() {
        let frac: &MfracLayout = frac;
        return get_core_mo_layout(&frac.numerator);
    }

    if let Some(_) = layout.as_any().downcast_ref::<MstyleLayout>() {
        unimplemented!();
    }

    if let Some(phantom) = layout.as_any().downcast_ref::<MphatomLayout>() {
        let phantom: &MphatomLayout = phantom;
        return get_core_mo_layout(&phantom.child_layout);
    }

    if let Some(padded) = layout.as_any().downcast_ref::<MpaddedLayout>() {
        let padded: &MpaddedLayout = padded;
        return get_core_mo_layout(&padded.child_layout);
    }

    if let Some(_) = layout.as_any().downcast_ref::<MactionLayout>() {
        unimplemented!();
    }

    if let Some(mrow) = layout.as_any().downcast_ref::<MrowLayout>() {
        let mrow: &MrowLayout = mrow;
        let non_space_like = mrow.children().iter()
            .find(|child| is_space_like_layout(*child));

        if let Some(layout) = non_space_like {
            return get_core_mo_layout(layout);
        }
    }

    return None;
}

pub fn get_variable_length_prop<T>(props: &Vec<T>, index: usize) -> &T {
    &props[index.min(props.len()-1)]
}