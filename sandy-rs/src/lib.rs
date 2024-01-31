
use wasm_bindgen::prelude::*;
extern crate cfg_if;
extern crate rand;
use rand::{ Rng, SeedableRng};
use rand::rngs::SmallRng;
use element::*;
extern crate console_error_panic_hook;
mod element;
mod utils;
/* 
cfg_if! {
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
         pub fn set_panic_hook() {}
    }
}
*/

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
#[repr(C)]
#[derive(Clone,Copy,Debug)]
pub struct Cell{
    pub element_type: ElementType,
    //bitmask to store misc info. Currently msb stores update_status
    mask: u8,
}

const W: f64 = 5.0;

pub struct Dimensions{
    width: u32,
    height: u32,
}


#[wasm_bindgen]
pub struct World{
    dimensions: Dimensions,
    cells: Vec<Cell>,
    frame_count:u8,
    rng: SmallRng,
}


const EMPTY_CELL: Cell = Cell {
    element_type: ElementType::Empty,
    mask:0,
};

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Coordinates{
    x: u32,
    y: u32,
}

impl World{
    fn rand_dir(&mut self) -> i32 {
        let i = self.rand_int(1000);
        (i % 3) - 1
    }
    fn rand_int(&mut self, n: i32) -> i32 {
        self.rng.gen_range(0..n)
    }

    fn rand_float(&mut self) -> f64 {
        let int_val = self.rand_int(1000);
        int_val as f64 / 1000.0
    }
}

#[wasm_bindgen]
impl World{
    #[wasm_bindgen(constructor)]
    pub fn new(height:u32,width:u32)->World{
        let cells:Vec<Cell> = (0..width*height).map(|_i| EMPTY_CELL).collect();
        console_error_panic_hook::set_once();
        World{
            cells,
            dimensions: Dimensions{ height, width},
            frame_count:0,
            rng : SmallRng::from_entropy(),
        }
    }
    
    //might not work since Cell
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn get_element_type(&self,index:usize)->ElementType{
        self.cells[index].element_type.clone()
    }

    pub fn width(&self)->u32{
        self.dimensions.width
    }

    pub fn height(&self)->u32{
        self.dimensions.height
    }

   

    pub fn get_coords(&self,pos:u32)->Coordinates{
        Coordinates{ y:pos%self.dimensions.width, x:pos/self.dimensions.width}
    }

    #[wasm_bindgen(js_name="emptyCell")]
    pub fn empty_cell(self)->Cell{
        EMPTY_CELL
    }

    pub fn reset(&mut self){
        for i in 0..self.dimensions.width*self.dimensions.height{
            self.cells[i as usize] = EMPTY_CELL;
        }
    }
    
    fn set_cell(&mut self, idx:usize , mut cell:Cell){
        cell.mask = (self.frame_count & 1) << 7;
        self.cells[idx] = cell;
    }

    fn remove_cell(&mut self, idx:usize){
        self.set_cell(idx, EMPTY_CELL)
    }


    pub fn get_index(&self, row: u32, col: u32) -> u32 {
        let res = row * self.dimensions.width + col;
        res
    }

    pub fn paint(&mut self, /*idx:usize*/ row:u32,col:u32,element_type:ElementType){
        let idx = self.get_index(row, col);
        self.cells[idx as usize] = Cell{element_type,mask:0};
        // add random cells with same particle type around this position for brush like effect
        let matrix = 5;
        let extent = matrix / 2;
        for i in (-extent as i32)..=(extent as i32) {
            for j in (-extent as i32)..=(extent as i32) {
                if self.rand_float() < 0.50 {
                    let new_col = col.wrapping_add(i as u32);
                    let new_row = row.wrapping_add(j as u32);

                    // Check if the new position is within bounds
                    if self.is_in_bounds(new_row, new_col) {
                        let new_idx = self.get_index(new_row, new_col) as usize;
                        self.set_cell(new_idx, Cell { element_type, mask: 0 });
                    }
                }
            }
        }
    }

    fn is_in_bounds(&self,x:u32,y:u32)->bool{
        x<self.dimensions.height && y<self.dimensions.width
    }

    pub fn is_cell_empty(&self,x:u32,y:u32)->bool{
        let index = self.get_index(x, y) as usize;
        self.cells[index].element_type==ElementType::Empty
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        let gravity = 0.1;

        for x in 0..self.dimensions.width {
            for y in (0..self.dimensions.height).rev() {
                let idx = self.get_index(y, x) as usize;
                let cell = self.cells[idx];

                if cell.element_type == ElementType::Sand {
                    let mut moved = false;
                    let new_pos = y;

                    for new_y in (new_pos + 1..self.dimensions.height).rev() {
                        if self.is_in_bounds(x,new_y){
                            let below_idx = self.get_index(new_y, x) as usize;
                            let below = &self.cells[below_idx];

                        if below.element_type == ElementType::Empty {
                            next[idx] = Cell {
                                element_type: ElementType::Empty,
                                mask:0,
                            };
                            
                            next[below_idx] = cell;
                            moved = true;
                            break;
                        } else {
                            let dir = 1;
                            if self.is_in_bounds(new_y,x+dir){
                                let ba_index = self.get_index(new_y, x + dir) as usize;
                                let below_a = &self.cells[ba_index];
                                next[idx] = Cell {
                                    element_type: ElementType::Empty,
                                    mask:0,
                                };
                                if below_a.element_type == ElementType::Empty {
                                    next[ba_index] = cell;
                                    moved = true;
                                    break;
                                } 
                            } else if self.is_in_bounds(new_y, x - dir){
                                let bb_index = self.get_index(new_y, x - dir) as usize;
                                let below_b = &self.cells[bb_index];
                                next[idx] = Cell {
                                    element_type: ElementType::Empty,
                                    mask:0,
                                };
                                if below_b.element_type == ElementType::Empty {
                                    next[bb_index] = cell;
                                    moved = true;
                                    break;
                                }
                            }
                        }
                        }
                        
                    }

                    if !moved {
                        next[idx] = cell;
                    }
                }
            }
        }

        self.cells = next;
    }
}