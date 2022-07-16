use macroquad::prelude::*;


pub struct Level {
    start: IVec2,
    size:  IVec2,
    tiles: Vec<char>,
    goals: Vec<IVec2>,
}

impl Level {
    pub fn parse(level: &[&str]) -> Level {
        let mut start = None;
        let mut width  = 0;
        let mut height = 0;
        let mut tiles = vec![];
        let mut goals = vec![];

        for line in level {
            if height == 0 {
                width = line.len();
            }
            assert!(line.len() == width);

            let y = height;
            height += 1;

            for (x, mut tile) in line.chars().enumerate() {
                assert!(" .123456s".contains(tile));
                let pos = IVec2::new(x as i32, y);

                if tile == 's' {
                    assert!(start.is_none());
                    start = Some(pos);
                    tile = '.';
                }

                if "123456".contains(tile) {
                    goals.push(pos);
                }

                tiles.push(tile);
            }
        }

        let start = start.unwrap();
        let size  = IVec2::new(width as i32, height);
        Level { start, size, tiles, goals }
    }

    pub fn get(&self, x: i32, y: i32) -> char {
        if x >= 0 && x < self.size.x && y >= 0 && y < self.size.y {
            self.tiles[(y*self.size.x + x) as usize]
        }
        else {
            ' '
        }
    }

    pub fn to_goal(tile: char) -> Option<u8> {
        if "123456".contains(tile) {
            return Some((tile as u8) - ('1' as u8) + 1);
        }
        None
    }

    pub fn detect_win(&self, dice: &Dice) -> bool {
        self.goals.iter().all(|goal| dice.on_tail(*goal) || *goal == dice.pos)
    }

    pub fn render(&self, origin: Vec2, tile_size: f32) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let pos = origin + Vec2::new(x as f32, y as f32)*tile_size;

                let tile = self.get(x, y);
                if tile == ' ' {
                    continue;
                }

                draw_rectangle(
                    pos.x, pos.y,
                    tile_size, tile_size,
                    DARKGRAY);

                if let Some(count) = Self::to_goal(tile) {
                    draw_eyes(count, pos, tile_size, BLACK);
                }
            }
        }

    }
}


#[derive(Clone, Copy, PartialEq)]
#[repr(usize)]
pub enum Side {
    Floor = 0,
    Sky   = 1,
    Left  = 2,
    Right = 3,
    Down  = 4,
    Up    = 5,
}

impl Side {
    pub fn unit(self) -> IVec2 {
        match self {
            Side::Left  => IVec2::new(-1, 0),
            Side::Right => IVec2::new( 1, 0),
            Side::Down  => IVec2::new(0,  1),
            Side::Up    => IVec2::new(0, -1),
            _ => unreachable!()
        }
    }

    pub fn from_unit(unit: IVec2) -> Side {
        match (unit.x, unit.y) {
            (-1, 0) => Side::Left,
            ( 1, 0) => Side::Right,
            (0,  1) => Side::Down,
            (0, -1) => Side::Up,
            _ => unreachable!()
        }
    }
}


pub struct Dice {
    pos: IVec2,
    sides: [u8; 6],
    tail: Vec<(IVec2, u8)>,
}

impl Dice {
    pub fn new(pos: IVec2) -> Dice {
        Dice { pos, sides: [1, 6, 4, 3, 5, 2], tail: vec![] }
    }

    pub fn get(&self, side: Side) -> u8 {
        self.sides[side as usize]
    }

    pub fn on_tail(&self, target: IVec2) -> bool {
        for (pos, _) in self.tail.iter() {
            if *pos == target {
                return true;
            }
        }
        false
    }

    pub fn render(&self, origin: Vec2, tile_size: f32) {
        let pos = origin + self.pos.as_f32()*tile_size;
        draw_rectangle(pos.x, pos.y, tile_size, tile_size, WHITE);
        draw_eyes(self.get(Side::Sky), pos, tile_size, BLACK);

        for (pos, count) in self.tail.iter() {
            draw_eyes(*count, origin + pos.as_f32()*tile_size, tile_size, Color::new(0.0, 0.0, 0.0, 0.5));
        }

        let eye_color = Color::new(0.0, 0.0, 0.0, 0.25);
        draw_eyes(self.get(Side::Left), pos + Vec2::new(-tile_size, 0.0), tile_size, eye_color);
        draw_eyes(self.get(Side::Right), pos + Vec2::new(tile_size, 0.0), tile_size, eye_color);
        draw_eyes(self.get(Side::Down), pos + Vec2::new(0.0, tile_size), tile_size, eye_color);
        draw_eyes(self.get(Side::Up), pos + Vec2::new(0.0, -tile_size), tile_size, eye_color);
    }

    pub fn rotate(&self, side: Side) -> [u8; 6] {
        let floor = Side::Floor as usize;
        let sky   = Side::Sky as usize;
        let left  = Side::Left as usize;
        let right = Side::Right as usize;
        let down  = Side::Down as usize;
        let up    = Side::Up as usize;

        let rotation = match side {
            Side::Left  => [left, floor, right, sky],
            Side::Right => [right, floor, left, sky],
            Side::Down  => [down, floor, up, sky],
            Side::Up    => [up, floor, down, sky],
            _ => unreachable!()
        };

        let mut sides = self.sides;
        for i in 0..rotation.len() {
            let from = rotation[i];
            let to   = rotation[(i + 1) % rotation.len()];
            sides[to] = self.sides[from];
        }

        sides
    }

