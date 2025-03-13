use core::str;
use std::{
    collections::HashMap,
    error::Error,
    fs::read,
    path::Path,
    sync::LazyLock,
};
use tree_sitter::{Language, Tree};
use tree_splicer::{
    node_types::NodeTypes,
    splice::{Config, Splicer},
};
use walkdir::WalkDir;

use crate::util::glob_next;

static NODE_TYPES: LazyLock<NodeTypes> =
    LazyLock::new(|| NodeTypes::new(tree_sitter_rust::NODE_TYPES).unwrap());
static LANGUAGE: LazyLock<Language> = LazyLock::new(tree_sitter_rust::language);

pub fn parse(s: &str) -> Result<Tree, Box<dyn Error>> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(*LANGUAGE)?;
    let res = parser.parse(s, None).ok_or("tree-sitter parse failed!")?;
    Ok(res)
}

pub fn parse_dir(p: &Path) -> Result<HashMap<String, (Vec<u8>, Tree)>, Box<dyn Error>> {
    let mut res = HashMap::new();
    for f in WalkDir::new(p) {
        let f = f?;
        if f.file_type().is_dir() {
            continue;
        }
        let path = f.path();
        let s = read(path)?;
        let tree = parse(str::from_utf8(&s)?)?;
        let path = path
            .as_os_str()
            .to_str()
            .ok_or("Path not stringable")?
            .to_string();
        res.insert(path, (s, tree));
    }
    Ok(res)
}

pub fn do_splicer(
    trees: &HashMap<String, (Vec<u8>, Tree)>,
    seed: Option<u64>,
) -> Result<Splicer, Box<dyn Error>> {
    let seed = match seed {
        Some(s) => s,
        None => glob_next(),
    };
    let config = Config {
        chaos: 5,
        deletions: 5,
        language: *LANGUAGE,
        inter_splices: 16,
        max_size: 1024 * 1024, // 1M
        node_types: NODE_TYPES.clone(),
        reparse: 2,
        seed,
    };
    let splicer = Splicer::new(config, trees).ok_or("Init splicer failed, no files.")?;
    Ok(splicer)
}
