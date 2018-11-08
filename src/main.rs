mod objects;

use objects::*;

mod utils;
use utils::ensure_valid_coordinate;

mod input_controller;
use input_controller::handle_keys;

extern crate tcod;
use tcod::console::*;
use tcod::colors;
use tcod::input::{self, Event, Key};


extern crate toml;

#[macro_use]
extern crate serde_derive;

const SCREEN_WIDTH: i32 = 150;
const SCREEN_HEIGHT: i32 = 100;
const LIMIT_FPS: i32 = 20;

pub struct Tcod {
	root: Root,
}

struct Game {
	objects_metadata: ObjectsMetadata,
}

fn main() {
	let root = Root::initializer()
	    .font("terminal8x8_gs_ro.png", FontLayout::AsciiInRow)
	    .font_type(FontType::Greyscale)
	    .size(SCREEN_WIDTH, SCREEN_HEIGHT)
	    .title("MHRL")
	    .init();
	tcod::system::set_fps(LIMIT_FPS);
	let mut tcod = Tcod {
		root: root,
	};
	let game = Game {
		objects_metadata: get_objects_metadata(),
	};
	let mut player_pos = Coordinate{x: (SCREEN_WIDTH / 2) - 5, y: (SCREEN_HEIGHT / 2) - 5};
	let mut key: Key = Default::default();

	while !tcod.root.window_closed() {
		match input::check_for_event(input::KEY_PRESS) {
			Some((_, Event::Key(k))) => key = k,
			_ => key = Default::default(),
		}

	    tcod.root.set_default_foreground(colors::WHITE);
	    draw_object(&game.objects_metadata.dragon, &mut tcod, Coordinate{x: SCREEN_WIDTH / 2, y: SCREEN_HEIGHT / 2});
	    draw_object(&game.objects_metadata.player, &mut tcod, player_pos);
	    tcod.root.flush();

	    tcod.root.put_char(player_pos.x, player_pos.y, ' ', BackgroundFlag::None);
        let exit = handle_keys(key, &mut tcod, &mut player_pos.x, &mut player_pos.y);
        if exit {
            break
        }
	}
}

fn draw_object(object_metadata: &ObjectMetadata, tcod: &mut Tcod, top_left_coordinates: Coordinate) -> Object {
	let valid_coordinate = ensure_valid_coordinate(top_left_coordinates, &object_metadata.size);
	let object = Object::new(&object_metadata.name, valid_coordinate, object_metadata.clone());


	for i in 0..object_metadata.size.width {
		tcod.root.put_char(object.position.x + i, object.position.y, object_metadata.character, BackgroundFlag::None);
		for j in 0..object_metadata.size.height {
			tcod.root.put_char(object.position.x + i, object.position.y + j, object_metadata.character, BackgroundFlag::None);
		}
	}

	object
}