use crate::{
    actor::{
        self,
        player::{
            self,
            systems::{is_player_busy, is_player_waiting_for_input},
        },
    },
    core::*,
    game::ECS,
    generator::map::build_map,
    rendering,
};
use bevy_ecs::{event::Events, prelude::*};

pub fn init_game() -> ECS {
    let mut ecs = ECS {
        world: create_world(),
        input: create_input_schedule(),
        game_logic: create_game_schedule(),
        rendering: create_render_schedule(),
    };

    register_modules(&mut ecs);

    ecs
}

fn create_world() -> World {
    let mut world = World::new();
    world.insert_resource(Events::<TimeProgressionEvent>::default());
    world.insert_resource(TurnBasedTime::default());

    build_map(&mut world);

    world
}

fn create_input_schedule() -> Schedule {
    let mut schedule = Schedule::default();
    schedule
        .set_run_criteria(is_player_waiting_for_input.system())
        .add_stage(InputStage::Poll, SystemStage::parallel())
        .add_stage_after(
            InputStage::Poll,
            InputStage::Handle,
            SystemStage::parallel(),
        );
    schedule
}

fn create_game_schedule() -> Schedule {
    let mut schedule = Schedule::default();
    schedule
        .add_stage(TurnGameStage::First, SystemStage::parallel())
        .add_stage_after(
            TurnGameStage::First,
            TurnGameStage::Decision,
            SystemStage::parallel(),
        )
        .add_stage_after(
            TurnGameStage::Decision,
            TurnGameStage::PreUpdate,
            SystemStage::parallel(),
        )
        .add_stage_after(
            TurnGameStage::PreUpdate,
            TurnGameStage::Update,
            SystemStage::parallel(),
        )
        .add_stage_after(
            TurnGameStage::Update,
            TurnGameStage::PostUpdate,
            SystemStage::parallel(),
        )
        .add_stage_after(
            TurnGameStage::PostUpdate,
            TurnGameStage::Last,
            SystemStage::parallel(),
        );
    schedule
        .set_run_criteria(is_player_busy.system())
        .add_system_to_stage(TurnGameStage::PreUpdate, advance_time.system());
    schedule
}

fn create_render_schedule() -> Schedule {
    let mut rendering = Schedule::default();
    let draw_stage = SystemStage::parallel();
    rendering.add_stage(RenderingStage::Draw, draw_stage);
    rendering
}

fn register_modules(ecs: &mut ECS) {
    actor::register(ecs);
    player::register(ecs);
    rendering::register(ecs);
}
