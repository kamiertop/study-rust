fn main() {}

#[test]
fn rc() {
	use std::rc::Rc;
	let r1 = Rc::new(1);
	println!("Rc::strong_count(&r1): {}", Rc::strong_count(&r1));
	let r2 = r1.clone();
	println!("Rc::strong_count(&r2): {}", Rc::strong_count(&r2));
}


// 内部可变性
#[test]
fn cell() {
	use std::cell::Cell;
	let data: Cell<i32> = Cell::new(10);
	let p = &data;
	let p1 = &data;
	p1.set(23);
	data.set(10);
	println!("p.get(): {}", p.get());
	p.set(100);
	println!("data: {:?}", data);
}

#[test]
fn ref_cell() {
	use std::cell::RefCell;
	let shared_vec: RefCell<Vec<isize>> = RefCell::new(vec![1, 2, 3]);
	let shared1 = &shared_vec;
	let shared2 = &shared_vec;
	shared1.borrow_mut().push(1);
	println!("shared_vec.borrow(): {:?}", shared_vec.borrow());

	shared2.borrow_mut().push(20);
	println!("shared_vec.borrow(): {:?}", shared_vec.borrow());
}