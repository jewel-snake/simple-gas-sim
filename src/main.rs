use std::vec::Vec;
use std::cell::RefCell;

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
	Children([Box<RefCell<Qtree>>; 4]),
	Elements(RefCell<Vec<Dot>>),
}

impl Contents{
	fn unwrap_children(&self) -> &[Box<RefCell<Qtree>>; 4] {
		match self {
			Contents::Children(a) => &a,
			Contents::Elements(_) => panic!("cannot unwarp Children node!"),
		}
	}
	fn unwrap_elements(&self) -> &RefCell<Vec<Dot>> {
		match self {
			Contents::Children(_) => panic!("cannot unwrap Elements vector!"),
			Contents::Elements(v) => &v,
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
			content: Contents::Elements(RefCell::new(Vec::new())),
		}
	}
	fn overflowed(&self) -> bool{
		let mut a = false;
		if self.content.unwrap_elements().borrow().len() > 5 {
			a = !a;
		}
		a
	}

	fn handle_overflow(&mut self) {
		let a = self.content.unwrap_elements().clone();
		let hmiddle = (self.y1 + self.y2)/2;
		let wmiddle = (self.x1 + self.x2)/2;
		self.content = Contents::Children([
			Box::new(RefCell::new(Qtree::new(wmiddle,self.y1,self.x2,hmiddle))),
			Box::new(RefCell::new(Qtree::new(self.x1,self.y1,wmiddle,hmiddle))),
			Box::new(RefCell::new(Qtree::new(self.x1,hmiddle,wmiddle,self.y2))),
			Box::new(RefCell::new(Qtree::new(wmiddle,hmiddle,self.x2,self.y2)))]);
		for k in &*a.borrow() {
			if k.x > wmiddle {
				if k.x > hmiddle{
					self.content.unwrap_children()[3].borrow_mut().querry(&k);
				}else{
					self.content.unwrap_children()[0].borrow_mut().querry(&k);
				}
			}else{
				if k.x > hmiddle{
					self.content.unwrap_children()[2].borrow_mut().querry(&k);
				}else{
					self.content.unwrap_children()[1].borrow_mut().querry(&k);
				}
			}
		}
	}
	fn querry(&mut self, elem: &Dot) {
		match &self.content {
			Contents::Children(c) => {
				let hmiddle = (self.y1 + self.y2)/2;
				let wmiddle = (self.x1 + self.x2)/2;
				if elem.x > wmiddle {
					if elem.y > hmiddle {
						c[3].borrow_mut().querry(elem);
					}else{
						c[0].borrow_mut().querry(elem);
					}
				}else{
					if elem.y > hmiddle {
						c[2].borrow_mut().querry(elem);
					}else{
						c[1].borrow_mut().querry(elem);
					}
				}
			},
			Contents::Elements(e) => {
				e.borrow_mut().push(*elem);
				if self.overflowed() {
					self.handle_overflow();
				}
			},
		}
	}
}

fn main() {
	let mut tree = Qtree::new(0,0,600,400);
	tree.querry(&Dot{x:5,y:5});
}
