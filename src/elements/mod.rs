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


mod property;               pub use self::property::*;
mod presentation;           pub use self::presentation::*;

mod token;                  pub use self::token::*;
mod general_layout;         pub use self::general_layout::*;

mod inherited_props;        pub use self::inherited_props::*;
mod style_props;            pub use self::style_props::*;

mod instance_id;            pub use self::instance_id::*;
mod family;                 pub use self::family::*;

mod mempty;                 pub use self::mempty::*;

use std::any::Any;

use ::platform::Context;
use ::layout::Layout;

pub trait Element {
    fn layout<'a>(&self, context: &Context, family: &Family<'a>, inherited: &InheritedProps,
                  style: &Option<&StyleProps>) -> Box<Layout>;
    fn type_info(&self) -> ElementType;
    fn as_any(&self) -> &Any;
    fn instance_id(&self) -> &InstanceId;
}

impl PartialEq for Element {
    fn eq(&self, other: &Element) -> bool {
        self.instance_id() == other.instance_id()
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum TokenElement {
    Mi,
    Mn,
    Mo,
    Mtext,
    Mspace,
    Ms,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum GeneralLayout {
    Mrow,
    Mfrac,
    Msqrt,
    Mroot,
    Mstyle,
    Merror,
    Mpadded,
    Mphantom,
    Mfenced,
    Menclose,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum ScriptsAndLimits {
    Msub,
    Msup,
    Msubsup,
    Munder,
    Mover,
    Munderover,
    Mmutliscripts,
    Mprescripts,
    Mnone,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum TablularMath {
    Mtable,
    Mtr,
    Mlabeledtr,
    Mtd,
    Maligngroup,
    Malignmark,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum ElementaryMath {
    Mstack,
    Mlongdiv,
    Msgroup,
    Msrow,
    Mscarries,
    Mscarry,
    Msline,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum EnliveningExpression {
    Maction,
}


// Not specified in MathML specs. Used internally.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Miscellaneous {
    Mempty,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum ElementType {
    TokenElement(TokenElement),
    GeneralLayout(GeneralLayout),
    ScriptsAndLimits(ScriptsAndLimits),
    TabularMath(TablularMath),
    ElementaryMath(ElementaryMath),
    EnliveningExpression(EnliveningExpression),
    Miscellaneous(Miscellaneous)
}

impl ElementType {
    pub fn is_token(&self) -> bool {
        if let ElementType::TokenElement(_) = *self { true } else { false }
    }

    pub fn is_mi(&self) -> bool {
        if let ElementType::TokenElement(ref value) = *self { *value == TokenElement::Mi } else { false }
    }

    pub fn is_mn(&self) -> bool {
        if let ElementType::TokenElement(ref value) = *self { *value == TokenElement::Mn } else { false }
    }

    pub fn is_mo(&self) -> bool {
        if let ElementType::TokenElement(ref value) = *self { *value == TokenElement::Mo } else { false }
    }

    pub fn is_mtext(&self) -> bool {
        if let ElementType::TokenElement(ref value) = *self { *value == TokenElement::Mtext } else { false }
    }

    pub fn is_mspace(&self) -> bool {
        if let ElementType::TokenElement(ref value) = *self { *value == TokenElement::Mspace } else { false }
    }

    pub fn is_ms(&self) -> bool {
        if let ElementType::TokenElement(ref value) = *self { *value == TokenElement::Ms } else { false }
    }

    pub fn is_general_layout(&self) -> bool {
        if let ElementType::GeneralLayout(_) = *self { true } else { false }
    }

    pub fn is_mrow(&self) -> bool {
        if let ElementType::GeneralLayout(ref value) = *self { *value == GeneralLayout::Mrow } else { false }
    }

    pub fn is_mfrac(&self) -> bool {
        if let ElementType::GeneralLayout(ref value) = *self { *value == GeneralLayout::Mfrac } else { false }
    }

    pub fn is_msqrt(&self) -> bool {
        if let ElementType::GeneralLayout(ref value) = *self { *value == GeneralLayout::Msqrt } else { false }
    }

    pub fn is_mroot(&self) -> bool {
        if let ElementType::GeneralLayout(ref value) = *self { *value == GeneralLayout::Mroot } else { false }
    }

    pub fn is_mstyle(&self) -> bool {
        if let ElementType::GeneralLayout(ref value) = *self { *value == GeneralLayout::Mstyle } else { false }
    }

    pub fn is_merror(&self) -> bool {
        if let ElementType::GeneralLayout(ref value) = *self { *value == GeneralLayout::Merror } else { false }
    }

    pub fn is_mpadded(&self) -> bool {
        if let ElementType::GeneralLayout(ref value) = *self { *value == GeneralLayout::Mpadded } else { false }
    }

    pub fn is_mphantom(&self) -> bool {
        if let ElementType::GeneralLayout(ref value) = *self { *value == GeneralLayout::Mphantom } else { false }
    }

    pub fn is_mfenced(&self) -> bool {
        if let ElementType::GeneralLayout(ref value) = *self { *value == GeneralLayout::Mfenced } else { false }
    }

    pub fn is_menclose(&self) -> bool {
        if let ElementType::GeneralLayout(ref value) = *self { *value == GeneralLayout::Menclose } else { false }
    }

    pub fn is_scrips_or_limits(&self) -> bool {
        if let ElementType::ScriptsAndLimits(_) = *self { true } else { false }
    }

    pub fn is_msub(&self) -> bool {
        if let ElementType::ScriptsAndLimits(ref value) = *self { *value == ScriptsAndLimits::Msub } else { false }
    }

    pub fn is_msup(&self) -> bool {
        if let ElementType::ScriptsAndLimits(ref value) = *self { *value == ScriptsAndLimits::Msup } else { false }
    }

    pub fn is_msubsup(&self) -> bool {
        if let ElementType::ScriptsAndLimits(ref value) = *self { *value == ScriptsAndLimits::Msubsup } else { false }
    }

    pub fn is_munder(&self) -> bool {
        if let ElementType::ScriptsAndLimits(ref value) = *self { *value == ScriptsAndLimits::Munder } else { false }
    }

    pub fn is_mover(&self) -> bool {
        if let ElementType::ScriptsAndLimits(ref value) = *self { *value == ScriptsAndLimits::Mover } else { false }
    }

    pub fn is_munderover(&self) -> bool {
        if let ElementType::ScriptsAndLimits(ref value) = *self { *value == ScriptsAndLimits::Munderover } else { false }
    }

    pub fn is_mmultiscripts(&self) -> bool {
        if let ElementType::ScriptsAndLimits(ref value) = *self { *value == ScriptsAndLimits::Mmutliscripts } else { false }
    }

    pub fn is_mprescripts(&self) -> bool {
        if let ElementType::ScriptsAndLimits(ref value) = *self { *value == ScriptsAndLimits::Mprescripts } else { false }
    }

    pub fn is_mnone(&self) -> bool {
        if let ElementType::ScriptsAndLimits(ref value) = *self { *value == ScriptsAndLimits::Mnone } else { false }
    }

    pub fn is_tablular_math(&self) -> bool {
        if let ElementType::TabularMath(_) = *self { true } else { false }
    }

    pub fn is_mtable(&self) -> bool {
        if let ElementType::TabularMath(ref value) = *self { *value == TablularMath::Mtable } else { false }
    }

    pub fn is_mtr(&self) -> bool {
        if let ElementType::TabularMath(ref value) = *self { *value == TablularMath::Mtr } else { false }
    }

    pub fn is_mlabeledtr(&self) -> bool {
        if let ElementType::TabularMath(ref value) = *self { *value == TablularMath::Mlabeledtr } else { false }
    }

    pub fn is_mtd(&self) -> bool {
        if let ElementType::TabularMath(ref value) = *self { *value == TablularMath::Mtd } else { false }
    }

    pub fn is_maligngroup(&self) -> bool {
        if let ElementType::TabularMath(ref value) = *self { *value == TablularMath::Maligngroup } else { false }
    }

    pub fn is_malignmark(&self) -> bool {
        if let ElementType::TabularMath(ref value) = *self { *value == TablularMath::Malignmark } else { false }
    }

    pub fn is_elementary_math(&self) -> bool {
        if let ElementType::ElementaryMath(_) = *self { true } else { false }
    }

    pub fn is_mstack(&self) -> bool {
        if let ElementType::ElementaryMath(ref value) = *self { *value == ElementaryMath::Mstack } else { false }
    }

    pub fn is_mlongdiv(&self) -> bool {
        if let ElementType::ElementaryMath(ref value) = *self { *value == ElementaryMath::Mlongdiv } else { false }
    }

    pub fn is_msgroup(&self) -> bool {
        if let ElementType::ElementaryMath(ref value) = *self { *value == ElementaryMath::Msgroup } else { false }
    }

    pub fn is_msrow(&self) -> bool {
        if let ElementType::ElementaryMath(ref value) = *self { *value == ElementaryMath::Msrow } else { false }
    }

    pub fn is_mscarries(&self) -> bool {
        if let ElementType::ElementaryMath(ref value) = *self { *value == ElementaryMath::Mscarries } else { false }
    }

    pub fn is_mscarry(&self) -> bool {
        if let ElementType::ElementaryMath(ref value) = *self { *value == ElementaryMath::Mscarry } else { false }
    }

    pub fn is_msline(&self) -> bool {
        if let ElementType::ElementaryMath(ref value) = *self { *value == ElementaryMath::Msline } else { false }
    }

    pub fn is_enlivening_expression(&self) -> bool {
        if let ElementType::EnliveningExpression(_) = *self { true } else { false }
    }

    pub fn is_maction(&self) -> bool {
        if let ElementType::EnliveningExpression(ref value) = *self {
            *value == EnliveningExpression::Maction
        } else { false }
    }
}