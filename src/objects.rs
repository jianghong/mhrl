use std::fs;
use toml::{from_str};

pub struct Coordinate {
	pub x: i32,
	pub y: i32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Size {
	pub width: i32,
	pub height: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ObjectMetadata {
	pub name: String,
	pub size: Size
}

#[derive(Deserialize, Debug)]
pub struct ObjectsMetadata {
	pub dragon: ObjectMetadata,
	player: ObjectMetadata,
}

pub struct Object {
	pub name: String,
	pub position: Coordinate,
	pub metadata: ObjectMetadata,
}

impl Object {
	pub fn new(name: &str, position: Coordinate, metadata: ObjectMetadata) -> Self {
		Object {
			name: name.into(),
			position: position,
			metadata: metadata,
		}
	}
}

pub fn get_objects_metadata() -> ObjectsMetadata {
	let objects_string = fs::read_to_string("objects_metadata.toml").expect("Failed to read metadata file");
	let objects: ObjectsMetadata = from_str(&objects_string).expect("Failed to deserialize metadata to struct");
	objects	
}