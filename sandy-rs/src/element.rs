use wasm_bindgen::prelude::*;

#[repr(u8)]
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElementType{
    Empty,
    Sand,
    Water,
    Stone,
    Ice,
    Fire,
    Acid,
}

#[derive(Debug, Clone, Copy)]
pub struct ElementColor{
    pub r: u8,
    pub g: u8,
    pub b: u8
}


impl ElementColor{
    pub const fn new(r:u8,g:u8, b:u8)->ElementColor{
        ElementColor{r,g,b}
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ElementProps{
    pub color: ElementColor,
    pub name: &'static str
}

const PARTICLE_PROPERTIES: [ElementProps;6] = [
    ElementProps{
        color: ElementColor::new(223, 175, 89),
        name: "Sand",
    },
    ElementProps{
        color: ElementColor::new(52, 108, 202),
        name: "Water",
    },
    ElementProps{
        color: ElementColor::new(101, 106, 115),
        name: "Stone",
    },
    ElementProps{
        color: ElementColor::new(195, 214, 247),
        name: "Ice",
    },
    ElementProps{
        color: ElementColor::new(255, 123, 36),
        name: "Fire",
    },
    ElementProps{
        color: ElementColor::new(165, 250, 95),
        name: "Acid",
    },
];

impl ElementType{
    fn get_props(&self)->ElementProps{
        PARTICLE_PROPERTIES[*self as usize]
    }
}

