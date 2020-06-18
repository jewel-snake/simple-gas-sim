use std::vec::Vec;

struct Dot{
	x: u16,
	y: u16,
}

impl Copy for Dot {}

impl Clone for Dot {
	fn clone(&self) -> Dot{
		*self
	}
}

struct Qtree{
	x1: u16,
	x2: u16,
	y1: u16,
	y2: u16,
	content: Contents,
}

enum Contents{
	Children([Box<Qtree>; 4]),
	Elements(Vec<Dot>),
}

impl Contents{
	fn unwrap_children(&self) -> &[Box<Qtree>; 4] {
		match self {
			Contents::Children(A) => &A,
			Contents::Elements(_) => panic!("cannot unwarp Children node!"),
		}
	}
	fn unwrap_elements(&self) -> &Vec<Dot> {
		match self {
			Contents::Children(_) => panic!("cannot unwrap Elements vector!"),
			Contents::Elements(V) => &V,
		}
	} 
}

impl Qtree{
	fn new(x1: u16, y1: u16, x2: u16, y2: u16) -> Qtree{
		Qtree{
			x1,
			x2,
			y1,
			y2,
			content: Contents::Elements(Vec::new()),
		}
	}
	fn overflowed(&self) -> bool{
		let mut a = false;
		if self.content.unwrap_elements().len() > 5 {
			a = !a;
		}
		a
	}

	fn handle_overflow(self) {
		let a = *self.content.unwrap_elements();
		let hmiddle = (self.y1 + self.y2)/2;
		let wmiddle = (self.x1 + self.x2)/2;
		self.content = Contents::Children([
			Box::new(Qtree::new(wmiddle,self.y1,self.x2,hmiddle)),
			Box::new(Qtree::new(self.x1,self.y1,wmiddle,hmiddle)),
			Box::new(Qtree::new(self.x1,hmiddle,wmiddle,self.y2)),
			Box::new(Qtree::new(wmiddle,hmiddle,self.x2,self.y2))]);
		for k in a {
			if k.x > wmiddle {
				if k.x > hmiddle{
					self.content.unwrap_children()[3].querry(k);
				}else{
					self.content.unwrap_children()[0].querry(k);
				}
			}else{
				if k.x > hmiddle{
					self.content.unwrap_children()[2].querry(k);
				}else{
					self.content.unwrap_children()[1].querry(k);
				}
			}
		}
	}
	fn querry(&self, elem: Dot) {
		match self.content {
			Contents::Children(C) => {
				let hmiddle = (self.y1 + self.y2)/2;
				let wmiddle = (self.x1 + self.x2)/2;
				if elem.x > wmiddle {
					if elem.y > hmiddle {
						C[3].querry(elem);
					}else{
						C[0].querry(elem);
					}
				}else{
					if elem.y > hmiddle {
						C[2].querry(elem);
					}else{
						C[1].querry(elem);
					}
				}
			},
			Contents::Elements(E) => {
				E.push(elem);
				if self.overflowed() {
					self.handle_overflow();
				}
			},
		}
	}
}

fn main() {
	let mut tree = Qtree::new(0,0,600,400);
	tree.querry(Dot{x:5,y:5});
}
