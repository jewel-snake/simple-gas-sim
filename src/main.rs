use gas_sim::logic::{Qtree,Dot};
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

struct App{
	gl: GlGraphics,
}

impl App{
	fn render(&mut self,args: &RenderArgs,t: &Qtree){
		use graphics::*;
		const BLACK: [f32;4] = [0.0,0.0,0.0,1.0];
		const WHITE: [f32;4] = [1.0,1.0,1.0,1.0];
		const RED: [f32;4] = [1.0,0.0,0.0,1.0];
		//let (x,y) = (args.window_size[0]/2.0,args.window_size[1]/2.0);
		self.gl.draw(args.viewport(), |c,gl| {
			clear(BLACK,gl);
			for i in &t.graphics.dots {
				let crl = rectangle::square(i[0],i[1],3.0);
				ellipse(RED,crl,c.transform,gl);
			}
			/*while let gas_sim::logic::Contents::Children(c) = &t.content {
				let f = |cont| {
					for i in cont {

					}
				}; 
			}*/
		});
	}
	fn update(&mut self,args: &UpdateArgs,t: Qtree){}
}

fn main() {
	let opengl = OpenGL::V3_2;
	let mut window: Window = WindowSettings::new("Quad Tree", [600, 400])
	.graphics_api(opengl)
  .exit_on_esc(true)
  .build()
  .unwrap();
  let mut app = App {
    gl: GlGraphics::new(opengl),
  };
  let mut tree = Qtree::new(0.0,0.0,600.0,400.0);
	tree.querry(&mut Dot{x:5_f64,y:5_f64});
	tree.graphics.dots.push([5.0,5.0]);
	tree.querry(&mut Dot{x:100_f64,y:200_f64});
	tree.graphics.dots.push([100.0,200.0]);
	tree.querry(&mut Dot{x:300_f64,y:200_f64});
	tree.graphics.dots.push([300.0,200.0]);
	tree.querry(&mut Dot{x:400_f64,y:300_f64});
	tree.graphics.dots.push([400.0,300.0]);
	tree.querry(&mut Dot{x:500_f64,y:350_f64});
	tree.graphics.dots.push([500.0,350.0]);
	tree.querry(&mut Dot{x:250_f64,y:100_f64});
	tree.graphics.dots.push([250.0,100.0]);
	tree.querry(&mut Dot{x:50_f64,y:50_f64});
	tree.graphics.dots.push([50.0,50.0]);
	tree.querry(&mut Dot{x:140_f64,y:220_f64});
	tree.graphics.dots.push([140.0,220.0]);
	tree.querry(&mut Dot{x:310_f64,y:201_f64});
	tree.graphics.dots.push([310.0,201.0]);
	tree.querry(&mut Dot{x:460_f64,y:370_f64});
	tree.graphics.dots.push([460.0,370.0]);
	tree.querry(&mut Dot{x:540_f64,y:350_f64});
	tree.graphics.dots.push([540.0,350.0]);
	tree.querry(&mut Dot{x:259_f64,y:190_f64});
	tree.graphics.dots.push([259.0,190.0]);
	tree.querry(&mut Dot{x:90_f64,y:90_f64});
	tree.graphics.dots.push([90.0,90.0]);
	tree.querry(&mut Dot{x:160_f64,y:230_f64});
	tree.graphics.dots.push([160.0,230.0]);
	tree.querry(&mut Dot{x:310_f64,y:230_f64});
	tree.graphics.dots.push([310.0,230.0]);
	tree.querry(&mut Dot{x:460_f64,y:330_f64});
	tree.graphics.dots.push([460.0,330.0]);
	tree.querry(&mut Dot{x:540_f64,y:340_f64});
	tree.graphics.dots.push([540.0,340.0]);
	tree.querry(&mut Dot{x:256_f64,y:140_f64});
	tree.graphics.dots.push([256.0,140.0]);
	 let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args,&tree);
        }
/*
        if let Some(args) = e.update_args() {
            app.update(&args,tree);
        }*/
    }
}
