use bevy::prelude::{
    default, shape::Cube, App, AssetPlugin, AssetServer, Assets, Camera3dBundle, Color, Commands,
    DefaultPlugins, DirectionalLight, DirectionalLightBundle, MaterialMeshBundle, Mesh,
    PluginGroup, PointLight, PointLightBundle, Quat, Res, ResMut, StandardMaterial, Transform,
    Vec3,
};

use bevy_rust_gpu::prelude::{LoadRustGpuShader, RustGpu, RustGpuMaterialPlugin, RustGpuPlugin};

#[cfg(feature = "entry-point-export")]
use bevy_rust_gpu::prelude::EntryPointExport;

const SHADER_PATH: &'static str = "rust-gpu/target/spirv-unknown-spv1.5/release/deps/shader.spv";

fn main() {
    let mut app = App::default();

    // Add the Rust-GPU plugin
    app.add_plugin(RustGpuPlugin);

    // Add default plugins
    app.add_plugins(DefaultPlugins.set(
        // Configure the asset plugin to watch the workspace path for changes
        AssetPlugin {
            asset_folder: "../../../".into(),
            watch_for_changes: true,
            ..default()
        },
    ));

    // Setup `RustGpu<StandardMaterial>`
    app.add_plugin(RustGpuMaterialPlugin::<StandardMaterial>::default());

    // Setup scene
    app.add_startup_system(setup);

    // Run
    app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut shader_materials: ResMut<Assets<RustGpu<StandardMaterial>>>,
    #[cfg(feature = "entry-point-export")] mut exports: bevy::prelude::NonSendMut<EntryPointExport>,
) {
    // Spawn camera
    commands.spawn(Camera3dBundle::default());

    // Spawn lights
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5000.0,
            ..default()
        },
        transform: Transform::IDENTITY.looking_at(Vec3::new(0.0, -1.0, -1.0), Vec3::Y),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 800.0,
            color: Color::BLUE,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, -6.0),
        ..default()
    });

    // Create mesh, shader and material assets
    let standard_material = standard_materials.add(default());

    let mesh = meshes.add(Cube { size: 1.0 }.into());

    let shader = asset_server.load_rust_gpu_shader(SHADER_PATH);

    let shader_material = shader_materials.add(RustGpu {
        vertex_shader: Some(shader.clone()),
        fragment_shader: Some(shader),
        #[cfg(feature = "entry-point-export")]
        sender: Some(exports.export("crates/viewer/entry_points.json")),
        ..default()
    });

    // Spawn example cubes
    commands.spawn(MaterialMeshBundle {
        transform: Transform::from_xyz(-1.0, 0.0, -6.0)
            .with_rotation(Quat::from_axis_angle(Vec3::new(1.0, 1.0, 1.0), 45.0).normalize()),
        mesh: mesh.clone(),
        material: standard_material,
        ..default()
    });

    commands.spawn(MaterialMeshBundle {
        transform: Transform::from_xyz(1.0, 0.0, -6.0)
            .with_rotation(Quat::from_axis_angle(Vec3::new(-1.0, 1.0, 1.0), -45.0).normalize()),
        mesh,
        material: shader_material,
        ..default()
    });
}
