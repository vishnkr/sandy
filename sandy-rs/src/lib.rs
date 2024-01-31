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
    /* Bitmask to store cell info. 
    |7 bits - random multiplier for color lightness | 1 bit - recent simulation step (odd - 1) or (even - 0)
    */
    mask: u8,
}

const W: f64 = 5.0;

pub struct Dimensions{
    width: i32,
    height: i32,
}


#[wasm_bindgen]
pub struct World{
    dimensions: Dimensions,
    cells: Vec<Cell>,
    rng: SmallRng,
    simulation_step: u8
}


const EMPTY_CELL: Cell = Cell {
    element_type: ElementType::Empty,
    mask:0,
};

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Coordinates{
    x: i32,
    y: i32,
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

    fn get_cell(&self,x:i32,y:i32)->Cell{
        let idx = self.get_index(y, x);
        self.cells[idx]
    }

    fn set_cell(&mut self, idx:usize , mut cell:Cell){
        cell.mask = self.simulation_step & 1;
        self.cells[idx] = cell;
    }
}

#[wasm_bindgen]
impl World{
    #[wasm_bindgen(constructor)]
    pub fn new(height:i32,width:i32)->World{
        let cells:Vec<Cell> = (0..width*height).map(|_i| EMPTY_CELL).collect();
        console_error_panic_hook::set_once();
        World{
            cells,
            dimensions: Dimensions{ height, width},
            simulation_step:0,
            rng : SmallRng::from_entropy(),
        }
    }
    
    //might not work since Cell cant be accessed from wasm memory directly
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn get_element_type(&self,index:usize)->ElementType{
        self.cells[index].element_type.clone()
    }

    pub fn width(&self)->i32{
        self.dimensions.width
    }

    pub fn height(&self)->i32{
        self.dimensions.height
    }

   
    pub fn get_coords(&self,pos:i32)->Coordinates{
        Coordinates{ x:pos%self.dimensions.width, y:pos/self.dimensions.width}
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

    pub fn get_index(&self, row: i32, col: i32) -> usize {
        (row * self.dimensions.width + col) as usize
    }

    pub fn paint(&mut self, /*idx:usize*/ row:i32,col:i32,element_type:ElementType){
        let idx = self.get_index(row, col);
        self.cells[idx as usize] = Cell{element_type,mask:0};
        // add random cells with same particle type around this position for brush like effect
        let p = match element_type{ElementType::Stone | ElementType::Ice => 1.0, _ => 0.5};
        let brush_size =  match element_type{ElementType::Stone | ElementType::Ice => 6, _ => 10};
        let extent = brush_size / 2;
        for i in (-extent as i32)..=(extent as i32) {
            for j in (-extent as i32)..=(extent as i32) {
                if self.rand_float() < p{
                    let new_col = col.wrapping_add(i as i32);
                    let new_row = row.wrapping_add(j as i32);

                    // Check if the new position is within bounds
                    if self.is_in_bounds(new_row, new_col) {
                        let new_idx = self.get_index(new_row, new_col) as usize;
                        self.set_cell(new_idx, Cell { element_type, mask: 0 });
                    }
                }
            }
        }
    }

    fn is_in_bounds(&self,y:i32,x:i32)->bool{
        x>=0 && x<self.dimensions.width && y>=0 && y<self.dimensions.height
    }

    pub fn is_cell_empty(&self,x:i32,y:i32)->bool{
        let index = self.get_index(x, y) as usize;
        self.cells[index].element_type==ElementType::Empty
    }


    pub fn tick(&mut self) {        
        for x in 0..self.dimensions.width {
            for y in (0..self.dimensions.height) {
                let idx = self.get_index(y, x) as usize;
                let cell = self.cells[idx];
                cell.element_type.tick(cell,Painter{
                    world: self,
                    x,
                    y
                })
            }
        }
        self.simulation_step = self.simulation_step.wrapping_add(1);
    }
}

impl Cell{
    fn update_mask(&mut self,simulation_step:u8){
        self.mask = (simulation_step & 1);
    }
}