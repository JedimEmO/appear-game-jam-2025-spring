use wasmtime::component::bindgen;

bindgen!({
    path: "../components/",
    world: "gamejam:game/game-entity-world",
});
