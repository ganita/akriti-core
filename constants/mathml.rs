use ::phf;

pub use ::constants_props::{OperatorForm};

#[derive(Default, Clone)]
pub struct OperatorProperty {
    fence: bool,
    stretchy: bool,
    symmetric: bool,
    separator: bool,
    accent: bool,
    largeop: bool,
    movable_limits: bool,
    linebreak_style: &'static str
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SpaceLevel {
    NoSpace,
    VeryVeryThinMathSpace,
    VeryThinMathSpace,
    ThinMathSpace,
    MediumMathSpace,
    ThickMathSpace,
    VeryThickMathSpace,
    VeryVeryThickMathSpace
}

impl SpaceLevel {
    pub fn em(&self) -> f32 {
        match *self {
            SpaceLevel::NoSpace => 0.0,
            SpaceLevel::VeryVeryThinMathSpace => 1.0/18.0,
            SpaceLevel::VeryThinMathSpace => 2.0/18.0,
            SpaceLevel::ThinMathSpace => 3.0/18.0,
            SpaceLevel::MediumMathSpace => 4.0/18.0,
            SpaceLevel::ThickMathSpace => 5.0/18.0,
            SpaceLevel::VeryThickMathSpace => 6.0/18.0,
            SpaceLevel::VeryVeryThickMathSpace => 7.0/18.0
        }
    }
}

impl OperatorProperty {
    pub fn new(fence: bool, stretchy: bool, symmetric: bool, separator: bool, accent: bool, largeop: bool,
               movable_limits: bool, linebreak_style: &'static str) -> OperatorProperty {
        OperatorProperty {
            fence,
            stretchy,
            symmetric,
            separator,
            accent,
            largeop,
            movable_limits,
            linebreak_style,
        }
    }

    pub fn fence(&self) -> bool {
        self.fence
    }

    pub fn stretchy(&self) -> bool {
        self.stretchy
    }

    pub fn symmetric(&self) -> bool {
        self.symmetric
    }

    pub fn separator(&self) -> bool {
        self.separator
    }

    pub fn accent(&self) -> bool {
        self.accent
    }

    pub fn largeop(&self) -> bool {
        self.largeop
    }

    pub fn movable_limits(&self) -> bool {
        self.movable_limits
    }

    pub fn linebreak_style(&self) -> &'static str {
        self.linebreak_style
    }
}

#[derive(Clone)]
pub struct Operator {
    lspace: SpaceLevel,
    rspace: SpaceLevel,
    properties: OperatorProperty
}

impl Operator {

    pub fn default() -> Operator {
        Operator {
            lspace: SpaceLevel::NoSpace,
            rspace: SpaceLevel::NoSpace,
            properties: OperatorProperty::default()
        }
    }

    pub fn lspace(&self) -> &SpaceLevel {
        &self.lspace
    }

    pub fn rspace(&self) -> &SpaceLevel {
        &self.rspace
    }

    pub fn properties(&self) -> &OperatorProperty {
        &self.properties
    }
}

pub struct OperatorDictionary {
    dictionary: &'static phf::Map<&'static str, phf::Map<OperatorForm, Operator>>
}

use OperatorForm::*;
include!(concat!(env!("OUT_DIR"), "/codegen_operator_dictionary.rs"));

impl OperatorDictionary {
    pub fn new() -> OperatorDictionary {
        OperatorDictionary { dictionary: &OPERATOR_DICTIONARY }
    }

    pub fn operator_attrs(&self, operator: &str, form: &OperatorForm) -> Option<&Operator> {
        self.dictionary.get(operator).and_then(| attrs | attrs.get(form))
    }

    pub fn operator_forms(&self, operator: &str) -> Option<&'static phf::Map<OperatorForm, Operator>> {
        self.dictionary.get(operator)
    }

    pub fn operator_attrs_approx(&self, operator: &str, form: &OperatorForm) -> Option<&Operator> {
        self.operator_attrs(
            operator, form
        ).or(
            self.operator_attrs(operator, &OperatorForm::Infix)
        ).or(
            self.operator_attrs(operator, &OperatorForm::Postfix)
        ).or(
            self.operator_attrs(operator, &OperatorForm::Prefix)
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn if_operator_form_is_present() {
        let dictionary = OperatorDictionary::new();
        let attrs = dictionary.operator_attrs("+", &OperatorForm::Infix).unwrap();
        assert_eq!(*attrs.lspace(), SpaceLevel::MediumMathSpace);
        assert_eq!(*attrs.rspace(), SpaceLevel::MediumMathSpace);

        let attrs = dictionary.operator_attrs("+", &OperatorForm::Prefix).unwrap();
        assert_eq!(*attrs.lspace(), SpaceLevel::NoSpace);
        assert_eq!(*attrs.rspace(), SpaceLevel::VeryVeryThinMathSpace);
    }

    #[test]
    fn if_operator_form_is_not_present() {
        let dictionary = OperatorDictionary::new();
        let attrs = dictionary.operator_attrs_approx("+", &OperatorForm::Postfix).unwrap();
        assert_eq!(*attrs.lspace(), SpaceLevel::MediumMathSpace);
        assert_eq!(*attrs.rspace(), SpaceLevel::MediumMathSpace);

        let attrs = dictionary.operator_attrs_approx("⏝", &OperatorForm::Infix).unwrap();
        assert_eq!(*attrs.lspace(), SpaceLevel::NoSpace);
        assert_eq!(*attrs.rspace(), SpaceLevel::NoSpace);

        let attrs = dictionary.operator_attrs_approx("!", &OperatorForm::Prefix).unwrap();
        assert_eq!(*attrs.lspace(), SpaceLevel::VeryVeryThinMathSpace);
        assert_eq!(*attrs.rspace(), SpaceLevel::NoSpace);
    }

    #[test]
    fn if_operator_is_not_present() {
        let dictionary = OperatorDictionary::new();
        let attrs = dictionary.operator_attrs("a", &OperatorForm::Prefix);
        assert!(attrs.is_none());
        let attrs = dictionary.operator_attrs_approx("a", &OperatorForm::Prefix);
        assert!(attrs.is_none());
    }

    #[test]
    fn is_operator_dictionary_correctly_parsed() {
        let dictionary = OperatorDictionary::new();
        let attrs = dictionary.operator_attrs("\u{2afc}", &OperatorForm::Prefix).unwrap();

        assert_eq!(*attrs.lspace(), SpaceLevel::VeryVeryThinMathSpace);
        assert_eq!(*attrs.rspace(), SpaceLevel::VeryThinMathSpace);
        assert!(attrs.properties().largeop());
        assert!(attrs.properties().movable_limits());
        assert!(attrs.properties().symmetric());
        assert!(!attrs.properties().fence());
        assert!(!attrs.properties().stretchy());
        assert!(!attrs.properties().separator());
        assert!(!attrs.properties().accent());
        assert_eq!(attrs.properties().linebreak_style(), "");

        let attrs = dictionary.operator_attrs(";", &OperatorForm::Infix).unwrap();
        assert_eq!(attrs.properties().linebreak_style(), "after");
    }
}