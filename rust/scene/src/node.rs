use cgmath::{One, Zero};
use std::{
    cell::{Ref, RefCell},
    default::Default,
    rc::{Rc, Weak},
};

use crate::Mesh;

pub enum Transform {
    Matrix(cgmath::Matrix4<f32>),
    TRS(
        cgmath::Vector3<f32>,
        cgmath::Quaternion<f32>,
        cgmath::Vector3<f32>,
    ),
}

impl Transform {
    pub fn matrix(&self) -> cgmath::Matrix4<f32> {
        match self {
            Self::Matrix(m) => m.clone(),
            Self::TRS(t, r, s) => {
                let t = cgmath::Matrix4::<f32>::from_translation(t.clone());
                let r = cgmath::Matrix4::from(r.clone());
                let s = cgmath::Matrix4::from_nonuniform_scale(s.x, s.y, s.z);

                let m =t * r * s;

                m
            }
        }
    }
}

pub struct Node {
    pub name: String,
    pub transform: Transform,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
    //
    pub mesh: Option<Rc<Mesh>>,
}

impl Node {
    pub fn new(name: &str) -> Node {
        Node {
            name: String::from(name),
            transform: Transform::TRS(
                Zero::zero(),
                One::one(),
                cgmath::Vector3::new(1f32, 1f32, 1f32),
            ),
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
