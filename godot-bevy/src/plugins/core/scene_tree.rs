use std::marker::PhantomData;

use bevy::{
    app::{App, Plugin},
    ecs::system::{NonSendMut, SystemParam},
};
use godot::{
    classes::{Engine, SceneTree},
    obj::Gd,
};

pub struct GodotSceneTreePlugin;

impl Plugin for GodotSceneTreePlugin {
    fn build(&self, app: &mut App) {
        app.init_non_send_resource::<SceneTreeRefImpl>();
    }
}

#[derive(SystemParam)]
pub struct SceneTreeRef<'w, 's> {
    gd: NonSendMut<'w, SceneTreeRefImpl>,
    phantom: PhantomData<&'s ()>,
}

impl<'w, 's> SceneTreeRef<'w, 's> {
    pub fn get(&mut self) -> Gd<SceneTree> {
        self.gd.0.clone()
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub(crate) struct SceneTreeRefImpl(Gd<SceneTree>);

impl SceneTreeRefImpl {
    fn get_ref() -> Gd<SceneTree> {
        Engine::singleton()
            .get_main_loop()
            .unwrap()
            .cast::<SceneTree>()
    }
}

impl Default for SceneTreeRefImpl {
    fn default() -> Self {
        Self(Self::get_ref())
    }
}
