pub mod scripted_game_entity;

pub mod game_entity {
    use wasmtime::component::bindgen;

    bindgen!({
        path: "./src/scripting/components/",
        world: "gamejam:game/game-entity"
    });
}