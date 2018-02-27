use engine::core::ComponentBased;
use std::rc::Rc;
use engine::render::{Material, MeshBuffer};

use na::Vector3;

pub struct MeshSurface {
    pub buffer: Rc<MeshBuffer>,
    pub material: Rc<Material>,
}

pub struct Mesh {
    pub surfaces: Vec<Rc<MeshSurface>>,
}

impl ComponentBased for Mesh {}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            surfaces: Vec::new(),
        }
    }

    pub fn add_surface(&mut self, buffer: Rc<MeshBuffer>, material: Rc<Material>) {
        self.surfaces
            .push(Rc::new(MeshSurface { buffer, material }));
    }

    /// bounds return (vmin, vmax)
    pub fn bounds(&self) -> Option<(Vector3<f32>, Vector3<f32>)> {
        // TODO compute merged bounds
        self.surfaces[0].buffer.bounds()
    }
}
