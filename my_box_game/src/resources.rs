use ggez::event::KeyCode;
use specs::World;

// 记录用户输入
#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

// 记录游戏状态
pub enum GameplayState {
    Playing,
    // Won
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32
}

impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(Gameplay::default());
}