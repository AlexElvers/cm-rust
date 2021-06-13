#[derive(Copy, Clone, Debug)]
pub enum KeyColor { Yellow, Red, Grey }

#[derive(Copy, Clone, Debug)]
pub enum GemKind { Gem1, Gem2, Gem3 }

#[derive(Copy, Clone, Debug)]
pub enum Object {
    Dirt { north: bool, east: bool, south: bool, west: bool },
    Wall { north: bool, east: bool, south: bool, west: bool },
    Empty,
    Door { frame_offset: u8 },
    Brick,
    Key { color: KeyColor, frame_offset: u8 },
    Pickaxe { frame_offset: u8 },
    Gem { kind: GemKind, frame_offset: u8 },
    Letter { char: char },
    Boulder,
    Balloon,
    AirshipLeft { frame_offset: u8 },
    AirshipRight { frame_offset: u8 },
    Player,
    Enemy,
    Lock { color: KeyColor },
}

impl Object {
    pub fn from_tile_number(sprite_number: u8) -> Result<Object, String> {
        match sprite_number {
            0..=15 | 32 => Ok(Object::Dirt {
                north: false,
                east: false,
                south: false,
                west: false,
            }),
            16..=31 | 33 => Ok(Object::Wall {
                north: false,
                east: false,
                south: false,
                west: false,
            }),
            34 | 189 => Ok(Object::Empty),
            35..=38 => Ok(Object::Door { frame_offset: 0 }),
            39 => Ok(Object::Brick),
            40..=45 => Ok(Object::Key { color: KeyColor::Yellow, frame_offset: sprite_number - 40 }),
            46..=51 => Ok(Object::Key { color: KeyColor::Red, frame_offset: sprite_number - 46 }),
            52..=57 => Ok(Object::Key { color: KeyColor::Grey, frame_offset: sprite_number - 52 }),
            58..=63 => Ok(Object::Pickaxe { frame_offset: sprite_number - 58 }),
            64..=69 => Ok(Object::Gem { kind: GemKind::Gem1, frame_offset: sprite_number - 64 }),
            70..=75 => Ok(Object::Gem { kind: GemKind::Gem2, frame_offset: sprite_number - 70 }),
            76..=81 => Ok(Object::Gem { kind: GemKind::Gem3, frame_offset: sprite_number - 76 }),
            82..=117 | 190 | 191 => Ok(Object::Letter { char: 'a' }),
            118 => Ok(Object::Boulder),
            119 => Ok(Object::Balloon),
            120..=125 => Ok(Object::AirshipLeft { frame_offset: sprite_number - 120 }),
            126..=131 => Ok(Object::AirshipRight { frame_offset: sprite_number - 126 }),
            132..=161 => Ok(Object::Player),
            162..=185 => Ok(Object::Enemy),
            186 => Ok(Object::Lock { color: KeyColor::Yellow }),
            187 => Ok(Object::Lock { color: KeyColor::Red }),
            188 => Ok(Object::Lock { color: KeyColor::Grey }),
            _ => Err(format!("invalid sprite number {}", sprite_number)),
        }
    }

    pub fn tile_number(self, tick_number: u8) -> u8 {
        match self {
            Object::Dirt { north, east, south, west } => {
                0 | north as u8 | (east as u8) << 1 | (south as u8) << 2 | (west as u8) << 3
            }
            Object::Wall { north, east, south, west } => {
                16 | north as u8 | (east as u8) << 1 | (south as u8) << 2 | (west as u8) << 3
            }
            Object::Empty => 34,
            Object::Door { .. } => 35,  // TODO unlocked door
            Object::Brick => 39,
            Object::Key { color, frame_offset } => match color {
                KeyColor::Yellow => 40 + (frame_offset + tick_number) % 6,
                KeyColor::Red => 46 + (frame_offset + tick_number) % 6,
                KeyColor::Grey => 52 + (frame_offset + tick_number) % 6,
            },
            Object::Pickaxe { frame_offset } => 58 + (frame_offset + tick_number) % 6,
            Object::Gem { kind, frame_offset } => match kind {
                GemKind::Gem1 => 64 + (frame_offset + tick_number) % 6,
                GemKind::Gem2 => 70 + (frame_offset + tick_number) % 6,
                GemKind::Gem3 => 76 + (frame_offset + tick_number) % 6,
            },
            Object::Letter { .. } => 82,  // TODO char
            Object::Boulder => 118,
            Object::Balloon => 119,
            Object::AirshipLeft { frame_offset } => 120 + (frame_offset + tick_number) % 6,
            Object::AirshipRight { frame_offset } => 126 + (frame_offset + tick_number) % 6,
            Object::Player => 156,  // TODO animation
            Object::Enemy => 180,  // TODO animation
            Object::Lock { color } => match color {
                KeyColor::Yellow => 186,
                KeyColor::Red => 187,
                KeyColor::Grey => 188,
            }
        }
    }

    pub fn is_transparent(self) -> bool {
        match self {
            Object::Gem { .. }
            | Object::Boulder
            | Object::Balloon
            | Object::AirshipLeft { .. }
            | Object::AirshipRight { .. }
            | Object::Player
            | Object::Enemy => true,
            _ => false,
        }
    }

    pub fn can_be_entered(self) -> bool {
        match self {
            Object::Empty
            | Object::Dirt { .. }
            | Object::Key { .. }
            | Object::Pickaxe { .. }
            | Object::Gem { .. }
            | Object::Letter { .. } => true,
            _ => false,
        }
    }
}
