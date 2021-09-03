use bevy_ecs::{prelude::*, schedule::ShouldRun};

use crate::{
    actor::{
        ai::systems::monster_ai,
        systems::{apply_player_viewsheds, process_move_actions, update_viewsheds},
        Action, Actor,
    },
    game::{Game, ECS},
    generator::map::build_map,
    player::{systems::handle_player_input, Player, PlayerInput},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum SystemGroup {
    Input,
    Actor,
    Action,
    EndTurn,
    Animate,
    Visualize,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum CoreStage {
    Update,
    Render,
}

pub fn init_game() -> ECS {
    let mut world = World::new();
    world.insert_resource::<Game>(Game {
        is_waiting_for_input: false,
    });
    world.insert_resource::<PlayerInput>(PlayerInput::default());

    let schedule = create_ecs_schedule();

    build_map(&mut world);

    ECS { world, schedule }
}

pub fn create_ecs_schedule() -> Schedule {
    let mut update_stage = SystemStage::parallel();
    update_stage
        .set_run_criteria(is_not_waiting_for_input.system())
        .add_system_set(
            SystemSet::new()
                .label(SystemGroup::Input)
                .with_system(handle_player_input.system()),
        )
        .add_system_set(
            SystemSet::new()
                .after(SystemGroup::Input)
                .label(SystemGroup::Actor)
                .with_system(monster_ai.system()),
        )
        .add_system_set(
            SystemSet::new()
                .after(SystemGroup::Actor)
                .label(SystemGroup::Action)
                .with_system(process_move_actions.system()),
        )
        .add_system_set(
            SystemSet::new()
                .after(SystemGroup::Action)
                .label(SystemGroup::EndTurn)
                .with_system(update_viewsheds.system())
                .with_system(end_player_turn.system()),
        );
    let mut render_stage = SystemStage::parallel();
    render_stage
        .add_system_set(SystemSet::new().label(SystemGroup::Animate))
        .add_system_set(
            SystemSet::new()
                .after(SystemGroup::Animate)
                .label(SystemGroup::Visualize)
                .with_system(apply_player_viewsheds.system()),
        );
    let mut schedule = Schedule::default();
    schedule
        .add_stage(CoreStage::Update, update_stage)
        .add_stage_after(CoreStage::Update, CoreStage::Render, render_stage);
    schedule
}

fn is_not_waiting_for_input(game: Res<Game>) -> ShouldRun {
    if game.is_waiting_for_input {
        ShouldRun::No
    } else {
        ShouldRun::Yes
    }
}

fn end_player_turn(mut game: ResMut<Game>, players: Query<&Actor, With<Player>>) {
    game.is_waiting_for_input = players.iter().any(|actor| match actor.action {
        Action::None => true,
        Action::Move(_) => false,
    });
}
