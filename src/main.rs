use std::thread::sleep;
use engine::collide::Collide2D;
use engine::math::solve::Environment;
use engine::math::{Column, Vector};
use engine::spaces::Space2D;
use engine::{Body, BodyProperties, Engine, Shape};
use macroquad::prelude::*;
use std::time::{Duration, Instant};

const FPS: f32 = 60.0;

#[macroquad::main("MyGame")]
async fn main() {
    let mut body = Body::<Space2D>::at_rest(
        "A".to_string(),
        Shape::Rec(2.0, 2.0),
        Column::vector([20.0, 35.0]),
        Column::vector([0.5]),
        BodyProperties::rectangle(10.0, 2.0, 2.0)
    );

    body.linear.velocity.content[1][0] = -50.0;

    let mut engine = Engine::new(
        vec![
            body,
            Body::at_rest(
                "Marker".to_string(),
                Shape::Rec(1.0, 1.0),
                Column::vector([40.0, 40.0]),
                Column::vector([0.0]),
                BodyProperties::rectangle(1.0/0.0, 2.0, 200.0)
            ),
            Body::at_rest(
                "Left".to_string(),
                Shape::Rec(2.0, 200.0),
                Column::vector([25.0, 15.0]),
                Column::vector([0.0]),
                BodyProperties::rectangle(1.0/0.0, 2.0, 200.0)
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
                // "p=40",
                // "r=sqrt((x_A-p)^2+(y_A-p)^2)",
                // "hatr=((x_A-p)*hati+(y_A-p)*hatj)/r",
                // "G=200",
                // "a_A=-G*hatr",
                "alpha_A=5hatk"
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

    loop {
        clear_background(WHITE);

        engine.tick().unwrap();

        if last_tick.elapsed().as_secs_f32() > 1.0 / FPS {
            println!("WARNING: Engine overloaded by {} ms", ((last_tick.elapsed().as_millis() as f32) - (1000.0 / FPS)).abs());
        } else {
            sleep(Duration::from_secs_f32(1.0 / FPS) - last_tick.elapsed());
        }

        last_tick = Instant::now();

        for x in engine.bodies() {
            match x.shape {
                Shape::Rec(width, height) => {
                    draw_rectangle_ex(
                        (*x.linear.displacement.get(0) as f32) * 10.0,
                        screen_height() - (*x.linear.displacement.get(1) as f32) * 10.0,
                        width as f32 * 10.0,
                        height as f32 * 10.0,
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
