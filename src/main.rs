extern crate iron;
#[macro_use]
extern crate router;
extern crate persistent;
extern crate rustc_serialize;
extern crate unidecode;
extern crate unicode_segmentation;
#[macro_use]
extern crate log;

mod api;
mod query;
mod mapping;
mod analysis;
mod logger;

use std::sync::{Mutex, RwLock};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs;

use iron::prelude::*;
use iron::typemap::Key;
use rustc_serialize::json::Json;


const VERSION: &'static str = "0.1a0";


#[derive(Debug, PartialEq)]
enum Value {
    String(String),
    TSVector(Vec<String>),
    Boolean(bool),
    I64(i64),
    U64(u64),
    F64(f64),
    Null,
}

impl Value {
    pub fn from_json(json: &Json) -> Value {
        // TODO: Should be aware of mappings
        match json {
            &Json::String(ref string) => Value::String(string.clone()),
            &Json::Boolean(value) => Value::Boolean(value),
            &Json::F64(value) => Value::F64(value),
            &Json::I64(value) => Value::I64(value),
            &Json::U64(value) => Value::U64(value),
            &Json::Null => Value::Null,

            // These two are unsupported
            // TODO: Raise error
            &Json::Array(_) => Value::Null,
            &Json::Object(_) => Value::Null,
        }
    }

    pub fn as_json(&self) -> Json {
        match self {
            &Value::String(ref string) => Json::String(string.clone()),
            &Value::TSVector(ref string) => Json::Null, // TODO
            &Value::Boolean(value) => Json::Boolean(value),
            &Value::F64(value) => Json::F64(value),
            &Value::I64(value) => Json::I64(value),
            &Value::U64(value) => Json::U64(value),
            &Value::Null => Json::Null,
        }
    }
}


#[derive(Debug)]
struct Document {
    fields: BTreeMap<String, Value>,
}

impl Document {
    fn from_json(data: Json, mapping: &mapping::Mapping) -> Document {
        let mut fields = BTreeMap::new();
        let mut all_field_tokens: Vec<String> = Vec::new();

        for (field_name, field_value) in data.as_object().unwrap() {
            let processed_value = if let Some(field_mapping) = mapping.fields.get(field_name) {
                let value = field_mapping.process_value(field_value.clone());

                // Add to _all
                if field_mapping.is_in_all {
                    if let &Some(Value::TSVector(ref tokens)) = &value {
                        for token in tokens.iter() {
                            all_field_tokens.push(token.clone());
                        }
                    }
                }

                value
            } else {
                Some(Value::from_json(field_value))
            };

            if let Some(field_value) = processed_value {
                fields.insert(field_name.clone(), field_value);
            }
        }

        // Insert _all field
        fields.insert("_all".to_owned(), Value::TSVector(all_field_tokens));

        Document { fields: fields }
    }
}


#[derive(Debug)]
struct Index {
    pub mappings: HashMap<String, mapping::Mapping>,
    pub docs: HashMap<String, Document>,
    pub aliases: HashSet<String>,
}


impl Index {
    fn new() -> Index {
        Index {
            mappings: HashMap::new(),
            docs: HashMap::new(),
            aliases: HashSet::new(),
        }
    }

    fn initialise(&mut self) {
    }
}


struct Globals {
    pub indices_path: PathBuf,
    pub indices: RwLock<HashMap<String, Index>>,
}


impl Globals {
    fn new(indices_path: PathBuf, indices: HashMap<String, Index>) -> Globals {
        Globals {
            indices_path: indices_path,
            indices: RwLock::new(indices),
        }
    }
}


impl Key for Globals {
    type Value = Globals;
}


fn load_index(path: &Path) -> Index {
    Index::new()
}


fn load_indices(indices_path: &Path) -> HashMap<String, Index> {
    let mut indices = HashMap::new();

    for file in fs::read_dir(&indices_path).unwrap() {
        let path = file.unwrap().path();
        let index_name: String = path.file_stem().unwrap().to_str().unwrap().to_owned();

        if let Some(ext) = path.extension() {
            if ext.to_str() == Some("rsi") {
                info!("Loaded index: {}", index_name);
                indices.insert(index_name, load_index(path.as_path()));
            }
        }
    }

    indices
}


fn main() {
    println!("rsearch ({})", VERSION);
    println!("");

    println!("{:?}", analysis::Analyzer::EdgeNGram.run("Up from the bowels of hell he sail. Weilding a tankard of freshly brewed ale.".to_string()));

    logger::init().unwrap();

    let indices_path = Path::new("./indices").to_path_buf();
    let indices = load_indices(&indices_path.as_path());
    let router = api::get_router();
    let mut chain = Chain::new(router);
    chain.link(persistent::Read::<Globals>::both(Globals::new(indices_path, indices)));
    Iron::new(chain).http("localhost:9200").unwrap();
}
