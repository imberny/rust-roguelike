use std::{cmp::Ordering, fmt::Debug};

use bevy_ecs::{event::Events, prelude::*, schedule::ShouldRun};

use crate::{
    actor::{
        self,
        player::{
            systems::{handle_player_input, is_input_valid},
            Player, PlayerInput,
        },
        Activity,
    },
    game::ECS,
    generator::map::build_map,
    rendering,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum CoreStage {
    First,
    Decision,   // AI and player choose action
    PreUpdate,  // Prepare for update
    Update,     // Run update systems
    PostUpdate, // React to update
    Draw,
    Last,
}

pub fn init_game() -> ECS {
    let mut world = World::new();

    init_resources(&mut world);

    let mut input = Schedule::default();
    input
        .set_run_criteria(is_player_waiting_for_input.system())
        .add_stage(
            "input",
            SystemStage::single(
                handle_player_input
                    .system()
                    .with_run_criteria(is_input_valid.system()),
            ),
        );

    let mut game_logic = create_game_schedule();
    let mut rendering = create_render_schedule();

    register_modules(&mut game_logic, &mut rendering);

    build_map(&mut world);

    ECS {
        world,
        input,
        game_logic,
        rendering,
    }
}

fn register_modules(game_logic: &mut Schedule, rendering: &mut Schedule) {
    actor::register(game_logic);
    // player::register(game_logic);
    rendering::register(rendering);
}

fn create_game_schedule() -> Schedule {
    let mut schedule = Schedule::default();
    schedule
        .add_stage(CoreStage::First, SystemStage::parallel())
        .add_stage_after(
            CoreStage::First,
            CoreStage::Decision,
            SystemStage::parallel(),
        )
        .add_stage_after(
            CoreStage::Decision,
            CoreStage::PreUpdate,
            SystemStage::parallel(),
        )
        .add_stage_after(
            CoreStage::PreUpdate,
            CoreStage::Update,
            SystemStage::parallel(),
        )
        .add_stage_after(
            CoreStage::Update,
            CoreStage::PostUpdate,
            SystemStage::parallel(),
        )
        .add_stage_after(
            CoreStage::PostUpdate,
            CoreStage::Last,
            SystemStage::parallel(),
        );
    schedule
        .set_run_criteria(is_player_busy.system())
        .add_system_to_stage(CoreStage::PreUpdate, advance_time.system());
    schedule
}

fn init_resources(world: &mut World) {
    world.insert_resource(Events::<TimeProgressionEvent>::default());
    world.insert_resource(PlayerInput::default());
    world.insert_resource(TurnBasedTime::default());
}

fn create_render_schedule() -> Schedule {
    let mut rendering = Schedule::default();
    let draw_stage = SystemStage::parallel();
    rendering.add_stage(CoreStage::Draw, draw_stage);
    rendering
}

#[derive(Debug, Default)]
pub struct TurnBasedTime {
    pub time: i32,
    pub delta_time: i32,
}

fn order_by_time_left<'r, 's>(activity1: &'r &Activity, activity2: &'s &Activity) -> Ordering {
    let delta = activity1.time_to_complete - activity2.time_to_complete;
    if 0 > delta {
        Ordering::Less
    } else if 0 < delta {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

pub struct TimeProgressionEvent {
    pub delta_time: i32,
}

fn advance_time(mut time: ResMut<TurnBasedTime>, activities: Query<&Activity>) {
    // TODO: use events
    if let Some(shortest_activity) = activities.iter().min_by(order_by_time_left) {
        time.time += shortest_activity.time_to_complete;
        time.delta_time = shortest_activity.time_to_complete;
        println!("Progressing time by {}", time.delta_time);
    }
}

fn clear_delta_time(mut time: ResMut<TurnBasedTime>) {
    time.delta_time = 0;
}

pub fn is_player_waiting_for_input(player: Query<&Player, With<Activity>>) -> ShouldRun {
    match player.single() {
        Ok(_) => ShouldRun::No,
        Err(_) => ShouldRun::Yes,
    }
}

pub fn is_player_busy(player: Query<&Player, With<Activity>>) -> ShouldRun {
    match player.single() {
        Ok(_) => ShouldRun::YesAndCheckAgain,
        Err(_) => ShouldRun::No,
    }
}
