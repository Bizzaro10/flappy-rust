# Fix Plan for render_score Function

## Issues Identified

1. **Dead Code Warning**: The `render_score` function is never used because it's not registered as a system in the app
2. **Incorrect Entity Setup**: Only one ScoreText entity is created, but the function expects multiple entities (one for each digit)
3. **Missing System Registration**: The function is not added to the app's update systems

## Proposed Fixes

### 1. Fix the render_score function implementation

The current implementation has a logic issue. It expects multiple ScoreText entities but only one is created:

```rust
pub fn render_score(game: Res<Game>, mut query: Query<&mut TextureAtlas, With<ScoreText>>) {
    let score_string = format!("{:03}", game.score); // Ensure at least 3 digits, pad with zeros
    let score_digits: Vec<usize> = score_string
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    for (digit, mut texture_atlas) in score_digits.iter().zip(query.iter_mut()) {
        texture_atlas.index = *digit;
    }
}
```

### 2. Register render_score as a system in main.rs

Add the system to the app in main.rs:

```rust
fn main() {
    App::new()
        .init_resource::<Game>()
        .add_systems(Startup, setup)
        .add_systems(Update, blink_space_bar_text.run_if(is_game_not_active))
        .add_systems(Update, move_background.run_if(is_game_active))
        .add_systems(Update, move_ground.run_if(is_game_active))
        .add_systems(Update, animate_bird.run_if(is_game_active))
        .add_systems(Update, start_game.run_if(is_game_not_active))
        .add_systems(Update, gravity.run_if(is_game_active))
        .add_systems(Update, jump.run_if(is_game_active))
        .add_systems(Update, pipes.run_if(is_game_active))
        .add_systems(Update, score.run_if(is_game_active))
        .add_systems(Update, render_score.run_if(is_game_active)) // Add this line
        .add_plugins(MyPlugin)
        .run();
}
```

### 3. Create multiple ScoreText entities in setup.rs

Instead of creating one ScoreText entity, create three entities positioned horizontally:

```rust
// Create three score digits
let digit_positions = [-350.0, -320.0, -290.0]; // Positions for each digit
for (i, &x_pos) in digit_positions.iter().enumerate() {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/numbers.png"),
            transform: Transform::from_xyz(x_pos, 200.0, 1.0),
            ..default()
        },
        TextureAtlas {
            index: 0,
            layout: number_texture_atlas_layout.clone(),
        },
        ScoreText,
    ));
}
```

## Alternative Approach

Instead of creating multiple entities, we could modify the approach to use a single entity with a more complex rendering system, but the current approach of multiple entities is more straightforward for this use case.
