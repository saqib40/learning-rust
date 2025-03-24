enum Direction {
    North,
    South,
    East,
    West,
}

fn navigate(direction: Direction) {
    match direction {
        Direction::North => println!("Going North"),
        Direction::South => println!("Going South"),
        Direction::East => println!("Going East"),
        Direction::West => println!("Going West"),
    }
}

pub fn basic() {
    let dir = Direction::East;
    navigate(dir);
}