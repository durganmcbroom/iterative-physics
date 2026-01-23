extern crate core;

use core::task;
use std::f64::consts::PI;
use std::iter::Scan;
use std::thread::{sleep, yield_now};
use engine::collide::Collide2D;
use engine::math::solve::Environment;
use engine::math::{Column, Vector};
use engine::spaces::Space2D;
use engine::{Body, BodyProperties, Engine, Shape};
use macroquad::prelude::*;
use std::time::{Duration, Instant};

const FPS: f32 = 60.0;
const SCALE : f32 = 20.0;

#[macroquad::main("MyGame")]
async fn main() {
    let mut body = Body::<Space2D>::at_rest(
        "A".to_string(),
        Shape::Rec(2.0, 2.0),
        Column::vector([25.0, 15.0]),
        Column::vector([0.0]),
        BodyProperties::rectangle(10.0, 2.0, 2.0)
    );

    body.linear.velocity.content[0][0] = -10.0;

    let mut engine = Engine::new(
        vec![
            body,
            // Body::at_rest(
            //     "Marker".to_string(),
            //     Shape::Rec(1.0, 1.0),
            //     Column::vector([40.0, 40.0]),
            //     Column::vector([0.0]),
            //     BodyProperties::rectangle(1.0/0.0, 2.0, 200.0)
            // ),
            // Body::at_rest(
            //     "Bottom".to_string(),
            //     Shape::Rec(200.0, 2.0),
            //     Column::vector([25.0, 10.0]),
            //     Column::vector([PI/16.0]),
            //     BodyProperties::rectangle(1.0/0.0, 200.0, 2.0)
            // ),
            Body::at_rest(
                "Side".to_string(),
                Shape::Rec(1.0, 10.0),
                Column::vector([5.0, 18.0]),
                Column::vector([0.0]),
                BodyProperties::rectangle(1.0, 200.0, 1.0)
            ),

            //
            // Body::at_rest(
            //     "Right".to_string(),
            //     Shape::Rec(2.0, 200.0),
            //     Column::vector([80.0, 15.0]),
            //     Column::vector([0.0]),
            //     BodyProperties::rectangle(1.0/0.0, 2.0, 200.0)
            // ),
            // Body::at_rest(
            //     "Floor".to_string(),
            //     Shape::Rec(1000.0, 2.0),
            //     Column::vector([10.0, 25.0]),
            //     Column::vector([0.0]),
            //     BodyProperties::rectangle(1.0/0.0, 1000.0, 10.0)
            // ),
            // Body::at_rest(
            //     "C".to_string(),
            //     Shape::Rec(1.0, 1.0),
            //     Column::vector([10.0, 40.0]),
            //     Column::vector([0.0]),
            // )
        ],
        Environment::build(
            vec![
                // "omega_Planet = 4pi*hatk",
                // "G=100000",
                // "r(x,y)=sqrt(x^2 + y^2)",
                // "hatr(x,y) = (x*hati + y*hatj)/r(x,y)",
                // "a_Satellite = -G/r(x_Satellite, y_Satellite)*hatr(x_Satellite, y_Satellite)"
            ],
            engine::math::solve::builtin::functions(),
            engine::math::solve::builtin::constants(),
        ).unwrap(),
        Box::new(Collide2D::new()),
        1.0 / FPS as f64,
        1.0
    );

    let mut last_tick = Instant::now();
    let mut tick = 0;

    let mut collisions = Vec::<(Column<2>, Instant)>::new();

    clear_background(WHITE);
    next_frame().await;
    sleep(Duration::new(1, 0));

    loop {
        clear_background(WHITE);

        let tick = engine.tick().unwrap();
        for x in tick.collisions {
            collisions.push((x, Instant::now()));
        }
        if last_tick.elapsed().as_secs_f32() > 1.0 / FPS {
            println!("WARNING: Engine overloaded by {} ms", ((last_tick.elapsed().as_millis() as f32) - (1000.0 / FPS)).abs());
        } else {
            sleep(Duration::from_secs_f32(1.0 / FPS) - last_tick.elapsed());
        }

        // sleep(Duration::from_secs_f32(2.0 / FPS));

        last_tick = Instant::now();

        collisions.retain(|x| x.1.elapsed().as_secs_f32() < 0.5);

        for x in collisions.iter() {
            let scale = x.1.elapsed().as_secs_f32() / 0.5;
            let size = scale * SCALE;
            draw_ellipse(
                screen_width() - (*x.0.get(0) as f32) * SCALE,
                screen_height() - (*x.0.get(1) as f32) * SCALE,
                size,size,
                0.0,
                BLUE.with_alpha(scale)
            )
        }

        for x in engine.bodies() {
            match x.shape {
                Shape::Rec(width, height) => {
                    draw_rectangle_ex(
                        screen_width() - (*x.linear.displacement.get(0) as f32) * SCALE,
                        screen_height() - (*x.linear.displacement.get(1) as f32) * SCALE,
                        width as f32 * SCALE,
                        height as f32 * SCALE,
                        DrawRectangleParams {
                            rotation: *x.angular.displacement.get(0) as f32,
                            offset: vec2(0.5, 0.5),
                            color: RED,
                            ..Default::default()
                        },
                    );
                }
                _ => panic!()
            }
        }

        next_frame().await
    }
}