    pub fn move_thyself(&mut self, side: Side) {
        self.tail.push((self.pos, self.get(Side::Floor)));
        self.sides = self.rotate(side);
        self.pos  += side.unit();
    }

    pub fn undo(&mut self) {
        let (pos, _) = self.tail.pop().unwrap();
        self.sides = self.rotate(Side::from_unit(pos - self.pos));
        self.pos = pos;
    }
}


pub fn draw_eyes(count: u8, pos: Vec2, tile_size: f32, color: Color) {
    assert!(count >= 1 && count <= 6);

    let padding = tile_size/4.0;
    let delta   = (tile_size - 2.0*padding) / 2.0;

    let positions = [
        &[Vec2::new(1.0, 1.0)][..],
        &[Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0)],
        &[Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(2.0, 2.0)],
        &[Vec2::new(0.0, 0.0), Vec2::new(0.0, 2.0), Vec2::new(2.0, 2.0), Vec2::new(2.0, 0.0)],
        &[Vec2::new(0.0, 0.0), Vec2::new(0.0, 2.0), Vec2::new(2.0, 2.0), Vec2::new(2.0, 0.0), Vec2::new(1.0, 1.0)],
        &[Vec2::new(0.0, 0.0), Vec2::new(0.0, 2.0), Vec2::new(2.0, 2.0), Vec2::new(2.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(2.0, 1.0)],
    ];

    for eye in positions[count as usize - 1] {
        let pos = pos + Vec2::splat(padding) + *eye*Vec2::splat(delta);
        draw_circle(pos.x, pos.y, tile_size/10.0, color)
    }
}

pub fn try_move(dice: &mut Dice, level: &Level, side: Side) -> bool {
    let target = dice.pos + side.unit();

    if let Some((pos, _)) = dice.tail.last() {
        if *pos == target {
            dice.undo();
            return false;
        }
    }

    if dice.on_tail(target) {
        return false;
    }

    let tile = level.get(target.x, target.y);

    if tile == ' ' {
        return false;
    }

    if let Some(count) = Level::to_goal(tile) {
        if count != dice.get(side) {
            return false;
        }
    }

    dice.move_thyself(side);
    true
}


#[macroquad::main("gmtk-2022")]
async fn main() {

    fn parse_levels(levels: &str) -> Vec<Level> {
        levels.split("\n\n")
        .map(|lines|
            Level::parse(
                &lines.split("\n")
                .filter(|line| line.len() > 0)
                .collect::<Vec<_>>())
        ).collect()
    }

    fn load(levels: &str) -> (Vec<Level>, usize, Dice) {
        let levels = parse_levels(levels);
        let level_index = 0;
        let dice = Dice::new(levels[0].start);
        (levels, level_index, dice)
    }

    fn hot_load() -> (Vec<Level>, usize, Dice) {
        load(&String::from_utf8(std::fs::read("src/levels.txt").unwrap()).unwrap())
    }

    fn set_level(index: usize, levels: &[Level], level_index: &mut usize, dice: &mut Dice) {
        *level_index = index;
        *dice  = Dice::new(levels[*level_index].start);
    }

    fn next_level(levels: &[Level], level_index: &mut usize, dice: &mut Dice) {
        if *level_index < levels.len() - 1 {
            set_level(*level_index + 1, levels, level_index, dice);
        }
    }

    fn prev_level(levels: &[Level], level_index: &mut usize, dice: &mut Dice) {
        if *level_index > 0 {
            set_level(*level_index - 1, levels, level_index, dice);
        }
    }

    let (mut levels, mut level_index, mut dice) = hot_load();

    let tile_size = 50.0;

    loop {

        let level = &levels[level_index];

        let mut moved = false;
        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
            moved |= try_move(&mut dice, &level, Side::Left);
        }
        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
            moved |= try_move(&mut dice, &level, Side::Right);
        }
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            moved |= try_move(&mut dice, &level, Side::Down);
        }
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            moved |= try_move(&mut dice, &level, Side::Up);
        }

        if moved && level.detect_win(&dice) {
            println!("win!");
            next_level(&levels, &mut level_index, &mut dice);
        }

        if is_key_pressed(KeyCode::F1) {
            prev_level(&levels, &mut level_index, &mut dice);
        }
        if is_key_pressed(KeyCode::F2) {
            next_level(&levels, &mut level_index, &mut dice);
        }
        if is_key_pressed(KeyCode::F5) {
            (levels, level_index, dice) = hot_load();
        }


        let level = &levels[level_index];

        clear_background(GRAY);

        let board_size = level.size.as_f32() * tile_size;
        let screen_size = Vec2::new(screen_width(), screen_height());
        let origin = (screen_size/2.0 - board_size/2.0).floor();

        level.render(origin, tile_size);
        dice.render(origin, tile_size);

        next_frame().await;
    }
}
