use serde::{Serialize, Deserialize};

use std::{collections::HashMap};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct RootData {
    pub name: String,
    sections: HashMap<i32, Section>
}
impl RootData {
    pub(crate) fn new() -> Self {
        Self {
            name: String::from("Untitled"),
            sections: HashMap::new(),
        }
    }

    pub(crate) fn get_sections(&self) -> &HashMap<i32, Section> {
        &self.sections
    }

    pub(crate) fn section_insert(&mut self, name: String) {
        let mut cand: i32 = rand::random();
        while self.sections.contains_key(&cand) {
            cand = rand::random();
        }
        self.sections.insert(cand, Section { 
            name, 
            rooms: HashMap::new(), 
            doors: HashMap::new(), 
            item_tables: HashMap::new(), 
            items: HashMap::new(), 
            characters: HashMap::new(), 
            routines: vec![], 
        });
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Section {
    pub name: String,
    pub rooms: HashMap<i32, Room>,
    pub doors: HashMap<i32, Door>,
    pub item_tables: HashMap<i32, ItemTable>,
    pub items: HashMap<i32, Item>,
    pub characters: HashMap<i32, Character>,
    pub routines: Vec<Routine>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Room {
    pub name: String,
    pub fixtures: HashMap<i32, Fixture>,
    pub item_ids: Vec<i32>,
    pub item_table_ids: Vec<i32>,
    pub events: Vec<Event>,
    pub routines: Vec<Routine>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Character {
    pub name: String,
    pub item_ids: Vec<i32>,
    pub item_table_ids: Vec<i32>,
    pub events: Vec<Event>,
    pub routines: Vec<Routine>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Door {
    pub name: String,
    pub connecting_room_ids: (i32, i32),
    pub events: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Fixture {
    pub name: String,
    pub item_ids: Vec<i32>,
    pub item_table_ids: Vec<i32>,
    pub events: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ItemTable {
    pub table_type: ItemTableType,
    pub associated_id: i32,
    pub item_ids: HashMap<i32, i32>, 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) enum ItemTableType {
    Room,
    Character,
    Door,
    Fixture,
    Event,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Item {
    pub name: String,
    pub desc: String,
    pub effect_desc: Option<String>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Event {
    pub name: String,
    pub desc: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Routine {
    pub name: String,
    pub desc: String,
}