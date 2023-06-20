use crate::Player;
use bevy::{
    prelude::*,
    render::{
        camera::{RenderTarget, ScalingMode},
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::BevyDefault,
        view::RenderLayers,
    },
    sprite::MaterialMesh2dBundle,
};

#[derive(Resource)]
pub struct MainRender(pub Handle<Image>);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct FinalCamera;

fn setup_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 500,
        height: 500,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(size);

    let image_handle = images.add(image);

    let mut camera = Camera2dBundle::default();
    camera.camera.target = RenderTarget::Image(image_handle.clone());

    commands.spawn((
        camera,
        MainCamera,
        VisibilityBundle::default(),
        UiCameraConfig { show_ui: false },
    ));

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(500.0, 500.0))));

    let material_handle = materials.add(ColorMaterial {
        texture: Some(image_handle.clone()),
        ..default()
    });

    let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: quad_handle.into(),
            material: material_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        },
        post_processing_pass_layer,
        Name::new("Base Render"),
    ));

    commands.insert_resource(MainRender(image_handle));

    let mut camera = Camera2dBundle::default();
    camera.camera.order = 999;
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 500.0,
        min_height: 500.0,
    };

    commands.spawn((
        camera,
        post_processing_pass_layer,
        FinalCamera,
        UiCameraConfig { show_ui: true },
    ));
}

fn follow_player(
    mut camera_transform_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player_transform_query: Query<&Transform, With<Player>>,
) {
    info!("Camera follow player running");
    let Some(mut camera_transform) = camera_transform_query.iter_mut().next() else { return };
    info!("There is a camera: ${}", camera_transform.translation);
    let Some(player_transform) = player_transform_query.iter().next() else { return };
    info!("There is a player: ${}", player_transform.translation);

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_system(follow_player.in_base_set(CoreSet::PostUpdate));
    }
}
