#![allow(dead_code)]

pub struct Node {
    pub entity: ecs::Entity,
    pub children: Vec<Box<Node>>,
}

impl Node {
    pub fn new(entity: ecs::Entity) -> Self {
        Self { entity, children: Vec::new() }
    }

    pub fn add_child(&mut self, entity: ecs::Entity) {
        self.children.push(Box::new(Node::new(entity)));
    }
}

pub struct Scene {
    entities_count: usize,
    pub ecs: ecs::ECS,
    pub root: Node,
}

// TODO: figure out if the engine should directly use GLIUM types
//       (buffers and such) or if a resource manager could/should do this.

impl Scene {
    pub fn new() -> Self {
        Self {
            entities_count: 0usize,
            ecs: ecs::ECS::new(),
            root: Node::new(0usize),
        }
    }

    pub fn get_node(&mut self, _entity: ecs::Entity) -> Node {
        unimplemented!();
    }

    pub fn add_child(&mut self, _entity: ecs::Entity) {
        unimplemented!();
    }

    pub fn render(&mut self) {
        unimplemented!();
    }
}
