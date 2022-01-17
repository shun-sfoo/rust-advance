use anyhow::{anyhow, Result};
use askama::Template;
use litrs::Literal;
use proc_macro::TokenStream;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

pub fn get_string_literal(input: TokenStream) -> Result<String> {
    input
        .into_iter()
        .next()
        .and_then(|v| Literal::try_from(v).ok())
        .and_then(|v| match v {
            Literal::String(s) => Some(s.value().to_string()),
            _ => None,
        })
        .ok_or_else(|| anyhow!("Only string literal are allowd"))
}

#[derive(Template)]
#[template(path = "code.j2")]
pub struct StructsTemplate {
    structs: Vec<St>,
}

impl StructsTemplate {
    fn try_new(filename: &str) -> Result<Self> {
        let content = fs::read_to_string(filename)?;
        let schema: Schema = serde_json::from_str(&content)?;
        Ok(StructsTemplate {
            structs: schema.into_vec_st(),
        })
    }

    pub fn render(filename: &str) -> Result<String> {
        let template = Self::try_new(filename)?;
        Ok(template.render()?)
    }
}

/// input data
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Schema {
    title: Option<String>,
    #[serde(rename = "type")]
    ty: String,
    properties: Option<HashMap<String, Schema>>,
}

/// output structure
pub struct St {
    /// structure name
    name: String,
    /// a list of structure fields
    fields: Vec<Fd>,
}

#[derive(Debug)]
pub struct Fd {
    name: String,
    ty: String,
}

impl St {
    pub fn new(name: impl Into<String>, fields: Vec<Fd>) -> Self {
        St {
            name: name.into(),
            fields,
        }
    }
}

impl Fd {
    pub fn new(name: impl Into<String>, ty: impl Into<String>) -> Self {
        Fd {
            name: name.into(),
            ty: ty.into(),
        }
    }
}

impl Schema {
    pub fn into_vec_st(&self) -> Vec<St> {
        let mut structs = vec![];
        match self.ty.as_ref() {
            "object" => {
                let fields = self
                    .properties
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| process_type(&mut structs, k.as_ref(), v))
                    .collect();
                structs.push(St::new(self.title.as_ref().unwrap(), fields));
                structs
            }
            _ => panic!("Not support"),
        }
    }
}

fn process_type(structs: &mut Vec<St>, k: &str, v: &Schema) -> Fd {
    let name = n(k);
    match v.ty.as_str() {
        "object" => {
            let sts = v.into_vec_st();
            structs.extend(sts);
            Fd::new(name, p(v.title.as_deref().unwrap_or(k)))
        }
        "integer" => Fd::new(name, "i64"),
        "float" => Fd::new(name, "f64"),
        "string" => Fd::new(name, "String"),
        v => panic!("Unsupprted schema type {}", v),
    }
}

/// pascal type name
fn p(k: &str) -> String {
    heck::AsPascalCase(k).to_string()
}

fn n(k: &str) -> String {
    heck::AsSnakeCase(k).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const P1: &str = include_str!("../fixtures/person1.json");
    const P2: &str = include_str!("../fixtures/person2.json");

    #[test]
    fn schema_should_be_converted_to_st() {
        let schema: Schema = serde_json::from_str(P1).unwrap();
        let mut v = schema.into_vec_st();
        assert_eq!(1, v.len());
        let st = v.pop().unwrap();
        let mut names = st
            .fields
            .iter()
            .map(|f| f.name.clone())
            .collect::<Vec<String>>();
        names.sort();
        assert_eq!(st.name, "Person1");
        assert_eq!(&names[..], &["first_name", "last_name"]);
        assert_eq!(st.fields[0].ty, "String");
        assert_eq!(st.fields[1].ty, "String");
    }

    #[test]
    fn schema_with_nested_object_should_be_converted_to_st() {
        let schema: Schema = serde_json::from_str(P2).unwrap();
        let v = schema.into_vec_st();
        assert_eq!(2, v.len());
    }

    #[test]
    fn schema_render_should_be_converted_to_st() {
        let result = StructsTemplate::render(
            "/Users/bruce/StudySpace/rust-advance/proc_macros/fixtures/person2.json",
        );
        println!("{:#?}", result);
    }
}
