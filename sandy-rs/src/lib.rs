use wasm_bindgen::prelude::*;
extern crate cfg_if;
extern crate rand;
use rand::{Rng,SeedableRng};
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
}

#[wasm_bindgen]
impl World{
    #[wasm_bindgen(constructor)]
    pub fn new(rows:u32,cols:u32)->World{
        let width = cols;
        let height = rows;
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

    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.dimensions.width + column) as usize
    }

    pub fn get_coords(&self,pos:u32)->Coordinates{
        Coordinates{ y:pos/self.dimensions.width, x:pos%self.dimensions.width}
    }

    #[wasm_bindgen(js_name="emptyCell")]
    pub fn empty_cell(self)->Cell{
        EMPTY_CELL
    }

    pub fn toggle_cell(&mut self, idx:usize) {
        //let idx = self.get_index(row, column);
        self.cells[idx] = Cell{element_type: ElementType::Sand, mask:0};
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

    pub fn paint(&mut self, x:u32, y:u32,element_type:ElementType){
        let idx = self.get_index(x, y);
        self.set_cell(idx, Cell{element_type,mask:0});
        // add random cells with same particle type around this position for brush like effect
        let brush_size:i32 = 10;
        for _ in 0..brush_size {
            let offset_x = self.rand_dir() * self.rand_int(3);
            let offset_y = self.rand_dir() * self.rand_int(3);

            let new_x = x.wrapping_add(offset_x as u32);
            let new_y = y.wrapping_add(offset_y as u32);

            // Check if the new position is within bounds
            if self.is_in_bounds(new_x, new_y) {
                let new_idx = self.get_index(new_y, new_x);
                self.set_cell(new_idx, Cell { element_type, mask: 0 });
            }
        }
    }

    fn is_in_bounds(&self,x:u32,y:u32)->bool{
        x>=0 && x<self.dimensions.height.try_into().unwrap() && y>=0 && y<self.dimensions.width.try_into().unwrap()
    }

    pub fn is_cell_empty(&self,x:u32,y:u32)->bool{
        let index = self.get_index(x, y);
        self.cells[index].element_type==ElementType::Empty
    }

    pub fn tick(&mut self){
        let mut next  = self.cells.clone();
        for row in 0..self.dimensions.height {
            for col in 0..self.dimensions.width {

            }
        }
        self.cells = next
    }

    
}