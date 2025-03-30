use game_entity_component::gamejam::game::game_host::*;
use glam::Vec2;

pub fn get_vec_to_player() -> Vec2 {
    let player_uniform = get_player_uniform();
    let self_uniform = get_self_uniform();

    let player_pos = glam::Vec2::new(player_uniform.position.0, player_uniform.position.1);
    let self_pos = glam::Vec2::new(self_uniform.position.0, self_uniform.position.1);

    player_pos - self_pos
}

pub fn get_direction_to_player() -> Direction {
    let vec = get_vec_to_player();
    
    if vec.x < 0. {
        Direction::West
    } else {
        Direction::East
    }
}