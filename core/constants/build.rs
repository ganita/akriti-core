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

#[macro_use] extern crate serde_derive;
extern crate phf_codegen;
extern crate constants_props;
extern crate serde;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write, Read};
use std::path::Path;
use constants_props::{OperatorForm, MathVariant};
use std::collections::HashMap;

struct StringBuilder {
    data: String
}

impl Write for StringBuilder {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        self.data += String::from_utf8(Vec::from(buf)).unwrap().as_ref();
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        // do nothing
        Ok(())
    }
}

impl StringBuilder {
    pub fn new() -> StringBuilder {
        StringBuilder { data: String::new() }
    }
}

fn main() {
    generate_operator_dictionary();
    generate_math_variant_dictionary();
}

#[derive(Hash, Deserialize, Eq, PartialEq)]
pub enum FormDef {
    #[serde(rename = "infix")]
    Infix = 0,
    #[serde(rename = "prefix")]
    Prefix = 1,
    #[serde(rename = "postfix")]
    Postfix = 2
}

#[derive(Serialize, Deserialize)]
struct OperatorDef {
    name: String,
    priority: u32,
    lspace: u8,
    rspace: u8,
    #[serde(with = "PropertiesDef")]
    #[serde(default)]
    properties: PropertiesDef
}

#[derive(Serialize, Deserialize, Default)]
pub struct PropertiesDef {
    #[serde(default)]
    fence: bool,
    #[serde(default)]
    stretchy: bool,
    #[serde(default)]
    symmetric: bool,
    #[serde(default)]
    separator: bool,
    #[serde(default)]
    accent: bool,
    #[serde(default)]
    largeop: bool,
    #[serde(default)]
    movablelimits: bool,
    #[serde(default)]
    linebreakstyle: String
}

fn generate_operator_dictionary() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let file = Path::new(&dir).join("blobs").join("operator_dictionary.json");
    let mut operator_dict = String::new();
    File::open(file).unwrap().read_to_string(&mut operator_dict).expect("Cannot read operator dictionary");
    let deserialize: HashMap<String, HashMap<FormDef, OperatorDef>> = serde_json::from_str(&operator_dict).unwrap();

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen_operator_dictionary.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());
    write!(&mut file, "static OPERATOR_DICTIONARY: phf::Map<&'static str, phf::Map<OperatorForm, Operator>> = ").unwrap();

    let mut operator_dictionary: phf_codegen::Map<String> = phf_codegen::Map::new();
    for (operator, forms) in deserialize {
        let mut mem_buffer = StringBuilder::new();
        let mut forms_map = phf_codegen::Map::new();

        for (form_def, operator_def) in forms {
            let form = form_def_to_form(&form_def);
            let operator = operator_def_to_operator(&operator_def);
            forms_map.entry(form, &operator);
        }
        forms_map.build(&mut mem_buffer).unwrap();

        operator_dictionary.entry(operator, &mem_buffer.data);
    }
    operator_dictionary.build(&mut file).unwrap();
    write!(&mut file, ";\n").unwrap();
}

fn form_def_to_form(form_def: &FormDef) -> OperatorForm {
    match *form_def {
        FormDef::Infix => OperatorForm::Infix,
        FormDef::Prefix => OperatorForm::Prefix,
        FormDef::Postfix => OperatorForm::Postfix
    }
}

fn operator_def_to_operator(operator_def: &OperatorDef) -> String {
    let property = format!("\
        OperatorProperty {{
            fence: {},
            stretchy: {},
            symmetric: {},
            separator: {},
            accent: {},
            largeop: {},
            movable_limits: {},
            linebreak_style: \"{}\",
        }}",
       operator_def.properties.fence,
       operator_def.properties.stretchy,
       operator_def.properties.symmetric,
       operator_def.properties.separator,
       operator_def.properties.accent,
       operator_def.properties.largeop,
       operator_def.properties.movablelimits,
       operator_def.properties.linebreakstyle
    );

    format!("Operator {{\
        lspace: {},
        rspace: {},
        properties: {}
    }}",
        space_level_int_to_enum(operator_def.lspace),
        space_level_int_to_enum(operator_def.rspace),
        property
    )
}

