use bevy::prelude::*;


pub fn collision_system(
    player_query: Query<(Entity, &Transform, &Health)>,
    wall_query: Query<&Transform, With<Wall>>,
    mut damage_query: Query<&mut Damage>,
) {
    for (player_entity, player_transform, player_health) in &player_query.iter() {
        for (wall_transform, _wall) in &wall_query.iter() {
            let player_position = player_transform.position;
            let wall_position = wall_transform.position;

            let distance = (player_position - wall_position).magnitude();
            if distance < 2.0 {
                commands.spawn(Damage { amount: 10.0 });
            }
        }
    }
}


