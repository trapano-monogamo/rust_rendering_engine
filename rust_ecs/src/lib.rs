#![allow(dead_code)]
#![allow(non_snake_case)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::any::{Any, TypeId};

pub trait Component {
    fn as_any(&mut self) -> &mut dyn Any;
    fn type_id(&'static self) -> TypeId {
        return std::any::TypeId::of::<Self>();
    }
}

pub trait System {
    fn call(&mut self, world: &mut World);
}

pub struct World {
    entities: Vec<usize>,
    components: HashMap<usize, Vec<Box<dyn Component>>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            components: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, entity: usize) {
        self.entities.push(entity);
    }

    pub fn add_component<T>(&mut self, entity: usize, component: T)
        where T: Component + 'static
    {
        match self.components.get_mut(&entity) {
            Some(v) => { v.push(Box::new(component)) },
            None => { self.components.insert(entity, vec![Box::new(component)]); },
        }
    }

    pub fn get_component<T>(&mut self, entity: usize) -> Option<&mut T>
        where T: Component + 'static
    {
        return match self.components.get_mut(&entity) {
            Some(components) => {
                for c in components {
                    if let Some(res) = c.as_any().downcast_mut::<T>() {
                        return Some(res);
                    }
                }
                None
            }, 
            None => None,
        }
    }

    pub fn get_components<T>(&mut self) -> Vec<&mut T> 
        where T: Component + 'static
    {
        let mut res: Vec<&mut T> = Vec::new();
        for (_, values) in self.components.iter_mut() {
            for component in values.iter_mut() {
                if let Some(c) = component.as_any().downcast_mut::<T>() {
                    res.push(c);
                }
            }
        }
        return res;
    }

    pub fn remove_entity(&mut self, entity: usize) {
        if let Some(index) = self.entities.iter().position(|&x| x == entity) {
            self.entities.remove(index);
        }
    }

    // pub fn remove_component<T>(&mut self, entity: usize)
    //     where T: Component + 'static
    // {
    //     if let Some(components) = self.components.get_mut(&entity) {
    //         for component in components.iter() {
    //             if let Some(c) = component.as_any().downcast_mut::<T>() {
    //                 if let Some(index) = 
    //             }
    //         }
    //     }
    // }
}

pub struct ECS {
    // startup_systems: Vec<Box<dyn System>>,
    systems: Vec<Box<dyn System>>,
    world: RefCell<World>,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
            world: RefCell::new(World::new()),
        }
    }

    pub fn update(&mut self, ) {
        for sys in self.systems.iter_mut() {
            sys.call(self.world.get_mut());
        }
    }

    pub fn add_entity(&mut self, entity: usize) { self.world.get_mut().add_entity(entity); }
    pub fn add_component<T>(&mut self, entity: usize, component: T) where T: Component + 'static { self.world.get_mut().add_component(entity, component); }
    pub fn add_system(&mut self, system: Box<dyn System>) { self.systems.push(system); }

    pub fn get_component<T>(&mut self, entity: usize) -> Option<&mut T> where T: Component + 'static { return self.world.get_mut().get_component(entity); }
    pub fn get_components<T>(&mut self) -> Vec<&mut T> where T: Component + 'static { return self.world.get_mut().get_components::<T>(); }

    pub fn remove_entity(&mut self, entity: usize) { self.world.get_mut().remove_entity(entity) }
    // TODO:
    // pub fn remove_component<T>(&mut self, entity: usize) where T: Component + 'static { self.world.get_mut().remove_component::<T>(entity); }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[derive(Debug, PartialEq)]
    struct Position {
        x: f32,
        y: f32,
    }
    impl Component for Position {
        fn as_any(&mut self) -> &mut dyn Any { self }
    }

    #[derive(Debug, PartialEq)]
    struct Message {
        msg: String,
    }
    impl Component for Message {
        fn as_any(&mut self) -> &mut dyn Any { self }
    }
    impl Message {
        fn new(msg: &str) -> Self {
            Self { msg: String::from(msg) }
        }
        fn say(&self) -> i32 {
            println!("{}", self.msg);
            return 1;
        }
    }

    struct UpdatePositionSystem;
    impl System for UpdatePositionSystem {
        fn call(&mut self, world: &mut World) {
            let mut positions = world.get_components::<Position>();
            for p in positions.iter_mut() {
                p.x = 999.0f32;
            }
        }
    }

    #[test]
    fn add_components() {
        let mut ecs = ECS::new();
        ecs.add_entity(0);
        ecs.add_component::<Position>(0, Position{ x: 3.0f32, y: 2.0f32 });
        ecs.add_component::<Message>(0, Message::new("Hello, I'm chiara"));
    }

    #[test]
    fn get_components() {
        let mut ecs = ECS::new();
        ecs.add_entity(0);
        ecs.add_entity(1);
        ecs.add_component::<Position>(0, Position{ x: 3.0f32, y: 2.0f32 });
        ecs.add_component::<Message>(0, Message::new("Hello, I'm chiara"));
        ecs.add_component::<Position>(1, Position{ x: 9.0f32, y: 9.0f32 });
        assert_eq!(*ecs.get_component::<Position>(0).unwrap(), Position{ x: 3.0f32, y: 2.0f32 });
        assert_eq!(*ecs.get_component::<Position>(1).unwrap(), Position{ x: 9.0f32, y: 9.0f32 });
        assert_eq!(*ecs.get_component::<Message>(0).unwrap(), Message::new("Hello, I'm chiara"));
        assert_eq!(ecs.get_component::<Message>(1), None);
    }

    #[test]
    fn use_systems() {
        let mut ecs = ECS::new();
        ecs.add_entity(0);
        ecs.add_entity(1);
        ecs.add_component::<Position>(0, Position{ x: 3.0f32, y: 2.0f32 });
        ecs.add_component::<Message>(0, Message::new("Hello, I'm chiara"));
        ecs.add_component::<Position>(1, Position{ x: 9.0f32, y: 9.0f32 });
        ecs.add_system(Box::new(UpdatePositionSystem));
        ecs.update();
        assert_eq!(*ecs.get_component::<Position>(0).unwrap(), Position{ x: 999.0f32, y: 2.0f32 });
        assert_eq!(*ecs.get_component::<Position>(1).unwrap(), Position{ x: 999.0f32, y: 9.0f32 });
    }

    #[test]
    fn remove_entity() {
        let mut ecs = ECS::new();
        ecs.add_entity(0);
        ecs.remove_entity(0);
        assert_eq!(ecs.world.get_mut().entities.len(), 0);
    }

    #[test]
    fn remove_component() {
        unimplemented!();
    }

}