fn space_level_int_to_enum(space_level: u8) -> &'static str {
    match space_level {
        0 => "SpaceLevel::NoSpace",
        1 => "SpaceLevel::VeryVeryThinMathSpace",
        2 => "SpaceLevel::VeryThinMathSpace",
        3 => "SpaceLevel::ThinMathSpace",
        4 => "SpaceLevel::MediumMathSpace",
        5 => "SpaceLevel::ThickMathSpace",
        6 => "SpaceLevel::VeryThickMathSpace",
        7 => "SpaceLevel::VeryVeryThickMathSpace",
        _ => panic!("Unknown space level {}", space_level)
    }
}

#[derive(Hash, Deserialize, Eq, PartialEq)]
pub enum MathVariantDef {
    Normal,
    Bold,
    Italic,
    BoldItalic,
    DoubleStruck,
    BoldFraktur,
    Script,
    BoldScript,
    Fraktur,
    SansSerif,
    BoldSansSerif,
    SansSerifItalic,
    SansSerifBoldItalic,
    Monospace,
    Initial,
    Tailed,
    Looped,
    Stretched,
}

#[derive(Hash, Deserialize, Eq, PartialEq)]
struct Variant {
    variant: u32,
    name: String
}

fn math_variant_def_to_math_variant(def: &MathVariantDef) -> MathVariant {
    match *def {
        MathVariantDef::Normal => MathVariant::Normal,
        MathVariantDef::Bold => MathVariant::Bold,
        MathVariantDef::Italic => MathVariant::Italic,
        MathVariantDef::BoldItalic => MathVariant::BoldItalic,
        MathVariantDef::DoubleStruck => MathVariant::DoubleStruck,
        MathVariantDef::BoldFraktur => MathVariant::BoldFraktur,
        MathVariantDef::Script => MathVariant::Script,
        MathVariantDef::BoldScript => MathVariant::BoldScript,
        MathVariantDef::Fraktur => MathVariant::Fraktur,
        MathVariantDef::SansSerif => MathVariant::SansSerif,
        MathVariantDef::BoldSansSerif => MathVariant::BoldSansSerif,
        MathVariantDef::SansSerifItalic => MathVariant::SansSerifItalic,
        MathVariantDef::SansSerifBoldItalic => MathVariant::SansSerifBoldItalic,
        MathVariantDef::Monospace => MathVariant::Monospace,
        MathVariantDef::Initial => MathVariant::Initial,
        MathVariantDef::Tailed => MathVariant::Tailed,
        MathVariantDef::Looped => MathVariant::Looped,
        MathVariantDef::Stretched => MathVariant::Stretched,
    }
}

fn generate_math_variant_dictionary() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let file = Path::new(&dir).join("blobs").join("math_variant_dictionary.json");
    let mut variant_dict = String::new();
    File::open(file).unwrap().read_to_string(&mut variant_dict).expect("Cannot read math variant dictionary");
    let deserialize: HashMap<String, HashMap<MathVariantDef, Variant>> = serde_json::from_str(&variant_dict).unwrap();

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen_math_variant_dictionary.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());
    write!(&mut file, "static MATH_VARIANT_DICTIONARY: phf::Map<char, phf::Map<MathVariant, char>> = ").unwrap();

    let mut variant_dictionary = phf_codegen::Map::new();
    for (bmp, variants) in deserialize {
        let mut mem_buffer = StringBuilder::new();
        let mut variants_map = phf_codegen::Map::new();
        let bmp = std::char::from_u32(bmp.parse::<u32>().unwrap()).unwrap();

        for (form_def, variant) in variants {
            let math_variant = math_variant_def_to_math_variant(&form_def);
            let variant = std::char::from_u32(variant.variant).unwrap();
            variants_map.entry(math_variant, &format!("'{}'", variant));
        }
        variants_map.build(&mut mem_buffer).unwrap();

        variant_dictionary.entry(bmp, &mem_buffer.data);
    }
    variant_dictionary.build(&mut file).unwrap();
    write!(&mut file, ";\n").unwrap();
}