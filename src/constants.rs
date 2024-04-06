pub mod image_handles {
    pub const HERO_SHIP_HANDLE_IMAGE: &str = "textures/sprites/ships/asteroids_hero_ship_24x24.png";
    pub const HERO_SHIP_FIRE_HANDLE_IMAGE: &str = "textures/sprites/ships/asteroids_hero_ship_fire_24x24.png";
    pub const SMALL_ASTEROID_HANDLE_IMAGE: &str = "textures/sprites/asteroids/asteroids_small_asteroid_24x24.png";
    pub const MEDIUM_ASTEROID_HANDLE_IMAGE: &str = "textures/sprites/asteroids/asteroids_medium_asteroid_48x48.png";
    pub const BIG_ASTEROID_HANDLE_IMAGE: &str = "textures/sprites/asteroids/asteroids_big_asteroid_150x150.png";
    pub const PROJECTILE_HANDLE_IMAGE: &str = "textures/sprites/projectiles/ship_projectile_4x4.png";
}

pub mod borders {
    pub const RIGHT_BORDER_POSITION: f32 = 400.;
    pub const LEFT_BORDER_POSITION: f32 = -400.;
    pub const TOP_BORDER_POSITION: f32 = 260.;
    pub const BOTTOM_BORDER_POSITION: f32 = -260.;
    pub const RIGHT_BORDER_OFFSCREEN_POSITION: f32 = 460.;
    pub const LEFT_BORDER_OFFSCREEN_POSITION: f32 = -460.;
    pub const TOP_BORDER_OFFSCREEN_POSITION: f32 = 320.;
    pub const BOTTOM_BORDER_OFFSCREEN_POSITION: f32 = -320.;
}

pub mod hero_ship_movement_values {
    pub const HERO_SHIP_MOVEMENT_SPEED_DRAG: f32 = 100.;
    pub const HERO_SHIP_INCREMENTAL_MOVEMENT_SPEED: f32 = 5.;
    pub const HERO_SHIP_MAX_MOVEMENT_SPEED: f32 = 320.;
    pub const HERO_SHIP_ROTATION_SPEED_DRAG: f32 = 250.;
    pub const HERO_SHIP_INCREMENTAL_ROTATION_SPEED: f32 = 15.;
    pub const HERO_SHIP_MAX_ROTATION_SPEED: f32 = 360.;
}

pub mod asteroid {
    pub const INITIAL_BIG_ASTEROIDS_ONSCREEN: usize = 4;
}

pub mod asteroid_movement_values {
    pub const SMALL_ASTEROID_MOVEMENT_SPEED: f32 = 320.;
    pub const SMALL_ASTEROID_ROTATION_SPEED: f32 = 150.;
    pub const MEDIUM_ASTEROID_MOVEMENT_SPEED: f32 = 160.;
    pub const MEDIUM_ASTEROID_ROTATION_SPEED: f32 = 100.;
    pub const BIG_ASTEROID_MOVEMENT_SPEED: f32 = 80.;
    pub const BIG_ASTEROID_ROTATION_SPEED: f32 = 40.;
}

pub mod projectile_movement_values {
    pub const PROJECTILE_MOVEMENT_SPEED: f32 = 200.;    
}
