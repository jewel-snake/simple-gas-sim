pub mod logic{
	use std::vec::Vec;
	use std::cell::RefCell;

	pub struct Dot{
		pub x: f64,
		pub y: f64,
	}

	impl Copy for Dot {}

	impl Clone for Dot {
		fn clone(&self) -> Dot{
			*self
		}
	}

	pub struct Qtree{
		pub x1: f64,
		pub x2: f64,
		pub y1: f64,
		pub y2: f64,
		pub content: Contents,
		pub graphics :GrInfo,
	}

	pub struct GrInfo{
		pub dots: Vec<[f64;2]>,
		pub lines: Vec<[[f64;2];2]>,
	} 

	pub enum Contents{
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
		pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Qtree{
			Qtree{
				x1,
				x2,
				y1,
				y2,
				content: Contents::Elements(RefCell::new(Vec::new())),
				graphics: GrInfo{
					dots: Vec::new(),
					lines: Vec::new(),
				},
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
			let a = self.content.unwrap_elements().borrow().clone();
			let hmiddle = (self.y1 + self.y2)/2.0;
			let wmiddle = (self.x1 + self.x2)/2.0;
			self.content = Contents::Children([
				Box::new(RefCell::new(Qtree::new(wmiddle,self.y1,self.x2,hmiddle))),
				Box::new(RefCell::new(Qtree::new(self.x1,self.y1,wmiddle,hmiddle))),
				Box::new(RefCell::new(Qtree::new(self.x1,hmiddle,wmiddle,self.y2))),
				Box::new(RefCell::new(Qtree::new(wmiddle,hmiddle,self.x2,self.y2)))]);
			self.graphics.lines.push([[self.x1,hmiddle],[self.x2,hmiddle]]);
			self.graphics.lines.push([[wmiddle,self.y1],[wmiddle,self.y2]]);
			for k in &*a {
				if k.x > wmiddle {
					if k.x > hmiddle {
						self.content.unwrap_children()[3].borrow_mut().querry(&k);
					}else{
						self.content.unwrap_children()[0].borrow_mut().querry(&k);
					}
				}else{
					if k.x > hmiddle {
						self.content.unwrap_children()[2].borrow_mut().querry(&k);
					}else{
						self.content.unwrap_children()[1].borrow_mut().querry(&k);
					}
				}
			}
		}
		pub fn querry(&mut self, elem: &Dot) {
			match &mut self.content {
				Contents::Children(c) => {
					let hmiddle = (self.y1 + self.y2)/2.0;
					let wmiddle = (self.x1 + self.x2)/2.0;
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
}
mod logic2{
	use std::vec::Vec;
	#[derive(Copy,Clone)]
	struct Dot{
		x: f64,
		y: f64,
	}

	enum Contents{
		Elements(Vec<Dot>),
		Children([Box<Qtree>;4]),
	}

	struct Qtree{
		x1: f64,
		y1: f64,
		x2: f64,
		y2: f64,
		content: Contents,
	}

	impl Qtree{
		fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Qtree {
			Qtree{
				x1,
				y1,
				x2,
				y2,
				content: Contents::Elements(Vec::new()),
			}
		}
		fn overflowed(&self) -> bool {
			match &self.content {
				Contents::Elements(e) => {
					if e.len() > 5{
						return true
					}
					return false
				},
				Contents::Children(_) => {
					true
				},
			}
		}
		fn handle_overflow(&mut self) {
			let elems = match &self.content {
				Contents::Children(_) => panic!("Error! Contntent is already consists of nodes!"),
				Contents::Elements(e) => e.clone(),
			};
			let wmiddle = (self.x2 + self.x1) / 2.0;
			let hmiddle = (self.y2 + self.y1) / 2.0;
			self.content = Contents::Children([
				Box::new(Qtree::new(wmiddle,self.y1,self.x2,hmiddle)),
				Box::new(Qtree::new(self.x1,self.y1,wmiddle,hmiddle)),
				Box::new(Qtree::new(self.x1,hmiddle,wmiddle,self.y2)),
				Box::new(Qtree::new(wmiddle,hmiddle,self.x2,self.y2)),
			]);
			match &mut self.content {
				Contents::Children(c) => {
					for elem in elems {
						if elem.x > wmiddle {
							if elem.y > hmiddle {
								c[3].querry(elem);
							}else{
								c[0].querry(elem);
							}
						}else{
							if elem.y > hmiddle {
								c[2].querry(elem);
							}else{
								c[1].querry(elem);
							}
						}
					}
				},
				Contents::Elements(_) => panic!("Error! Content is still a vector!"),
			}
		}
		fn querry(&mut self, elem: Dot) {
			match &mut self.content {
				Contents::Elements(e) => {
					e.push(elem);
					if e.len() > 5 {
						self.handle_overflow();
					}
				},
				Contents::Children(c) => {
					let wmiddle = (self.x2 + self.x1) / 2.0;
					let hmiddle = (self.y2 + self.y1) / 2.0;
					if elem.x > wmiddle {
						if elem.y > hmiddle {
							c[3].querry(elem);
						}else{
							c[0].querry(elem);
						}
					}else{
						if elem.y > hmiddle {
							c[2].querry(elem);
						}else{
							c[1].querry(elem);
						}
					}
				},
			}
		}
	}
}