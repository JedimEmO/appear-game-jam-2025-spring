pub mod prelude {

}
wit_bindgen::generate!({
    path: "../../script-wasi-components/components",
    world: "game-entity-world",
    pub_export_macro: true,
});