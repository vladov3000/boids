use crate::boids::{BoidController, Boid};
use sdl2::rect::Point;
use std::f64::consts::PI;
use std::ops::Add;
use crate::utils::point_to_angle;

pub struct RuleContext<'a, 'b, 'c, 'd, 'e, 'f> {
    pub boid_controller: &'c BoidController<'a, 'b>,
    pub nearby_boids: &'e Vec<&'d Boid>,
    pub boid: &'f Boid,
    pub canvas_w: i32,
    pub canvas_h: i32,
}

pub struct Rule {
    // takes in context and returns new boid angle
    pub dir_fn: fn(&RuleContext) -> f64,

    // larger weight => larger effect
    // weight is relative to other rules
    pub weight: f64,
}

pub mod default_dir_fns {
    use crate::rule::RuleContext;
    use sdl2::rect::Point;
    use crate::utils::point_to_angle;
    use std::f64::consts::PI;
    use std::ops::Add;

    /// steer to avoid crowding local flock mates
    pub fn separation(ctx: &RuleContext) -> f64 {
        let &RuleContext { nearby_boids, boid, .. } = ctx;

        if nearby_boids.is_empty() {
            boid.angle
        } else {
            // take sum of deltas to centers of nearby boids
            let deltas_sum = nearby_boids.iter()
                .map(|nearby_boid| nearby_boid.rect.center() - boid.rect.center())
                .fold(Point::new(0, 0), Point::add);

            // and go in the opposite direction
            point_to_angle(deltas_sum) + PI
        }
    }

    /// steer towards the average heading of local flock mates
    pub fn alignment(ctx: &RuleContext) -> f64 {
        let &RuleContext { nearby_boids, boid, .. } = ctx;

        // compute average of angles of nearby boids
        if nearby_boids.is_empty() {
            boid.angle
        } else {
            nearby_boids.iter()
                .map(|boid| boid.angle)
                .sum::<f64>() / nearby_boids.len() as f64
        }
    }

    /// steer to move towards the average position (center of mass) of local flock mates
    pub fn cohesion(ctx: &RuleContext) -> f64 {
        let &RuleContext { nearby_boids, boid, .. } = ctx;
        let boids_centroid = nearby_boids.iter()
            .map(|boid| boid.rect.center())
            .fold(Point::new(0, 0), Point::add);
        point_to_angle(boids_centroid - boid.rect.center())
    }
}