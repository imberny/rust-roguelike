#![allow(unused_variables)]
#![allow(dead_code)]
use bevy_ecs::{prelude::*, schedule::ShouldRun};
use rltk::{GameState, Point, Rltk, RGB};

mod map;
use map::Map;

mod components;
use components::*;

mod systems;
use systems::*;

mod player;
use player::{handle_player_input, poll_input, PlayerInput};

mod rendering;
use rendering::render;

mod actor;
use actor::{action::process_move_actions, Actor};

mod constants;

mod types;

type PlayerPosition = Point;

#[derive(PartialEq, Copy, Clone)]
pub enum TurnBasedState {
    None,
    PlayerTurn,
    OpponentsTurn,
}

pub struct Game {
    is_waiting_for_input: bool,
    turn_based_state: TurnBasedState,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
pub enum SystemGroup {
    Actor,
    Action,
    Visual,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum MainStage {
    InputPoll,
    Actor,
    Action,
    EndOfTurn,
    Animate,
    Render,
}

pub struct ECS {
    pub world: World,
    pub schedule: Schedule,
}

impl GameState for ECS {
    fn tick(&mut self, ctx: &mut Rltk) {
        // update PlayerInput resource
        poll_input(&mut self.world, ctx);

        // run systems
        self.schedule.run(&mut self.world);

        // draw
        render(&mut self.world, ctx);
    }
}

fn add_monsters_to_rooms(gs: &mut ECS, map: &Map) {
    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let name: String;
        let glyph: rltk::FontCharType;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => {
                name = "Goblin".to_string();
                glyph = rltk::to_cp437('g');
            }
            _ => {
                name = "Orc".to_string();
                glyph = rltk::to_cp437('o')
            }
        }

        gs.world
            .spawn()
            .insert(Monster {})
            .insert(Actor::default())
            .insert(Name {
                name: format!("{} #{}", &name, i),
            })
            .insert(Position { x, y })
            .insert(Renderable {
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .insert(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            });
    }
}

fn create_player_at_pos(gs: &mut ECS, player_x: i32, player_y: i32) {
    gs.world
        .spawn()
        .insert(Player)
        .insert(Actor::default())
        .insert(Name {
            name: "Player".to_string(),
        })
        .insert(Position {
            x: player_x,
            y: player_y,
        })
        .insert(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .insert(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        });
}

fn build_map(gs: &mut ECS) {
    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    add_monsters_to_rooms(gs, &map);
    gs.world.insert_resource(map);

    create_player_at_pos(gs, player_x, player_y);
}

fn is_not_waiting_for_input(game: Res<Game>) -> ShouldRun {
    if game.is_waiting_for_input {
        ShouldRun::No
    } else {
        ShouldRun::Yes
    }
}

fn is_player_ready() -> ShouldRun {
    ShouldRun::Yes
}

fn end_player_turn(mut game: ResMut<Game>) {
    game.is_waiting_for_input = true;
}

fn init_game() -> ECS {
    let mut world = World::new();
    world.insert_resource::<Game>(Game {
        is_waiting_for_input: false,
        turn_based_state: TurnBasedState::PlayerTurn,
    });
    world.insert_resource::<PlayerInput>(PlayerInput::default());

    let mut input_poll = SystemStage::parallel();
    input_poll
        .set_run_criteria(is_not_waiting_for_input.system())
        .add_system(handle_player_input.system());

    let mut process_actors = SystemStage::parallel();
    process_actors
        .set_run_criteria(is_not_waiting_for_input.system())
        .add_system_set(
            SystemSet::new()
                .label(SystemGroup::Actor)
                .with_system(process_move_actions.system())
                .with_system(monster_ai.system()),
        );

    let mut process_actions = SystemStage::parallel();
    process_actions
        .set_run_criteria(is_not_waiting_for_input.system())
        .add_system_set(SystemSet::new().label(SystemGroup::Action));

    let mut end_of_turn = SystemStage::parallel();
    end_of_turn
        .set_run_criteria(is_not_waiting_for_input.system())
        .add_system_set(
            SystemSet::new()
                .label(SystemGroup::Visual)
                .with_system(update_viewsheds.system())
                .with_system(update_player_viewshed.system()),
        );

    let mut render_stage = SystemStage::parallel();
    render_stage
        .add_system(end_player_turn.system())
        .set_run_criteria(is_not_waiting_for_input.system());

    let mut schedule = Schedule::default();
    schedule
        .add_stage(MainStage::InputPoll, input_poll)
        .add_stage_after(MainStage::InputPoll, MainStage::Actor, process_actors)
        .add_stage_after(MainStage::Actor, MainStage::Action, process_actions)
        .add_stage_after(MainStage::Action, MainStage::EndOfTurn, end_of_turn)
        .add_stage_after(MainStage::EndOfTurn, MainStage::Render, render_stage);

    let mut gs = ECS { world, schedule };

    build_map(&mut gs);
    gs
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_dimensions(120, 75)
        .with_title("The Possession of Barbe Halle")
        .build()?;

    let gs = init_game();

    rltk::main_loop(context, gs)
}
