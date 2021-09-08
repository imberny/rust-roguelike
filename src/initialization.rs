use std::cmp::Ordering;

use bevy_ecs::{prelude::*, schedule::ShouldRun};

use crate::{
    actor::{self, Action, Activity, Actor},
    game::{Game, ECS},
    generator::map::build_map,
    player::{self, systems::is_player_waiting_for_input, Player, PlayerInput},
    rendering,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum CoreStage {
    First,
    Decision,
    Update,
    PreUpdate,
    PostUpdate,
    Animate,
    Draw,
    Last,
}

pub fn init_game() -> ECS {
    let mut world = World::new();
    world.insert_resource::<Game>(Game {
        is_waiting_for_input: false,
    });
    world.insert_resource(PlayerInput::default());
    world.insert_resource(TurnBasedTime::default());
    world.insert_resource(TurnBasedGame {
        state: RunningState::Paused,
    });

    let mut game_logic = create_game_schedule();
    actor::register(&mut game_logic);
    player::register(&mut game_logic);
    let mut rendering = Schedule::default();
    let draw_stage = SystemStage::parallel();
    rendering.add_stage(CoreStage::Draw, draw_stage);
    rendering::register(&mut rendering);

    build_map(&mut world);

    ECS {
        world,
        game_logic,
        rendering,
    }
}

fn create_game_schedule() -> Schedule {
    let first_stage = SystemStage::parallel();
    let decision_stage = SystemStage::parallel();
    let pre_update_stage = SystemStage::parallel();
    let update_stage = SystemStage::parallel();
    let post_update_stage = SystemStage::parallel();
    let last_stage = SystemStage::parallel();
    let mut schedule = Schedule::default();
    schedule
        .add_stage(CoreStage::First, first_stage)
        .add_stage_after(CoreStage::First, CoreStage::Decision, decision_stage)
        .add_stage_after(CoreStage::Decision, CoreStage::PreUpdate, pre_update_stage)
        .add_stage_after(CoreStage::PreUpdate, CoreStage::Update, update_stage)
        .add_stage_after(CoreStage::Update, CoreStage::PostUpdate, post_update_stage)
        .add_stage_after(CoreStage::PostUpdate, CoreStage::Last, last_stage);
    schedule
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_run_criteria(is_game_running.system())
                .with_run_criteria(is_player_waiting_for_input.system())
                .with_system(pause_game.system()),
        )
        .add_system_set_to_stage(
            CoreStage::Last,
            SystemSet::new()
                .with_run_criteria(is_game_running.system())
                .with_system(advance_time.system()),
        );
    schedule
}

#[derive(Debug, Clone, Copy)]
pub enum RunningState {
    Running,
    Paused,
}

#[derive(Debug, Clone, Copy)]
pub struct TurnBasedGame {
    pub state: RunningState,
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

fn advance_time(mut time: ResMut<TurnBasedTime>, query: Query<&Activity>) {
    if let Some(shortest_activity) = query.iter().min_by(order_by_time_left) {
        time.time += shortest_activity.time_to_complete;
        time.delta_time = shortest_activity.time_to_complete;
        println!("Progressing time by {}", time.delta_time);
    }
}

fn is_game_running(game: Res<TurnBasedGame>) -> ShouldRun {
    match game.state {
        RunningState::Running => ShouldRun::Yes,
        RunningState::Paused => ShouldRun::No,
    }
}

fn pause_game(mut turn_based_game: ResMut<TurnBasedGame>) {
    turn_based_game.state = RunningState::Paused;
}

pub fn resume_game(mut turn_based_game: ResMut<TurnBasedGame>) {
    turn_based_game.state = RunningState::Running;
    println!("Resuming game");
}
