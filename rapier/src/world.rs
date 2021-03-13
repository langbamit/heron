use bevy::prelude::*;

use crate::convert::IntoRapier;
use crate::rapier::dynamics::{JointSet, RigidBodySet};
use crate::rapier::geometry::{ColliderSet, InteractionGroups};
use crate::rapier::parry::query::Ray;
use crate::rapier::pipeline::QueryPipeline;

pub struct PhysicsWorld {
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    pub joints: JointSet,
    pub query: QueryPipeline,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self {
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            joints: JointSet::new(),
            query: QueryPipeline::default(),
        }
    }
}

impl PhysicsWorld {
    pub fn ray_cast(&self, origin: Vec3, direction: Vec3, length: f32) -> Option<(Entity, f32)> {
        self.query
            .cast_ray(
                &self.colliders,
                &Ray {
                    origin: origin.into_rapier(),
                    dir: direction.into_rapier(),
                },
                length,
                false,
                InteractionGroups::all(),
            )
            .and_then(|(handle, distance)| {
                self.colliders
                    .get(handle)
                    .map(|collider| (Entity::from_bits(collider.user_data as u64), distance))
            })
    }
}
