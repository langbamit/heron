use bevy_ecs::{Entity, Fetch, Query, QueryFilter, ReadOnlyFetch, WorldQuery};

/// An event fired when the collision state between two entities changed
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CollisionEvent {
    /// The two entity started to collide
    Started(Entity, Entity),

    /// The two entity no longer collide
    Stopped(Entity, Entity),
}

impl CollisionEvent {
    pub fn on_match<Q1, Q2, F1, F2, R>(
        self,
        query1: &mut Query<'_, Q1, F1>,
        query2: &mut Query<'_, Q2, F2>,
        action: impl Fn(<Q1::Fetch as Fetch<'_>>::Item, <Q2::Fetch as Fetch<'_>>::Item) -> R,
    ) -> Option<R>
    where
        Q1: WorldQuery,
        Q1::Fetch: ReadOnlyFetch,
        Q2: WorldQuery,
        F1: QueryFilter,
        F2: QueryFilter,
    {
        let (entity1, entity2) = self.entities();

        {
            if let (Ok(res1), Ok(res2)) = (query1.get_mut(entity1), query2.get_mut(entity2)) {
                return Some(action(res1, res2));
            }
        }
        {
            if let (Ok(res1), Ok(res2)) = (query1.get_mut(entity2), query2.get_mut(entity1)) {
                return Some(action(res1, res2));
            }
        }

        None
    }

    fn entities(self) -> (Entity, Entity) {
        match self {
            CollisionEvent::Started(e1, e2) => (e1, e2),
            CollisionEvent::Stopped(e1, e2) => (e1, e2),
        }
    }
}
