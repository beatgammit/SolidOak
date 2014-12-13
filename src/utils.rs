use rgtk::*;
use serialize::{Encodable, json};
use std::collections::HashSet;
use std::io::fs;

pub struct State<'a> {
    pub projects: HashSet<String>,
    pub expansions: HashSet<String>,
    pub selection: Option<String>,
    pub tree_store: &'a gtk::TreeStore,
    pub tree_model: &'a gtk::TreeModel,
    pub tree_selection: &'a gtk::TreeSelection,
    pub rename_button: &'a gtk::Button,
    pub remove_button: &'a gtk::Button,
}

#[deriving(Decodable, Encodable)]
struct Prefs {
    projects: Vec<String>,
    expansions: Vec<String>,
    selection: Option<String>
}

pub fn get_data_dir() -> Path {
    let home = ::std::os::homedir();
    let mut path = match home {
        Some(p) => p,
        None => Path::new(".")
    };
    path.push(".solidoak");
    path
}

fn get_prefs_file() -> Path {
    let mut path = get_data_dir();
    path.push("prefs.json");
    path
}

fn get_prefs(state: &State) -> Prefs {
    Prefs {
        projects: state.projects.clone().into_iter().collect(),
        expansions: state.expansions.clone().into_iter().collect(),
        selection: state.selection.clone()
    }
}

pub fn are_siblings(path1: &String, path2: &String) -> bool {
    let parent_path1 = Path::new(path1).dir_path();
    let parent_path2 = Path::new(path2).dir_path();

    let parent1 = parent_path1.as_str();
    let parent2 = parent_path2.as_str();

    parent1.is_some() && parent2.is_some() &&
    parent1.unwrap() == parent2.unwrap()
}

pub fn get_selected_path(state: &State) -> Option<String> {
    let mut iter = gtk::TreeIter::new().unwrap();

    let path = if state.tree_selection.get_selected(state.tree_model, &mut iter) {
        state.tree_model.get_value(&iter, 1).get_string()
    } else {
        None
    };

    iter.drop();
    path
}

pub fn write_prefs(state: &State) {
    let prefs = get_prefs(state);

    let mut buffer: Vec<u8> = Vec::new();
    {
        let mut encoder = json::PrettyEncoder::new(&mut buffer);
        prefs.encode(&mut encoder).ok().expect("Error encoding prefs.");
    }
    let json_str = String::from_utf8(buffer).unwrap();

    let prefs_path = get_prefs_file();
    let mut f = fs::File::create(&prefs_path);
    match f.write_str(json_str.as_slice()) {
        Ok(_) => {},
        Err(e) => println!("Error writing prefs: {}", e)
    };
}

pub fn read_prefs(state: &mut State) {
    let prefs_path = get_prefs_file();
    let mut f = fs::File::open(&prefs_path);
    let prefs_option : Option<Prefs> = match f.read_to_string() {
        Ok(json_str) => {
            match json::decode(json_str.as_slice()) {
                Ok(object) => Some(object),
                Err(e) => {
                    println!("Error decoding prefs: {}", e);
                    None
                }
            }
        },
        Err(_) => None
    };

    if let Some(prefs) = prefs_option {
        state.projects.clear();
        for path in prefs.projects.iter() {
            state.projects.insert(path.clone());
        }

        state.expansions.clear();
        for path in prefs.expansions.iter() {
            state.expansions.insert(path.clone());
        }

        state.selection = prefs.selection;
    }
}
