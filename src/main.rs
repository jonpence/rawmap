use anymap::any::{Any, UncheckedAnyExt};
use anymap::raw::RawMap;
use std::any::TypeId;
use std::collections::HashMap;

trait Component: std::fmt::Debug {}

#[derive(Debug)]
struct ComponentA;

#[derive(Debug)]
struct ComponentB;

#[derive(Debug)]
struct ComponentC;

#[derive(Debug)]
struct ComponentD;

#[derive(Debug)]
struct ComponentE;

impl Component for ComponentA {}
impl Component for ComponentB {}
impl Component for ComponentC {}
impl Component for ComponentD {}
impl Component for ComponentE {}

struct ComponentMap {
    type_map: RawMap<dyn Any>,
    downcast_map: HashMap<TypeId, Box<fn(&dyn Any) -> &dyn Component>>,
}

impl ComponentMap {
    fn new() -> Self {
        Self {
            type_map: RawMap::new(),
            downcast_map: HashMap::new(),
        }
    }

    fn downcast<T: Component + 'static>(any: &dyn Any) -> &dyn Component {
        unsafe { any.downcast_ref_unchecked::<T>() }
    }

    fn insert<T: Component + 'static>(&mut self, val: T) {
        let tid = TypeId::of::<T>();
        unsafe {
            self.type_map.insert(tid, Box::new(val));
        }
        self.downcast_map.insert(tid, Box::new(Self::downcast::<T>));
    }

    fn get(&self, tid: &TypeId) -> &dyn Component {
        let df = self.downcast_map.get(tid).unwrap();
        let c = self.type_map.get(tid).unwrap();
        df(c)
    }

    fn iter(&self) -> Box<dyn Iterator<Item = &dyn Component> + '_> {
        Box::new(self.type_map.iter().map(|x| {
            let tid = x.type_id();
            let df = self.downcast_map.get(&tid).unwrap();
            df(x)
        }))
    }
}

fn main() {
    let mut map = ComponentMap::new();
    let a = ComponentA;
    let b = ComponentB;
    let c = ComponentC;
    let d = ComponentD;
    let e = ComponentE;

    map.insert(a);
    map.insert(b);
    map.insert(c);
    map.insert(d);
    map.insert(e);

    for x in map.iter() {
        println!("{:?}", x);
    }
}
