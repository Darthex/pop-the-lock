use rand::prelude::*;

/// Returns a random element from a slice
pub fn random_choice<T>(items: &[T]) -> &T {
    let mut rng = rand::rng();
    items.choose(&mut rng).unwrap()
}

const QUADRANTS: [(f32, f32); 4] = [
    (0.0, std::f32::consts::FRAC_PI_2),                          // Q1: 0° to 90°
    (std::f32::consts::FRAC_PI_2, std::f32::consts::PI),         // Q2: 90° to 180°
    (std::f32::consts::PI, 3.0 * std::f32::consts::FRAC_PI_2),   // Q3: 180° to 270°
    (3.0 * std::f32::consts::FRAC_PI_2, std::f32::consts::TAU),  // Q4: 270° to 360°
];

// (clockwise options, anticlockwise options)
const OPPOSITE_QUADS: [([usize; 2], [usize; 2]); 4] = [
    ([3, 2], [1, 2]), // Q1 → clockwise: Q4/Q3, anticlockwise: Q2/Q3
    ([0, 3], [2, 3]), // Q2 → clockwise: Q1/Q4, anticlockwise: Q3/Q4
    ([0, 1], [0, 3]), // Q3 → clockwise: Q1/Q2, anticlockwise: Q1/Q4
    ([1, 2], [0, 1]), // Q4 → clockwise: Q2/Q3, anticlockwise: Q1/Q2
];

pub fn get_safe_dot_angle(trigger_angle: f32, direction: f32) -> f32 {
    let angle = trigger_angle.rem_euclid(std::f32::consts::TAU);

    // find which quadrant trigger is in
    let quad_index = QUADRANTS.iter().position(|(low, high)| {
        angle >= *low && angle < *high
    }).unwrap_or(0);

    // pick allowed quadrants based on direction
    let allowed = if direction > 0.0 {
        OPPOSITE_QUADS[quad_index].0 // clockwise
    } else {
        OPPOSITE_QUADS[quad_index].1 // anticlockwise
    };

    // pick one of the two allowed quadrants randomly
    let chosen_quad = allowed[rand::random::<bool>() as usize];
    let (low, high) = QUADRANTS[chosen_quad];

    // random angle within that quadrant with a small buffer
    let buffer = 0.1;
    rand::random::<f32>() * (high - low - buffer * 2.0) + low + buffer
}