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
    is_code_active: i32,
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
    belt: Item, 
    boots: Item,
    chest: Item,
    ring1: Item,
    ring2: Item,
    amulet: Item,
    gloves: Item,
    helmet: Item,
    shield: Item,
    main_hand: Item,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    r: i32,
    md: i32,
    ele: i32,
    lvl: i32,
    mods: String,
    qual: i32,
    tpe: i32,
    mat_type: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mogs {}
