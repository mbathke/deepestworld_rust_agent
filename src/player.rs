use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    id: u32,
    x: f32,
    y: f32,
    z: f32,
    md: i32,
    mtx: Appearance,
    gear: Gear,
    mogs: Mogs,
    #[serde(rename = "isCodeActive")]
    is_code_active: i32,
    #[serde(rename = "subStatus")]
    sub_status: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Appearance {
    face: i32,
    skin: i32,
    helmet: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gear {
    belt: Option<Item>, 
    boots: Option<Item>,
    chest: Option<Item>,
    ring1: Option<Item>,
    ring2: Option<Item>,
    amulet: Option<Item>,
    gloves: Option<Item>,
    helmet: Option<Item>,
    shield: Option<Item>,
    main_hand: Option<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    r: i32,
    md: i32,
    ele: i32,
    lvl: i32,
    mods: std::collections::HashMap<String, i32>,
    qual: i32,
    #[serde(rename = "type")]
    kind: i32,
    #[serde(rename = "matType")]
    mat_type: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mogs {}
