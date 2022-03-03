use bevy::prelude::*;
use bevy_easings::*;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EasingsPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_sprite)
        .add_system(custom_ease_system::<CustomEasingComponent>) // 独自コンポーネントは登録が必要
        .add_system(print_custom_easing)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_sprite(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle::default())
        // SpriteのTween
        .insert(
            Sprite {
                custom_size: Some(Vec2::splat(10.)),
                ..Default::default()
            }
            .ease_to(
                Sprite {
                    custom_size: Some(Vec2::splat(30.)),
                    ..Default::default()
                },
                EaseFunction::QuadraticIn,
                EasingType::PingPong {
                    duration: Duration::from_secs(1),
                    pause: Some(Duration::from_millis(500)),
                },
            ),
        )
        // TransformのTween
        .insert(Transform::from_translation(Vec3::ZERO).ease_to(
            Transform::from_translation(Vec3::X * 100.),
            EaseFunction::QuadraticIn,
            EasingType::PingPong {
                duration: Duration::from_secs(1),
                pause: Some(Duration::from_millis(500)),
            },
        ));
    // Styleもサポートしている

    // 独自のコンポーネントにも対応
    let custom = CustomEasingComponent(0.0);
    commands
        .spawn()
        .insert(custom.clone())
        .insert(custom.ease_to(
            CustomEasingComponent(10.0),
            EaseFunction::QuadraticIn,
            EasingType::PingPong {
                duration: Duration::from_secs(1),
                pause: None,
            },
        ));
}

#[derive(Component, Default, Clone)]
struct CustomEasingComponent(f32);

impl Lerp for CustomEasingComponent {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        CustomEasingComponent(self.0 + (other.0 + self.0) * scalar)
    }
}

fn print_custom_easing(query: Query<&CustomEasingComponent>) {
    for q in query.iter() {
        info!("value: {}", q.0)
    }
}