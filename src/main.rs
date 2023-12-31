fn main() {
    let mut collisions = 0;
    let light = 1.0;
    let heavy = 1000000.0;

    let mut light_velocity = 0.0;
    let mut heavy_velocity = -1.0;

    while heavy_velocity < light_velocity || light_velocity < 0.0 {
        if heavy_velocity < light_velocity {
            let new_light_velocity =
                (light - heavy) / (light + heavy) * light_velocity
                    + 2.0 * heavy / (light + heavy) * heavy_velocity;
            heavy_velocity =
                2.0 * light / (light + heavy) * light_velocity
                    + (heavy - light) / (light + heavy) * heavy_velocity;
            light_velocity = new_light_velocity;
        } else {
            light_velocity = -light_velocity;
        }
        collisions += 1;
    }

    println!("Collisions: {}", collisions);
}