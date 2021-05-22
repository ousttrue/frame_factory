use cgmath::{One, Zero};
use std::{
    cell::{Ref, RefCell},
    default::Default,
    rc::{Rc, Weak},
};

use crate::Mesh;

pub struct Node {
    pub name: String,
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub scaling: cgmath::Vector3<f32>,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
    //
    pub mesh: Option<Rc<Mesh>>,
}

impl Node {
    pub fn new(name: &str) -> Node {
        Node {
            name: String::from(name),
            position: Zero::zero(),
            rotation: One::one(),
            scaling: cgmath::Vector3::new(1f32, 1f32, 1f32),
            parent: Default::default(),
            children: Default::default(),
            mesh: None,
        }
    }

    pub fn get_parent(&self) -> Option<Rc<Node>> {
        self.parent.borrow().upgrade()
    }

    pub fn get_children<'a>(&'a self) -> Ref<'a, Vec<Rc<Node>>> {
        self.children.borrow()
    }

    pub fn add_child(parent: &Rc<Node>, child: &Rc<Node>) {
        child.parent.replace(Rc::downgrade(parent));
        parent.children.borrow_mut().push(child.clone());
    }
}
