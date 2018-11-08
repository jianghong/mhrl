mod objects;

use objects::*;

extern crate tcod;
extern crate toml;
#[macro_use]
extern crate serde_derive;

use tcod::console::*;
use tcod::colors;


const SCREEN_WIDTH: i32 = 150;
const SCREEN_HEIGHT: i32 = 100;
const LIMIT_FPS: i32 = 60;

struct Tcod {
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

	while !tcod.root.window_closed() {
	    tcod.root.set_default_foreground(colors::WHITE);
	    tcod.root.clear();
	    draw_boss(&game, &mut tcod, Coordinate{x: SCREEN_WIDTH / 2, y: SCREEN_HEIGHT / 2});
	    tcod.root.flush();
	    tcod.root.wait_for_keypress(true);
	}
}

fn draw_boss(game: &Game, tcod: &mut Tcod, top_left_coordinates: Coordinate) {
	let boss_metadata = &game.objects_metadata.dragon;
	let valid_coordinate = ensure_valid_coordinate(top_left_coordinates, &boss_metadata.size);
	let dragon = Object::new("Dragon boss", valid_coordinate, boss_metadata.clone());


	for i in 0..boss_metadata.size.width {
		tcod.root.put_char(dragon.position.x + i, dragon.position.y, 'D', BackgroundFlag::None);
		for j in 0..boss_metadata.size.height {
			tcod.root.put_char(dragon.position.x + i, dragon.position.y + j, 'D', BackgroundFlag::None);
		}
	}
}

fn ensure_valid_coordinate(coordinate: Coordinate, size: &Size) -> Coordinate {
	let verified_x = if coordinate.x + size.width > SCREEN_WIDTH {
		SCREEN_WIDTH - size.width
	} else {
		coordinate.x
	};
	let verified_y = if coordinate.y + size.height > SCREEN_HEIGHT {
		SCREEN_HEIGHT - size.height
	} else {
		coordinate.y
	};
	Coordinate{x: verified_x, y: verified_y}
}
