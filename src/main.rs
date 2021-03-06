use lazy_static::lazy_static;
use macroquad::prelude::*;
use macroquad::audio::*;


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

    pub fn render(&self, origin: Vec2, tile_size: Vec2, _t: f32) {
        for y in 0..self.size.y + 1 {
            for x in 0..self.size.x {
                let pos = origin + Vec2::new(x as f32, y as f32)*tile_size;

                let tile = self.get(x, y);
                if tile == ' ' {
                    if self.get(x, y-1) != ' ' {
                        draw_texture_ex(*TEX_GRASS_FRONT, pos.x, pos.y, WHITE, DrawTextureParams {
                            dest_size: Some(tile_size),
                            .. Default::default()
                        });
                    }

                    continue;
                }

                draw_texture_ex(*TEX_GRASS_BASE, pos.x, pos.y, WHITE, DrawTextureParams {
                    dest_size: Some(tile_size),
                    .. Default::default()
                });

                if let Some(count) = Self::to_goal(tile) {
                    draw_eyes(count, pos, tile_size, Color::from_rgba(23, 22, 38, 128));

                    let s = 0.875 + ((2.5 * get_time()).sin().abs() as f32)*0.125;
                    let c = Color::from_rgba(103, 175, 65, 200).to_vec();
                    draw_goal(pos, tile_size, Color::from_vec(s*c));
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
    prev_eyes: u8,
    prev_pos: IVec2,
}

impl Dice {
    pub fn new(pos: IVec2) -> Dice {
        Dice {
            pos,
            sides: [1, 6, 4, 3, 5, 2],
            tail: vec![],
            prev_eyes: 0,
            prev_pos: pos,
        }
    }

    pub fn get(&self, side: Side) -> u8 {
        self.sides[side as usize]
    }

    pub fn eyes(&self) -> u8 {
        self.get(Side::Sky)
    }

    pub fn on_tail(&self, target: IVec2) -> bool {
        for (pos, _) in self.tail.iter() {
            if *pos == target {
                return true;
            }
        }
        false
    }

    pub fn render(&self, origin: Vec2, tile_size: Vec2, level: &Level, t: f32) {
        let eye_color = Color::from_rgba(23, 22, 38, 255);

        let mut pos = origin + self.pos.as_f32()*tile_size;

        // tail.
        let mut from = Side::Sky;
        for i in 0..self.tail.len() {
            let (pos, count) = self.tail[i];

            let next = self.tail.get(i + 1).map(|(pos, _)| *pos).unwrap_or(self.pos);
            let to = Side::from_unit(next - pos);

            let mut mask = [true; 6];
            mask[from as usize] = false;
            mask[to   as usize] = false;
            from = Side::from_unit(pos - next);

            let draw_pos = origin + pos.as_f32()*tile_size;

            // eyes
            let mut c = eye_color; c.a = 0.5;
            if let Some(_) = Level::to_goal(level.get(pos.x, pos.y)) {
                c = Color::from_rgba(103, 175, 65, 200);
            }
            draw_eyes(count, draw_pos, tile_size, c);

            // border
            let s = 0.875 + ((2.5 * get_time()).sin().abs() as f32)*0.125;
            let c = Color::from_rgba(120, 170, 255, 127).to_vec();
            draw_border(draw_pos, tile_size, mask, Color::from_vec(s*c));
        }

        // roll anim.
        let (mut curr_pos, mut curr_size) = (pos, tile_size);
        if t < 1.0 && self.prev_eyes != 0 {
            let (prev_pos, prev_size);

            let unit = (self.pos - self.prev_pos).as_f32() * tile_size;

            pos -= (1.0 - t)*unit;

            // we do not talk about how long it took me to figure this out...
            if unit.x + unit.y < 0.0 {
                prev_pos  = pos;
                curr_pos  = pos - (1.0 - t)*unit;
            }
            else {
                prev_pos  = pos + t*unit;
                curr_pos  = pos;
            }
            prev_size = tile_size - t*unit.abs();
            curr_size = tile_size - (1.0 - t)*unit.abs();

            draw_dice(prev_pos, prev_size, self.prev_eyes, eye_color);
        }

        draw_dice(curr_pos, curr_size, self.eyes(), eye_color);
    }

    pub fn rotate(&self, side: Side) -> [u8; 6] {
        use Side::*;
        let rotation = match side {
            Side::Left  => [Left, Floor, Right, Sky],
            Side::Right => [Right, Floor, Left, Sky],
            Side::Down  => [Down, Floor, Up, Sky],
            Side::Up    => [Up, Floor, Down, Sky],
            _ => unreachable!()
        };

        let mut sides = self.sides;
        for i in 0..rotation.len() {
            let from = rotation[i] as usize;
            let to   = rotation[(i + 1) % rotation.len()] as usize;
            sides[to] = self.sides[from];
        }

        sides
    }

    pub fn move_thyself(&mut self, side: Side) {
        self.prev_pos = self.pos;
        self.prev_eyes = self.eyes();

        self.tail.push((self.pos, self.get(Side::Floor)));
        self.sides = self.rotate(side);
        self.pos  += side.unit();
    }

    pub fn undo(&mut self) {
        self.prev_pos = self.pos;
        self.prev_eyes = self.eyes();

        let (pos, _) = self.tail.pop().unwrap();
        self.sides = self.rotate(Side::from_unit(pos - self.pos));
        self.pos = pos;
    }
}


pub fn draw_eyes(count: u8, pos: Vec2, size: Vec2, color: Color) {
    assert!(count >= 1 && count <= 6);
    draw_texture_ex(TEX_EYES[(count - 1) as usize], pos.x, pos.y, color, DrawTextureParams {
        dest_size: Some(size),
        .. Default::default()
    });
}

pub fn draw_dice(pos: Vec2, size: Vec2, eye_count: u8, eye_color: Color) {
    draw_texture_ex(*TEX_DICE, pos.x, pos.y, WHITE, DrawTextureParams {
        dest_size: Some(size),
        .. Default::default()
    });

    draw_eyes(eye_count, pos, size, eye_color);
}

pub fn draw_moves(level: &Level, dice: &Dice, origin: Vec2, tile_size: Vec2) {
    for side in [Side::Left, Side::Right, Side::Down, Side::Up] {
        let target = dice.pos + side.unit();

        if dice.on_tail(target) {
            continue;
        }

        let draw_pos = origin + target.as_f32()*tile_size;

        let tile = level.get(target.x, target.y);
        if tile == '.' {
            draw_eyes(dice.get(side), draw_pos, tile_size, Color::new(1.0, 1.0, 1.0, 0.25));
        }
        if let Some(count) = Level::to_goal(tile) {
            if count == dice.get(side) {
                draw_eyes(dice.get(side), draw_pos, tile_size, Color::from_rgba(103, 175, 65, 200));
            }
            else {
                draw_eyes(dice.get(side), draw_pos, tile_size, Color::from_rgba(216, 59, 39, 200));
            }
        }
    }
}

pub fn draw_border(pos: Vec2, size: Vec2, mask: [bool; 6], color: Color) {
    let index =
          (!mask[Side::Left  as usize] as usize) << 0
        | (!mask[Side::Up    as usize] as usize) << 1
        | (!mask[Side::Right as usize] as usize) << 2
        | (!mask[Side::Down  as usize] as usize) << 3;

    draw_texture_ex(TEX_BORDER[index], pos.x, pos.y, color, DrawTextureParams {
        dest_size: Some(size),
        .. Default::default()
    });
}

pub fn draw_background(origin: Vec2, tile_size: Vec2) {
    let o = origin.as_i32();
    let t = tile_size.as_i32();

    let offset = IVec2::new(o.x % t.x, o.y % t.y);
    let size = IVec2::new(screen_width() as i32, screen_height() as i32);
    let start = offset - t;

    let mut y = start.y;
    while y < size.y {

        let mut x = start.x;
        while x < size.x {
            draw_texture_ex(*TEX_WATER, x as f32, y as f32, WHITE, DrawTextureParams {
                dest_size: Some(tile_size),
                .. Default::default()
            });

            x += t.x;
        }

        y += t.y;
    }
}

pub fn draw_goal(pos: Vec2, size: Vec2, color: Color) {
    draw_border(pos, size, [true; 6], color)
}

pub fn play_step() {
    let i = rand::rand() as usize % SND_STEPS.len();
    play_sound_once(SND_STEPS[i]);
}

pub fn play_goal() {
    let i = rand::rand() as usize % SND_GOAL.len();
    play_sound_once(SND_GOAL[i]);
}

pub fn try_move(dice: &mut Dice, level: &Level, side: Side) -> bool {
    let target = dice.pos + side.unit();

    if let Some((pos, _)) = dice.tail.last() {
        if *pos == target {
            dice.undo();
            return true;
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
        play_goal();
    }

    dice.move_thyself(side);
    true
}



// ANIMATION

struct Anim {
    start: f64,
    duration: f64,
}

impl Anim {
    pub fn new(start: f64, duration: f64) -> Anim {
        Anim { start, duration }
    }

    pub fn t(&self) -> f32 {
        ((get_time() - self.start).min(self.duration) / self.duration) as f32
    }
}


// TEXTURES

pub fn load_texture(bytes: &[u8]) -> Texture2D {
    let t = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
    t.set_filter(FilterMode::Nearest);
    t
}

lazy_static!(
    static ref TEX_DICE: Texture2D = load_texture(include_bytes!("texture/dice.png"));

    static ref TEX_EYES: [Texture2D; 6] = [
        load_texture(include_bytes!("texture/eyes-1.png")),
        load_texture(include_bytes!("texture/eyes-2.png")),
        load_texture(include_bytes!("texture/eyes-3.png")),
        load_texture(include_bytes!("texture/eyes-4.png")),
        load_texture(include_bytes!("texture/eyes-5.png")),
        load_texture(include_bytes!("texture/eyes-6.png")),
    ];

    static ref TEX_BORDER: [Texture2D; 16] = [
        load_texture(include_bytes!("texture/border-0.png")),
        load_texture(include_bytes!("texture/border-1.png")),
        load_texture(include_bytes!("texture/border-2.png")),
        load_texture(include_bytes!("texture/border-3.png")),
        load_texture(include_bytes!("texture/border-4.png")),
        load_texture(include_bytes!("texture/border-5.png")),
        load_texture(include_bytes!("texture/border-6.png")),
        load_texture(include_bytes!("texture/border-7.png")),
        load_texture(include_bytes!("texture/border-8.png")),
        load_texture(include_bytes!("texture/border-9.png")),
        load_texture(include_bytes!("texture/border-10.png")),
        load_texture(include_bytes!("texture/border-11.png")),
        load_texture(include_bytes!("texture/border-12.png")),
        load_texture(include_bytes!("texture/border-13.png")),
        load_texture(include_bytes!("texture/border-14.png")),
        load_texture(include_bytes!("texture/border-15.png")),
    ];

    static ref TEX_GRASS_BASE: Texture2D = load_texture(include_bytes!("texture/grass-base.png"));
    static ref TEX_GRASS_FRONT: Texture2D = load_texture(include_bytes!("texture/grass-front.png"));

    static ref TEX_WATER: Texture2D = load_texture(include_bytes!("texture/water.png"));

    static ref TEX_WELCOME: Texture2D = load_texture(include_bytes!("texture/menu.png"));
    static ref TEX_DONE: Texture2D = load_texture(include_bytes!("texture/thanks.png"));
);


// SOUNDS

pub async fn load_sound(bytes: &[u8]) -> Sound {
    macroquad::audio::load_sound_from_bytes(bytes).await.unwrap()
}

static mut _SND_STEPS: Option<[Sound; 4]> = None;
static mut _SND_GOAL: Option<[Sound; 2]> = None;

lazy_static!(
    static ref SND_STEPS: [Sound; 4] = unsafe { _SND_STEPS.unwrap() };
    static ref SND_GOAL: [Sound; 2] = unsafe { _SND_GOAL.unwrap() };
);


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
        #[cfg(debug_assertions)] {
            load(&String::from_utf8(std::fs::read("src/levels.txt").unwrap()).unwrap())
        }
        #[cfg(not(debug_assertions))] {
            load(std::str::from_utf8(include_bytes!("levels.txt")).unwrap())
        }
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



    // load sounds.
    unsafe {
        _SND_STEPS = Some([
            load_sound(include_bytes!("sound/step-0.wav")).await,
            load_sound(include_bytes!("sound/step-1.wav")).await,
            load_sound(include_bytes!("sound/step-2.wav")).await,
            load_sound(include_bytes!("sound/step-3.wav")).await,
        ]);
        _SND_GOAL = Some([
            load_sound(include_bytes!("sound/goal-0.wav")).await,
            load_sound(include_bytes!("sound/goal-1.wav")).await,
        ]);
    }


    #[derive(Clone, Copy, PartialEq)]
    enum GameState {
        Welcome,
        Ready,
        Moving,
        Done,
    }


    let (mut levels, mut level_index, mut dice) = hot_load();

    let mut game_state = GameState::Welcome;
    let mut move_anim = Anim::new(-100.0, 0.125);

    loop {
        let now = get_time();

        let level = &levels[level_index];

        if game_state == GameState::Ready {
            let mut moved = false;
            if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
                moved |= try_move(&mut dice, &level, Side::Left);
            }
            else if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
                moved |= try_move(&mut dice, &level, Side::Right);
            }
            else if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                moved |= try_move(&mut dice, &level, Side::Down);
            }
            else if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                moved |= try_move(&mut dice, &level, Side::Up);
            }
            else if is_key_pressed(KeyCode::Z) && dice.tail.len() > 0 {
                dice.undo();
                moved = true;
            }


            if moved {
                play_step();

                if level.detect_win(&dice) {
                    if level_index + 1 < levels.len() {
                        next_level(&levels, &mut level_index, &mut dice);
                    }
                    else {
                        game_state = GameState::Done;
                    }
                }
                else {
                    game_state = GameState::Moving;
                    move_anim.start = now;
                }
            }

            if is_key_pressed(KeyCode::R) && dice.tail.len() > 0 {
                set_level(level_index, &levels, &mut level_index, &mut dice);
                play_step();
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
        }
        else if game_state == GameState::Moving {
            if move_anim.t() == 1.0 {
                game_state = GameState::Ready;
            }
        }
        else if game_state == GameState::Welcome || game_state == GameState::Done {
            if is_key_pressed(KeyCode::Enter) {
                set_level(0, &levels, &mut level_index, &mut dice);
                game_state = GameState::Ready;
                play_goal();
            }
        }


        if game_state == GameState::Ready || game_state == GameState::Moving {
            let level = &levels[level_index];

            let sx = screen_width()  / level.size.x as f32;
            let sy = screen_height() / level.size.y as f32;
            let s = (sx.min(sy) / 1.5).floor().min(150.0);
            let tile_size = Vec2::splat(s);

            let board_size = level.size.as_f32() * tile_size;
            let screen_size = Vec2::new(screen_width(), screen_height());
            let origin = (screen_size/2.0 - board_size/2.0).floor();

            draw_background(origin, tile_size);

            let t = move_anim.t();
            level.render(origin, tile_size, t);
            draw_moves(&level, &dice, origin, tile_size);
            dice.render(origin, tile_size, &level, t);
        }

        if game_state == GameState::Welcome || game_state == GameState::Done {
            draw_background(Vec2::ZERO, Vec2::splat(150.0));

            let mut tex = *TEX_WELCOME;
            if game_state == GameState::Done {
                tex = *TEX_DONE;
            }

            let size = Vec2::new(tex.width(), tex.height());
            let sx = screen_width()  / size.x;
            let sy = screen_height() / size.y;
            let size = (sx.min(sy) / 1.5 * size).min(size);
            let x = screen_width()/2.0  - size.x/2.0;
            let y = screen_height()/2.0 - size.y/2.0;
            draw_texture_ex(tex, x, y, WHITE, DrawTextureParams {
                dest_size: Some(size),
                .. Default::default()
            });
        }

        next_frame().await;
    }
}
