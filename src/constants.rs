pub mod image_handles {
    pub const HERO_SHIP_HANDLE_IMAGE: &str = "textures/sprites/ships/asteroids_hero_ship_24x24.png";
    pub const HERO_SHIP_FIRE_HANDLE_IMAGE: &str = "textures/sprites/ships/asteroids_hero_ship_fire_24x24.png";
    pub const SMALL_ASTEROID_HANDLE_IMAGE: &str = "textures/sprites/asteroids/asteroids_small_asteroid_24x24.png";
    pub const MEDIUM_ASTEROID_HANDLE_IMAGE: &str = "textures/sprites/asteroids/asteroids_medium_asteroid_48x48.png";
    pub const BIG_ASTEROID_HANDLE_IMAGE: &str = "textures/sprites/asteroids/asteroids_big_asteroid_96x96.png";
}

pub mod hero_ship_movement_values {
    pub const HERO_SHIP_MOVEMENT_SPEED_DRAG: f32 = 100.;
    pub const HERO_SHIP_INCREMENTAL_MOVEMENT_SPEED: f32 = 5.;
    pub const HERO_SHIP_MAX_MOVEMENT_SPEED: f32 = 320.;
    pub const HERO_SHIP_ROTATION_SPEED_DRAG: f32 = 250.;
    pub const HERO_SHIP_INCREMENTAL_ROTATION_SPEED: f32 = 15.;
    pub const HERO_SHIP_MAX_ROTATION_SPEED: f32 = 360.;
}

pub mod borders {
    pub const BOTTOM_BORDER_POSITION: f32 = -260.;
    pub const TOP_BORDER_POSITION: f32 = 260.;
    pub const LEFT_BORDER_POSITION: f32 = -400.;
    pub const RIGHT_BORDER_POSITION: f32 = 400.;
}
