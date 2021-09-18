use std::borrow::Borrow;
use std::convert::TryInto;
use std::f64::consts::{PI, TAU};
use std::ops::Add;

use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};

use crate::rule::{Rule, RuleContext};
use crate::utils::PolarVector;

#[derive(Copy, Clone)]
pub struct Boid {
    pub rect: Rect,
    pub angle: f64,  // in radians
}

pub struct BoidController<'a, 'b> {
    pub boids: Vec<Boid>,
    pub boid_texture: &'b Texture<'a>,
    pub rules: Vec<Rule>,
    pub boid_speed: f64,
    pub boid_turn_resistance: f64,
    pub nearby_range: i32,
}

impl<'a, 'b> BoidController<'a, 'b> {
    pub fn update(&mut self, canvas_size: (u32, u32)) {
        let mut new_boids = Vec::new();

        for (i, boid) in self.boids.iter().enumerate() {
            let mut nearby_boids = Vec::new();

            for (j, other_boid) in self.boids.iter().enumerate() {
                if i != j && self.points_nearby(boid.rect.center(), other_boid.rect.center()) {
                    nearby_boids.push(other_boid);
                }
            }

            new_boids.push(self.update_boid(RuleContext {
                boid_controller: self.borrow(),
                nearby_boids: &nearby_boids,
                boid,
                canvas_w: canvas_size.0.try_into().unwrap(),
                canvas_h: canvas_size.1.try_into().unwrap(),
            }));
        }

        self.boids = new_boids;
    }

    fn update_boid(&self, ctx: RuleContext) -> Boid {
        let RuleContext {
            boid,
            canvas_w,
            canvas_h,
            ..
        } = ctx;

        let new_angle = if self.rules.is_empty() {
            boid.angle
        } else {
            let target_angle = self.rules.iter()
                .map(|rule| PolarVector {
                    r: rule.weight,
                    theta: (rule.dir_fn)(&ctx),
                })
                .reduce(|acc, x| {
                    (acc + x).set_r(1.)
                }).unwrap().theta;

            boid.angle + (target_angle - boid.angle) / self.boid_turn_resistance
        };

        // move boid
        let new_x = (boid.rect.x() + (new_angle.sin() * self.boid_speed) as i32).rem_euclid(canvas_w);
        let new_y = (boid.rect.y() - (new_angle.cos() * self.boid_speed) as i32).rem_euclid(canvas_h);

        Boid {
            rect: Rect::new(new_x, new_y, boid.rect.width(), boid.rect.height()),
            angle: new_angle,
        }
    }

    fn points_nearby(&self, point: Point, other_point: Point) -> bool {
        let delta = other_point - point;
        delta.x().pow(2) + delta.y().pow(2) <= self.nearby_range.pow(2)
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        for boid in &self.boids {
            canvas.copy_ex(
                self.boid_texture,
                None,
                Some(boid.rect),
                boid.angle.to_degrees(),
                None,
                false,
                false,
            ).unwrap();
        }
    }
}