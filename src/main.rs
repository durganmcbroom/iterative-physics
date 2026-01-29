extern crate core;

use core::task;
use engine::collide::Collide2D;
use engine::math::solve::Environment;
use engine::math::{Column, Vector};
use engine::spaces::Space2D;
use engine::{Body, BodyProperties, Engine, Shape};
use macroquad::prelude::*;
use std::f64::consts::PI;
use std::iter::Scan;
use std::thread::{sleep, yield_now};
use std::time::{Duration, Instant};

const FPS: f32 = 60.0;
const SCALE: f32 = 1.0;

#[macroquad::main("MyGame")]
async fn main() {
    // let mut body = Body::<Space2D>::at_rest(
    //     "A".to_string(),
    //     Shape::Rec(40.0, 40.0),
    //     Column::vector([-100.0, -300.0]),
    //     Column::vector([-PI]),
    //     BodyProperties::rectangle(10.0, 1.0, 10.0),
    // );
    //
    // body.linear.velocity.content[0][0] = 50.0;
    // let mut engine = Engine::new(
    //     vec![
    //         body,
    //         Body::at_rest(
    //             "Pendulum".to_string(),
    //             Shape::Rec(20.0, 400.0),
    //             Column::vector([0.0,-200.0]),
    //             Column::vector([0.0]),
    //             BodyProperties::rectangle(10.0, 20.0, 400.0),
    //         ),
    //     ],
    //     Environment::build(
    //         vec![
    //             "s_Pendulum = (200sin(theta_Pendulum))hati+(-200cos(theta_Pendulum))hatj",
    //             "alpha_Pendulum*I_cm_Pendulum=(200*(m_Pendulum*-100)*sin(theta_Pendulum))hatk"
    //         ],
    //         engine::math::solve::builtin::functions(),
    //         engine::math::solve::builtin::constants(),
    //     )
    //     .unwrap(),
    //     Box::new(Collide2D::new()),
    //     1.0 / FPS as f64,
    //     1.0,
    // );
    let mut engine = Engine::new(
        vec![
            Body::<Space2D>::at_rest(
                "B".to_string(),
                Shape::Rec(40.0, 40.0),
                Column::vector([-300.0, 200.0]),
                Column::vector([0.0]),
                BodyProperties::rectangle(1.0, 40.0, 40.0),
            ),
            Body::at_rest(
                "Bumper 2".to_string(),
                Shape::Rec(200.0, 20.0),
                Column::vector([-300.0, -300.0]),
                Column::vector([0.0]),
                BodyProperties::rectangle(10000000000.0, 200.0, 20.0),
            ),

        ],
        Environment::build(
            vec![
                "g=100",
                "a_B=-g*hatj",
            ],
            engine::math::solve::builtin::functions(),
            engine::math::solve::builtin::constants(),
        )
        .unwrap(),
        Box::new(Collide2D::new()),
        1.0 / FPS as f64,
        1.0,
    );

    let mut last_tick = Instant::now();

    let mut collisions = Vec::<(Column<2>, Instant)>::new();

    loop {
        clear_background(WHITE);

        let tick = engine.tick().unwrap();
        for x in tick.collisions {
            collisions.push((x, Instant::now()));
        }
        if last_tick.elapsed().as_secs_f32() > 1.0 / FPS {
            println!(
                "WARNING: Engine overloaded by {} ms",
                ((last_tick.elapsed().as_millis() as f32) - (1000.0 / FPS)).abs()
            );
        } else {
            sleep(Duration::from_secs_f32(1.0 / FPS) - last_tick.elapsed());
        }

        // sleep(Duration::from_secs_f32(3.0 / FPS));

        last_tick = Instant::now();

        for x in engine.bodies() {
            match x.shape {
                Shape::Rec(width, height) => {
                    draw_rectangle_ex(
                        (*x.linear.displacement.get(0) as f32) * SCALE + screen_width() / 2.0,
                        screen_height() / 2.0 - (*x.linear.displacement.get(1) as f32) * SCALE,
                        width as f32 * SCALE,
                        height as f32 * SCALE,
                        DrawRectangleParams {
                            rotation: -*x.angular.displacement.get(0) as f32,
                            offset: vec2(0.5, 0.5),
                            color: RED,
                            ..Default::default()
                        },
                    );
                }
                _ => panic!(),
            }
        }
        collisions.retain(|x| x.1.elapsed().as_secs_f32() < 0.5);
        for (i, x) in collisions.iter().enumerate() {
            let scale = x.1.elapsed().as_secs_f32() / 0.5;
            let size = scale * 10.0;

            draw_ellipse(
                ((*x.0.get(0) as f32) * SCALE) + screen_width() / 2.0,
                screen_height() / 2.0 - (*x.0.get(1) as f32) * SCALE,
                size,
                size,
                0.0,
                if i % 2 == 1 {
                    BLUE.with_alpha(scale)
                } else {
                    BLACK
                },
            )
        }

        next_frame().await
    }
}
