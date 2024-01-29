use dependencies_sync::tonic_build;

fn main() {
    tonic_build::configure()
        .out_dir("src/protocols")
        .build_client(false)
        .build_server(false)
        .extern_path(".cashmere", "::manage_define::cashmere")
        .type_attribute(
            "EventType",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute("Event", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(
            "EventEmitter",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "EventListener",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .compile(
            &["protocols/event_module.proto"],
            &["protocols", "../cashmere_core/protocols"],
        )
        .unwrap();

    define_utils::generate_manage_defines(
        &["manage_defines"],
        "src/ids_codes",
        // None,
        Some("dart_packages/event_module/lib"),
        Some("event_module")
    );
}
