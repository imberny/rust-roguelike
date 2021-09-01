#![allow(unused_variables)]
#![allow(dead_code)]
use bevy_ecs::{prelude::*, schedule::ShouldRun};
use constants::facings::SOUTH;
use rltk::{GameState, Point, Rltk, VirtualKeyCode, INPUT, RGB};

mod map;
use map::Map;

mod components;
use components::*;

mod systems;
use systems::*;

mod player;
use player::handle_input;

mod rendering;
use rendering::render;

mod actor;
use actor::Actor;

mod constants;

mod types;

type PlayerPosition = Point;

#[derive(PartialEq, Copy, Clone)]
pub enum TurnBasedState {
    PlayerTurn,
    OpponentsTurn,
}

pub struct Game {
    turn_based_state: TurnBasedState,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
pub enum SystemGroup {
    Player,
    Opponent,
    Visual,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum MainStage {
    InputPoll,
    PlayerTurn,
    OpponentsTurn,
    EndOfTurn,
    Render,
}

pub struct ECS {
    pub world: World,
    pub schedule: Schedule,
}

#[derive(Default)]
pub struct PlayerInput {
    key: Option<VirtualKeyCode>,
    pub cursor_pos: (i32, i32),
    pub left_click: bool,
    pub is_strafing: bool,
    pub skew_move: bool,
    pub alt: bool,
}

const LEFT_MOUSE_BUTTON: usize = 0;

pub fn poll_input(world: &mut World, ctx: &Rltk) {
    let mut input = world.get_resource_mut::<PlayerInput>().unwrap();
    input.key = ctx.key;
    input.cursor_pos = ctx.mouse_pos;

    let rltk_input = INPUT.lock();
    input.left_click = rltk_input.is_mouse_button_pressed(LEFT_MOUSE_BUTTON);
    input.skew_move = rltk_input.is_key_pressed(VirtualKeyCode::LControl)
        || rltk_input.is_key_pressed(VirtualKeyCode::RControl);
    input.is_strafing = rltk_input.is_key_pressed(VirtualKeyCode::LShift)
        || rltk_input.is_key_pressed(VirtualKeyCode::RShift);
    input.alt = rltk_input.is_key_pressed(VirtualKeyCode::LAlt)
        || rltk_input.is_key_pressed(VirtualKeyCode::RAlt);
    input.skew_move = rltk_input.is_key_pressed(VirtualKeyCode::LControl);
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
            .insert(Actor { facing: SOUTH })
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
        .insert(Actor { facing: SOUTH })
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

fn is_player_turn(game: Res<Game>) -> ShouldRun {
    if game.turn_based_state == TurnBasedState::PlayerTurn {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
    // match game.turn_based_state {
    //     TurnBasedState::PlayerTurn => ShouldRun::Yes,
    //     _ => ShouldRun::No,
    // }
}

fn is_opponents_turn(game: Res<Game>) -> ShouldRun {
    match game.turn_based_state {
        TurnBasedState::OpponentsTurn => ShouldRun::Yes,
        _ => ShouldRun::No,
    }
}

fn end_opponents_turn(mut game: ResMut<Game>) {
    game.turn_based_state = TurnBasedState::PlayerTurn;
}

fn init_game() -> ECS {
    let mut world = World::new();
    world.insert_resource::<Game>(Game {
        turn_based_state: TurnBasedState::PlayerTurn,
    });
    world.insert_resource::<PlayerInput>(PlayerInput::default());
    // player input stage
    // runcondition: gamerunstate == player_turn

    // update stage
    //  run player systems
    //  run opponent systems
    //  handle game over

    // render stage
    //  update all animations
    //  draw (runcondition: something changed)

    let mut player_turn = SystemStage::parallel();
    player_turn
        .set_run_criteria(is_player_turn.system())
        .add_system_set(
            SystemSet::new()
                .label(SystemGroup::Player)
                // .with_run_criteria(is_player_turn.system())
                .with_system(handle_input.system()),
        );
    let mut opponents_turn = SystemStage::parallel();
    opponents_turn
        .set_run_criteria(is_opponents_turn.system())
        .add_system_set(
            SystemSet::new()
                .label(SystemGroup::Opponent)
                // .with_run_criteria(is_opponents_turn.system())
                .with_system(monster_ai.system()),
        );

    let mut end_of_turn = SystemStage::parallel();
    end_of_turn
        .add_system_set(
            SystemSet::new()
                .label(SystemGroup::Visual)
                .with_system(update_viewsheds.system())
                .with_system(update_player_viewshed.system()),
        )
        .add_system(
            end_opponents_turn
                .system()
                .with_run_criteria(is_opponents_turn.system()),
        );

    let render_stage = SystemStage::parallel();

    let mut schedule = Schedule::default();
    schedule
        .add_stage(MainStage::PlayerTurn, player_turn)
        .add_stage_after(
            MainStage::PlayerTurn,
            MainStage::OpponentsTurn,
            opponents_turn,
        )
        .add_stage_after(MainStage::OpponentsTurn, MainStage::EndOfTurn, end_of_turn)
        .add_stage_after(MainStage::EndOfTurn, MainStage::Render, render_stage);

    let mut gs = ECS { world, schedule };

    build_map(&mut gs);
    gs
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("The Possession of Barbe Halle")
        .build()?;

    let gs = init_game();

    rltk::main_loop(context, gs)
}
