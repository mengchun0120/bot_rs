use crate::misc::*;
use bevy::prelude::*;

pub struct QueryMapper<'w, 's, 'a, 'b, T>
where
    T: Component
{
    query: &'a Query<'w, 's, &'b T>,
}

pub struct QueryMapperByMut<'w, 's, 'a, 'b, T>
where
    T: Component<Mutability = bevy::ecs::component::Mutable>
{
    query: &'a Query<'w, 's, &'b mut T>,
}

pub struct MutQueryMapper<'w, 's, 'a, 'b, T>
where
    T: Component<Mutability = bevy::ecs::component::Mutable>
{
    query: &'a mut Query<'w, 's, &'b mut T>,
}

impl<'w, 's, 'a, 'b, T> QueryMapper<'w, 's, 'a, 'b, T>
where
    T: Component
{
    pub fn new(query: &'a Query<'w, 's, &'b T>) -> Self {
        Self { query }
    }
}

impl<'w, 's, 'a, 'b, T> Mapper<Entity, T> for QueryMapper<'w, 's, 'a, 'b, T>
where
    T: Component
{
    fn get(&self, entity: Entity) -> Option<&T> {
        self.query.get(entity).ok()
    }
}

impl<'w, 's, 'a, 'b, T> QueryMapperByMut<'w, 's, 'a, 'b, T>
where
    T: Component<Mutability = bevy::ecs::component::Mutable>
{
    pub fn new(query: &'a Query<'w, 's, &'b mut T>) -> Self {
        Self { query }
    }
}

impl<'w, 's, 'a, 'b, T> Mapper<Entity, T> for QueryMapperByMut<'w, 's, 'a, 'b, T>
where
    T: Component<Mutability = bevy::ecs::component::Mutable>
{
    fn get(&self, entity: Entity) -> Option<&T> {
        self.query.get(entity).ok()
    }
}

impl<'w, 's, 'a, 'b, T> MutQueryMapper<'w, 's, 'a, 'b, T>
where
    T: Component<Mutability = bevy::ecs::component::Mutable>
{
    pub fn new(query: &'a mut Query<'w, 's, &'b mut T>) -> Self {
        Self { query }
    }
}

impl<'w, 's, 'a, 'b, T> MutMapper<Entity, T> for MutQueryMapper<'w, 's, 'a, 'b, T>
where
    T: Component<Mutability = bevy::ecs::component::Mutable>
{
    fn get(&mut self, entity: Entity) -> Option<&mut T> {
        self.query.get_mut(entity).ok().map(|t| t.into_inner())
    }
}