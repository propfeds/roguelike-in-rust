use rltk::{RGB, Rltk, RandomNumberGenerator};
use super::Rect;
use std::cmp::{max, min};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType
{
    Wall,
    Floor
}

pub fn xy_idx(x: i32, y: i32) -> usize
{
    (y as usize * 80) + x as usize
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType])
{
    for y in room.y1 + 1 ..= room.y2
    {
        for x in room.x1 + 1 ..= room.x2
        {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

pub fn new_map_rooms_corridors() -> Vec<TileType>
{
    let mut map = vec![TileType::Wall; 80*50];

    let room1 = Rect::new(20, 15, 10, 15);
    let room2 = Rect::new(35, 15, 10, 15);

    apply_room_to_map(&room1, &mut map);
    apply_room_to_map(&room2, &mut map);

    map
}

/// Test rust function commentos
pub fn new_map_test() -> Vec<TileType>
{
    let mut map = vec![TileType::Floor; 80*50];

    // Boundary walls
    for x in 0..80
    {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50
    {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    let mut rng = rltk::RandomNumberGenerator::new();
    let player_pos_idx = xy_idx(40, 25);
    for _i in 0..400
    {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        if idx != player_pos_idx
        {
            map[idx] = TileType::Wall;
        }
    }
    map
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk)
{
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter()
    {
        match tile
        {
            TileType::Floor =>
            {
                ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5),
                RGB::from_f32(0., 0., 0.), rltk::to_cp437('.'));
            }
            TileType::Wall =>
            {
                ctx.set(x, y, RGB::from_f32(0., 1., 0.),
                RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
            }
        }

        x += 1;
        if x > 79
        {
            x = 0;
            y += 1;
        }
    }
}
