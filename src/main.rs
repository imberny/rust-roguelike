#![allow(unused_variables)]
#![allow(dead_code)]
use rltk::{GameState, Point, Rltk, RGB};
use specs::prelude::*;

mod rect;
use rect::Rect;

mod map;
use map::Map;

mod components;
use components::*;

mod systems;
use systems::*;

mod player;
use player::*;

type PlayerPosition = Point;

#[derive(PartialEq, Copy, Clone)]
pub enum GameRunState {
    Paused,
    Running,
}

pub struct State {
    pub ecs: World,
    pub game_state: GameRunState,
}

impl State {
    fn run_systems(&mut self) {
        use rltk::console;

        console::log("Running systems...");

        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAISystem {};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    for (idx, tile) in map.tiles.iter().enumerate() {
        if !map.revealed[idx] {
            continue;
        }

        let (x, y) = map.idx_xy(idx);
        let mut fg;
        let glyph;
        match tile {
            map::TileType::Floor => {
                fg = RGB::from_f32(0.0, 0.5, 0.5);
                glyph = rltk::to_cp437('.');
            }
            map::TileType::Wall => {
                fg = RGB::from_f32(0., 1.0, 0.);
                glyph = rltk::to_cp437('#');
            }
        }
        if !map.visible[idx] {
            fg = fg.to_greyscale()
        }
        ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        match self.game_state {
            GameRunState::Running => {
                self.run_systems();

                self.game_state = GameRunState::Paused;
            }

            GameRunState::Paused => {
                self.game_state = player_input(self, ctx);
            }
        }

        ctx.cls();
        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

fn register_components(ecs: &mut World) {
    ecs.register::<Position>();
    ecs.register::<Renderable>();
    ecs.register::<Player>();
    ecs.register::<Monster>();
    ecs.register::<Name>();
    ecs.register::<Viewshed>();
}

fn add_monsters_to_rooms(gs: &mut State, map: &Map) {
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

        gs.ecs
            .create_entity()
            .with(Monster {})
            .with(Name {
                name: format!("{} #{}", &name, i),
            })
            .with(Position { x, y })
            .with(Renderable {
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .build();
    }
}

fn create_player_at_pos(gs: &mut State, player_x: i32, player_y: i32) {
    gs.ecs
        .create_entity()
        .with(Player {})
        .with(Name {
            name: "Player".to_string(),
        })
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();
}

fn build_map(gs: &mut State) {
    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    add_monsters_to_rooms(gs, &map);
    gs.ecs.insert(map);
    gs.ecs.insert(PlayerPosition::new(player_x, player_y));

    create_player_at_pos(gs, player_x, player_y);
}

fn init_game() -> State {
    let mut ecs = World::new();
    register_components(&mut ecs);
    let mut gs = State {
        ecs: ecs,
        game_state: GameRunState::Running,
    };

    build_map(&mut gs);
    gs
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let gs = init_game();

    rltk::main_loop(context, gs)
}
