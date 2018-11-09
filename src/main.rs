use std::sync::mpsc::Sender;
use std::time::{Instant};
use std::thread;
use std::sync::mpsc;

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
	turn: i32,
}

#[derive(Clone, Copy, Debug)]
enum DelayedAttackCallback {
	SlamStart,
	SlamEnd,
}

#[derive(Clone, Debug)]
struct DelayedAttack {
	coordinates: Vec<Coordinate>,
	starting_turn: i32,
	ends_in: i32,
	started: bool,
	start: DelayedAttackCallback,
	end: DelayedAttackCallback,
}

impl DelayedAttackCallback {
	fn callback(self, coordinates: Vec<Coordinate>, tcod: &mut Tcod) {
		use DelayedAttackCallback::*;
		let callback: fn(Vec<Coordinate>, &mut Tcod) = match self {
			SlamStart => slam_start,
			SlamEnd => slam_end,
		};
		callback(coordinates, tcod);		
	}
}

fn slam_start(coordinates: Vec<Coordinate>, tcod: &mut Tcod) {
	for coordinate in coordinates {
		tcod.root.set_char_background(coordinate.x, coordinate.y, colors::YELLOW, BackgroundFlag::Set);
	}
}

fn slam_end(coordinates: Vec<Coordinate>, tcod: &mut Tcod) {
	for coordinate in coordinates {
		tcod.root.set_char_background(coordinate.x, coordinate.y, colors::BLACK, BackgroundFlag::Set);
	}
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
	tcod.root.set_keyboard_repeat(1, 0);
	let mut game = Game {
		objects_metadata: get_objects_metadata(),
		turn: 1,
	};
	let mut player_pos = Coordinate{x: (SCREEN_WIDTH / 2) - 5, y: (SCREEN_HEIGHT / 2) - 5};
	let mut key: Key = Default::default();

	// handle turn clock
	let (tx, rx) = mpsc::channel();
	spawn_turn_clock(tx);

	let mut delayed_attacks: Vec<DelayedAttack> = vec![];
	let dragon_coords = Coordinate{x: SCREEN_WIDTH / 2, y: SCREEN_HEIGHT / 2};
	let mut slam = DelayedAttack{
		coordinates: vec![Coordinate{x: dragon_coords.x - 1, y: dragon_coords.y}, Coordinate{x: dragon_coords.x - 1, y: dragon_coords.y + 1}],
		starting_turn: game.turn,
		ends_in: 2,
		started: false,
		start: DelayedAttackCallback::SlamStart,
		end: DelayedAttackCallback::SlamEnd,	    	
	};
	slam.start.callback(slam.coordinates.clone(), &mut tcod);
	slam.started = true;
    delayed_attacks.push(slam);

	while !tcod.root.window_closed() {
		log_turn(&mut tcod, &game);
		match input::check_for_event(input::KEY_PRESS) {
			Some((_, Event::Key(k))) => key = k,
			_ => key = Default::default(),
		}

		// draw shit
	    tcod.root.set_default_foreground(colors::WHITE);
	    
	   	// check delayed attack ends
	   	let mut ended_attacks: Vec<usize> = vec![];
	    for (index, attack) in delayed_attacks.iter().enumerate() {
	    	if (game.turn - attack.starting_turn) >= attack.ends_in {
	    		attack.end.callback(attack.coordinates.clone(), &mut tcod);
	    		ended_attacks.push(index);
	    	}
	    }

	    for ended_attack in ended_attacks {
	    	delayed_attacks.remove(ended_attack);
	    }


	    draw_object(&game.objects_metadata.dragon, &mut tcod, dragon_coords);
	    draw_object(&game.objects_metadata.player, &mut tcod, player_pos);

	    tcod.root.flush();

	    tcod.root.put_char(player_pos.x, player_pos.y, ' ', BackgroundFlag::None);
        let exit = handle_keys(key, &mut tcod, &mut player_pos.x, &mut player_pos.y);
        if exit {
            break
        }

        let val = match rx.try_recv() {
			Ok(v) => v,
			Err(_) => 0,
        };
        game.turn += val;
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

fn log_turn(tcod: &mut Tcod, game: &Game) {
	let turns_text = format!("Turn: {}", game.turn);
	tcod.root.print_ex(1, 1, BackgroundFlag::None, TextAlignment::Left, turns_text);
}

fn spawn_turn_clock(tx: Sender<i32>) {
	thread::spawn(move || {
		let mut previous_instant = Instant::now();
		loop {
	        if previous_instant.elapsed().subsec_millis() == 999 {
	        	previous_instant = Instant::now();
            	tx.send(1).expect("Transmission failed");
	        }
		};
    });
}