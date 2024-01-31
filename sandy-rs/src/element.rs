use wasm_bindgen::prelude::*;

use crate::{Cell, World, EMPTY_CELL};

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

pub struct Painter<'a>{
    pub world : &'a mut World,
    pub x: i32,
    pub y: i32
}

impl<'a> Painter<'a>{
    fn set_cell(&mut self, dx: i32, dy:i32, mut cell:Cell){
        let new_x = self.x + dx;
        let new_y = self.y + dy;
        if self.world.is_in_bounds(new_y, new_x){
            let new_idx = self.world.get_index(new_y, new_x);
            cell.update_mask(self.world.simulation_step);
            self.world.set_cell(new_idx, cell);
        }
    }

    fn get_cell(&mut self,dx:i32, dy:i32)->Cell{
        let new_x = self.x+dx;
        let new_y = self.y+dy;
        if self.world.is_in_bounds(new_y,new_x){
            self.world.get_cell(new_x,new_y)
        } else{
            Cell{ element_type: ElementType::Stone, mask:0}
        }
    }

    fn swap_cells(&mut self, dx:i32,dy:i32,origin:Cell){
        let dest = self.get_cell(dx, dy);
        self.set_cell(dx, dy, origin);
        self.set_cell(0, 0, dest);
    }
}

impl ElementType{
    pub fn tick(&self, cell: Cell, mut painter: Painter){
        if (cell.mask&1)^(painter.world.simulation_step&1) == 0{
            return;
        }
        match *self{
            ElementType::Sand => {
                let next = painter.get_cell(0, 1);
                if next.element_type==ElementType::Empty{
                    painter.set_cell(0, 0, EMPTY_CELL);
                    painter.set_cell(0, 1, cell);
                }
            },
            ElementType::Water =>{
                let next = painter.get_cell(0, 1);
                if next.element_type==ElementType::Empty{
                    painter.set_cell(0, 0, EMPTY_CELL);
                    painter.set_cell(0, 1, cell);
                } else if next.element_type == ElementType::Sand{
                    painter.swap_cells(0,1,cell);
                }
            }
            _=>{}
        }
    }
}