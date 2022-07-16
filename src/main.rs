use macroquad::prelude::*;


struct Level {
    start: IVec2,
    size:  IVec2,
    tiles: Vec<char>,
    goals: Vec<IVec2>,
}

impl Level {
    pub fn parse(level: &str) -> Level {
        let lines = level.split("\n").map(str::trim).filter(|line| line.len() > 0);

        let mut start = None;
        let mut width  = 0;
        let mut height = 0;
        let mut tiles = vec![];
        let mut goals = vec![];

        for line in lines {
            assert!(line.len() > 2);
            assert!(line.as_bytes()[0]              == '|' as u8
                &&  line.as_bytes()[line.len() - 1] == '|' as u8);
            let line = &line[1..line.len() - 1];

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

                if "123456".contains(tile) {
                    let count = tile as u8 - '1' as u8 + 1;
                    draw_eyes(count, pos, tile_size, BLACK);
                }
            }
        }

    }
}


#[derive(Clone, Copy, PartialEq)]
#[repr(usize)]
enum Side {
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
}


struct Dice {
    pos: IVec2,
    sides: [u8; 6],
}

impl Dice {
    pub fn new(pos: IVec2) -> Dice {
        Dice { pos, sides: [1, 6, 4, 3, 5, 2] }
    }

    pub fn get(&self, side: Side) -> u8 {
        self.sides[side as usize]
    }

    pub fn render(&self, origin: Vec2, tile_size: f32) {
        let pos = origin + self.pos.as_f32()*tile_size;
        draw_rectangle(pos.x, pos.y, tile_size, tile_size, WHITE);
        draw_eyes(self.get(Side::Sky), pos, tile_size, BLACK);

        let eye_color = Color::new(0.0, 0.0, 0.0, 0.25);
        draw_eyes(self.get(Side::Left), pos + Vec2::new(-tile_size, 0.0), tile_size, eye_color);
        draw_eyes(self.get(Side::Right), pos + Vec2::new(tile_size, 0.0), tile_size, eye_color);
        draw_eyes(self.get(Side::Down), pos + Vec2::new(0.0, tile_size), tile_size, eye_color);
        draw_eyes(self.get(Side::Up), pos + Vec2::new(0.0, -tile_size), tile_size, eye_color);
    }

    pub fn move_thyself(&mut self, side: Side) {
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

        self.sides = sides;
        self.pos  += side.unit();
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


#[macroquad::main("gmtk-2022")]
async fn main() {

    let level = Level::parse(r#"
        |.....|
        |.s.6.|
        |.....|
    "#);

    let mut dice = Dice::new(level.start);

    let tile_size = 50.0;

    loop {

        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
            dice.move_thyself(Side::Left);
        }
        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
            dice.move_thyself(Side::Right);
        }
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            dice.move_thyself(Side::Down);
        }
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            dice.move_thyself(Side::Up);
        }

        clear_background(GRAY);

        let board_size = level.size.as_f32() * tile_size;
        let screen_size = Vec2::new(screen_width(), screen_height());
        let origin = (screen_size/2.0 - board_size/2.0).floor();

        level.render(origin, tile_size);
        dice.render(origin, tile_size);

        next_frame().await;
    }
}
