use bevy::prelude::*;
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(
    Debug, Clone, Copy, Default, States, Eq, PartialEq, Hash, IntoPrimitive, FromPrimitive,
)]
#[repr(u8)]
pub enum SimulState {
    #[default]
    Startup,
    StartupEnd,
    RunningWithoutGravity,
    RunningWithGravity,
    Cleanup,
}

impl SimulState {
    // CRUD-R: Superproperties

    pub fn is_running(self) -> bool {
        match self {
            Self::RunningWithoutGravity | Self::RunningWithGravity => true,
            _ => false,
        }
    }
    pub fn is_running_cond() -> impl Condition<()> {
        in_state(Self::RunningWithoutGravity).or_else(in_state(Self::RunningWithGravity))
    }

    // CRUD-R: Base properties

    pub fn is_startup(self) -> bool {
        self == Self::Startup
    }
    pub fn is_startup_end(self) -> bool {
        self == Self::StartupEnd
    }
    pub fn is_running_without_gravity(self) -> bool {
        self == Self::RunningWithoutGravity
    }
    pub fn is_running_with_gravity(self) -> bool {
        self == Self::RunningWithGravity
    }
    pub fn is_cleanup(self) -> bool {
        self == Self::Cleanup
    }

    // CRUD-U: Transitions

    pub fn forward(self) -> Self {
        (u8::from(self).wrapping_add(1)).into()
    }
    pub fn backward(self) -> Self {
        (u8::from(self).wrapping_sub(1)).into()
    }
}
