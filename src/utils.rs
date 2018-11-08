use SCREEN_WIDTH;
use SCREEN_HEIGHT;

use objects::Coordinate;
use objects::Size;

pub fn ensure_valid_coordinate(coordinate: Coordinate, size: &Size) -> Coordinate {
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
