pub use entity_not_spawned::EntityNotSpawned;
pub mod entity_not_spawned {
    use derive_more::Constructor;

    #[derive(Debug, Constructor, thiserror::Error)]
    #[error("Entity corresponding to this `{}` hasn't been spawned.", std::any::type_name::<crate::simul::Sector>())]
    pub struct EntityNotSpawned {}
}
pub use entity_already_present::EntityAlreadySpawned;
pub mod entity_already_present {
    use derive_more::Constructor;

    #[derive(Debug, Constructor, thiserror::Error)]
    #[error("Entity corresponding to this `{}` has already been spawned.", std::any::type_name::<crate::simul::Sector>())]
    pub struct EntityAlreadySpawned {}
}
