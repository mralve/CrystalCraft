use amethyst::{
    core::{
        math::{Point3, Vector2, Vector3},
        transform::Transform,
    },
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    utils::removal::exec_removal,
    utils::removal::Removal,
    window::ScreenDimensions,
};
use amethyst_tiles::TileMap;

use crate::miscfunc;
use crate::systems;

pub struct CrystalCraft;

impl SimpleState for CrystalCraft {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Removal<i32>>();
        // Remove all entities with the RemovalId value of Something.
        exec_removal(&world.entities(), &world.read_storage(), -1);

        // Force the world to be up to date. This is normally called automatically at the end of the
        // frame by amethyst.
        world.maintain();
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        miscfunc::init_camera(world, &dimensions, 1.0);

    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                //info!("handling key event: {:?}", event);
            }
        }

        // Keep going
        Trans::None
    }
}

