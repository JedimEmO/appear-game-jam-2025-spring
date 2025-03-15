pub mod prelude {
    wit_bindgen::generate!({
        path: "../../gamejam-platform-controller/src/scripting/components",
        world: "game-entity",
        pub_export_macro: true
    });
}
