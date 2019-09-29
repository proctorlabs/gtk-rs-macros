use serde::Deserialize;

use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(Debug, Deserialize)]
pub(crate) struct GladeRoot {
    #[serde(default)]
    object: Vec<GladeObject>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GladeObject {
    #[serde(default)]
    id: Option<String>,

    #[serde(default)]
    child: Vec<GladeRoot>,

    #[serde(default)]
    signal: Vec<GladeSignal>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GladeSignal {
    #[serde(default)]
    handler: String,
}

impl GladeRoot {
    pub fn parse(path: &str) -> GladeRoot {
        let root = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
        let path = Path::new(&root).join("demo/").join(&path);
        let content = read_file(path).unwrap();
        serde_xml_rs::from_str(&content).unwrap()
    }
}

fn read_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path.as_ref())?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}
