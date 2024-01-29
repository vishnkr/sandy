use wasm_bindgen::prelude::*;


#[derive(Clone,Copy,Debug)]
pub struct Particle{
    pub particle_type: ParticleType,

}

pub struct Dimensions{
    width: u32,
    height: u32,
}

#[wasm_bindgen]
pub struct Matrix{
    dimensions: Dimensions,
    particles: Vec<Particle>,

}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParticleType{
    Empty,
    Sand,
    Water,
    Stone,
    Ice,
    Fire,
    Acid,
}

const EMPTY_PARTICLE : Particle = Particle{
    particle_type: ParticleType::Empty
};

impl Matrix{
    pub fn new(height:u32, width: u32)->Matrix{
        let particles = (0..width*height).map(|_i| EMPTY_PARTICLE).collect();
        Matrix{
            particles,
            dimensions: Dimensions{ height, width}
        }
    }

    fn get_index(&self,x:u32,y:u32)->usize{
        (self.dimensions.width*x + y) as usize
    }

    pub fn reset(&mut self){
        for i in 0..self.dimensions.width{
            for j in 0..self.dimensions.height{
                let index = self.get_index(i, j);
                self.particles[index] = EMPTY_PARTICLE;
            }
        }
    }
    
}


/* 
use egui_macroquad::{egui};
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct ParticleColor{
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl ParticleColor{
    pub const fn new(r:u8,g:u8, b:u8)->ParticleColor{
        ParticleColor{r,g,b}
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ParticleProps{
    pub color: ParticleColor,
    pub name: &'static str
}



pub struct Controller{
    pub selected_ptype: ParticleType
}

const PARTICLE_PROPERTIES: [ParticleProps;6] = [
    ParticleProps{
        color: ParticleColor::new(223, 175, 89),
        name: "Sand",
    },
    ParticleProps{
        color: ParticleColor::new(52, 108, 202),
        name: "Water",
    },
    ParticleProps{
        color: ParticleColor::new(101, 106, 115),
        name: "Stone",
    },
    ParticleProps{
        color: ParticleColor::new(195, 214, 247),
        name: "Ice",
    },
    ParticleProps{
        color: ParticleColor::new(255, 123, 36),
        name: "Fire",
    },
    ParticleProps{
        color: ParticleColor::new(165, 250, 95),
        name: "Acid",
    },
];

impl ParticleType{
    fn get_props(&self)->ParticleProps{
        PARTICLE_PROPERTIES[*self as usize]
    }
}
#[macroquad::main("Sandy")]
async fn main() {
    let mut controller = Controller{
        selected_ptype: ParticleType::Sand
    };
    loop {
        clear_background(BLACK);
        egui_macroquad::ui(|ctx| setup_egui(ctx,&mut controller));
        egui_macroquad::draw();
        next_frame().await;
    }
}

fn setup_egui(ctx: &egui::Context, controller: &mut Controller){
    egui::Window::new("Controller")
        .show(ctx, |ui| {
            ui.label("Particle Type");
            ui.separator();
            ui.group(|ui|{
                ui.vertical_centered_justified(|ui|{
                    particle_selectable(ui, ParticleType::Sand, controller);
                    particle_selectable(ui, ParticleType::Water, controller);
                    particle_selectable(ui, ParticleType::Stone, controller);
                    particle_selectable(ui, ParticleType::Fire, controller);
                    particle_selectable(ui, ParticleType::Ice, controller);
                    particle_selectable(ui, ParticleType::Acid, controller);
                })
            })
    });
}

fn particle_selectable(ui: &mut egui::Ui, particle_type: ParticleType, controller: &mut Controller){
    egui::Frame::none().fill(particle_type.get_props().color.into()).show(ui,|ui|{
        ui.selectable_value(&mut controller.selected_ptype, particle_type, 
            egui::RichText::new(particle_type.get_props().name).background_color(egui::Color32::from_black_alpha(150))
        );
    });
}

impl From<ParticleColor> for egui::Color32 {
    fn from(value: ParticleColor) -> Self {
        egui::Color32::from_rgb(value.r, value.g, value.b)
    }
}

impl From<ParticleColor> for Color {
    fn from(value: ParticleColor) -> Self {
        Color::new(
            value.r as f32 / 255.0,
            value.g as f32 / 255.0,
            value.b as f32 / 255.0,
            1.0,
        )
    }
}

impl From<Color> for ParticleColor {
    fn from(value: Color) -> Self {
        if value.a < 1.0 {
            println!(
                "WARNING: Converting Color to ParticleColor ignores alpha of {}",
                value.a
            );
        }
        ParticleColor::new(
            (value.r * 255.0) as u8,
            (value.g * 255.0) as u8,
            (value.b * 255.0) as u8,
        )
    }
}*/