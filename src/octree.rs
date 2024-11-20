use core::f32;
use std::{cmp::*, collections::VecDeque};

use bevy::{color::Color, math::{self, bounding::RayCast3d, NormedVectorSpace}, prelude::*};

pub struct Octant {
    pub children: Option<Vec<Octant>>,
    pub color: Color,
    pub position: Vec3,
    pub depth: u32
}

impl Default for Octree {
    fn default() -> Self {
        Octree::new()
    }
}

impl Octant {
    fn new(pos: Vec3, depth: u32, color: Color) -> Self {
        Self {
            children: None,
            color,
            position: pos,
            depth
        }
    }

    pub fn get_size(&self) -> f32 {
        return Self::get_size_from_depth(self.depth)
    }

    // returns if the octant has children
    pub fn is_leaf(&self) -> bool {
        return self.children.is_none()
    }

    // returns if the octant has a visible color
    // raycast needs to ignore invisible octants
    // by design, non-leaf nodes should not be visible
    pub fn is_visible(&self) -> bool {
        if self.is_leaf() {
            return self.color.alpha() > 0.0
        } else {
            // is not leaf, has children, not visible by design
            return false
        }
    }

    fn get_size_from_depth(depth: u32) -> f32 {
        1.0 / (1 << depth) as f32
    }

    pub fn create_children(&mut self) {
        let mut children: Vec<Octant> = Vec::with_capacity(8);
        for x in 0..2 {
            for y in 0..2 {
                for z in 0..2 {
                    let child_size = Self::get_size_from_depth(self.depth+1);
                    let pos = self.position + Vec3::from((x as f32, y as f32, z as f32)) * Vec3::from([child_size; 3]) - Vec3::from([child_size / 2.0; 3]);
                    children.push(Octant::new(pos, self.depth+1, self.color));
                }
            }
        }
        self.children = Some(children);
    }
}

#[derive(Resource)]
pub struct Octree {
    pub root: Octant
}

struct RaycastHit<'a> {
    pos: Vec3,
    octant: &'a mut Octant
}

impl Octree {
    fn new() -> Self {
        Self {
            root: Octant::new(Vec3::new(0.0, 0.0, 0.0), 0, Color::linear_rgba(0.0, 0.5, 0.0, 0.25))
        }
    }

    pub fn raycast(&mut self, ray: &mut Ray3d) -> Option<RaycastHit> {
        // find the intersection point of ray and bounding cube (-0.5, 0.5)
        // first find distance to cube using sdf
        let q = ray.origin.abs() - Vec3::from([0.5; 3]);
        let dist = q.max(Vec3::from([0.0; 3])).length() + q.x.max(q.y.max(q.z)).min(0.0);

        // traverse ray distance of dist to get intersection point between ray and bounding cube
        ray.origin = ray.get_point(dist);

        // check if root is leaf
        if self.root.is_leaf() {
            return Some(RaycastHit{
                pos: ray.origin,
                octant: &mut self.root
            })
        }
        
        let mut parent_stack: Vec<&mut Octant> = Vec::new();
        // push root onto parent stack
        parent_stack.push(&mut self.root);

        // traverse through octree
        while !parent_stack.is_empty() {
            let parent = parent_stack.pop().unwrap();

            if let Some(children) = &mut parent.children {
                // assuming intersecting_octant is nearest octant in children
                if let Some(intersecting_octant) = children.iter_mut().min_by(|a, b|
                    a.position.distance(ray.origin).total_cmp(&b.position.distance(ray.origin))) {
                        if intersecting_octant.is_leaf() {
                            // hit leaf node, so no children
                            // check if visible
                            if intersecting_octant.is_visible() {
                                // octant is visible so return raycasthit
                                return Some(RaycastHit{
                                    pos: ray.origin,
                                    octant: intersecting_octant,
                                })
                            } else {
                                // octant is not visible, so treat as empty space and advance ray
                                todo!()
                            }
                        } else {
                            // octant is not a leaf, so has children
                            // add octant onto parent_stack
                            parent_stack.push(intersecting_octant);
                        }
                }
            }
        }

        // if finished while loop then should be a miss
        return None
    }
}