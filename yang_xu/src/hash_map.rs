use std::collections::{BTreeMap, HashMap, HashSet};

#[test]
fn test1() {
	let mut m: HashMap<i32, i32> = HashMap::new();
	m.insert(1, 1);
	m.remove(&1);

	println!("{:#?}", m)
}

#[test]
fn test2() {
    let mut m: BTreeMap<i32,i32> = BTreeMap::new();
	m.insert(1,10);
	let v = m.get(&1);
	println!("{:?}",v);
}

#[test]
fn set1() {
    let mut s:HashSet<i32> = HashSet::new();
	s.insert(1);
	s.insert(200);
	println!("{:?}",s)
}