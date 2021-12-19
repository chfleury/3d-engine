use console_engine::pixel;
// use console_engine::Color;
use console_engine::KeyCode;

struct Point {
    x: f64,
    y: f64,
    z: f64,
}

struct Triangle {
    points: [Point; 3],
}

struct Mesh {
    tris: Vec<Triangle>,
}

struct Matrix4x4 {
    m: [[f64; 4]; 4],
}

fn draw_triangle(
    engine: &mut console_engine::ConsoleEngine,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
) {
    engine.line(x1, y1, x2, y2, pixel::pxl('#'));
    engine.line(x2, y2, x3, y3, pixel::pxl('#'));
    engine.line(x3, y3, x1, y1, pixel::pxl('#'));
}

fn multiply_matrix_point(i: &Point, o: &mut Point, m: &Matrix4x4) {
    o.x = i.x * m.m[0][0] + i.y * m.m[1][0] + i.z * m.m[2][0] + m.m[3][0];
    o.y = i.x * m.m[0][1] + i.y * m.m[1][1] + i.z * m.m[2][1] + m.m[3][1];
    o.z = i.x * m.m[0][2] + i.y * m.m[1][2] + i.z * m.m[2][2] + m.m[3][2];
    let w = (i.x * m.m[0][3] + i.y * m.m[1][3] + i.z * m.m[2][3] + m.m[3][3]) as f64;

    if w != 0.0 {
        o.x /= w;
        o.y /= w;
        o.z /= w;
    }
}

fn main() {
    let mesh_cube = Mesh {
        tris: vec![
            // SOUTH
            Triangle {
                points: [
                    Point {
                        x: 0.0_f64,
                        y: 0.0_f64,
                        z: 0.0_f64,
                    },
                    Point {
                        x: 0.0_f64,
                        y: 1.0_f64,
                        z: 0.0_f64,
                    },
                    Point {
                        x: 1.0_f64,
                        y: 1.0_f64,
                        z: 0.0_f64,
                    },
                ],
            },
            Triangle {
                points: [
                    Point {
                        x: 0.0_f64,
                        y: 0.0_f64,
                        z: 0.0_f64,
                    },
                    Point {
                        x: 1.0_f64,
                        y: 1.0_f64,
                        z: 0.0_f64,
                    },
                    Point {
                        x: 1.0_f64,
                        y: 0.0_f64,
                        z: 0.0_f64,
                    },
                ],
            },
            // EAST
            Triangle {
                points: [
                    Point {
                        x: 1.0_f64,
                        y: 0.0_f64,
                        z: 0.0_f64,
                    },
                    Point {
                        x: 1.0_f64,
                        y: 1.0_f64,
                        z: 0.0_f64,
                    },
                    Point {
                        x: 1.0_f64,
                        y: 1.0_f64,
                        z: 1.0_f64,
                    },
                ],
            },
            Triangle {
                points: [
                    Point {
                        x: 1.0_f64,
                        y: 0.0_f64,
                        z: 0.0_f64,
                    },
                    Point {
                        x: 1.0_f64,
                        y: 1.0_f64,
                        z: 1.0_f64,
                    },
                    Point {
                        x: 1.0_f64,
                        y: 0.0_f64,
                        z: 1.0_f64,
                    },
                ],
            },
            // NORTH
            Triangle {
                points: [
                    Point {
                        x: 1.0_f64,
                        y: 0.0_f64,
                        z: 1.0_f64,
                    },
                    Point {
                        x: 1.0_f64,
                        y: 1.0_f64,
                        z: 1.0_f64,
                    },
                    Point {
                        x: 0.0_f64,
                        y: 1.0_f64,
                        z: 1.0_f64,
                    },
                ],
            },
            Triangle {
                points: [
                    Point {
                        x: 1.0_f64,
                        y: 0.0_f64,
                        z: 1.0_f64,
                    },
                    Point {
                        x: 0.0_f64,
                        y: 1.0_f64,
                        z: 1.0_f64,
                    },
                    Point {
                        x: 0.0_f64,
                        y: 0.0_f64,
                        z: 1.0_f64,
                    },
                ],
            },
            // WEST
            Triangle {
                points: [
                    Point {
                        x: 0.0_f64,
                        y: 0.0_f64,
                        z: 1.0_f64,
                    },
                    Point {
                        x: 0.0_f64,
                        y: 1.0_f64,
                        z: 1.0_f64,
                    },
                    Point {
                        x: 0.0_f64,
                        y: 1.0_f64,
                        z: 0.0_f64,
                    },
                ],
            },
            Triangle {
                points: [
                    Point {
                        x: 0.0_f64,
                        y: 0.0_f64,
                        z: 1.0_f64,
                    },
                    Point {
                        x: 0.0_f64,
                        y: 1.0_f64,
                        z: 0.0_f64,
                    },
                    Point {
                        x: 0.0_f64,
                        y: 0.0_f64,
                        z: 0.0_f64,
                    },
                ],
            },
            // TOP
            Triangle {
                points: [
                    Point {
                        x: 0.0_f64,
                        y: 1.0_f64,
                        z: 0.0_f64,
                    },
                    Point {
                        x: 0.0_f64,
                        y: 1.0_f64,
                        z: 1.0_f64,
                    },
                    Point {
                        x: 1.0_f64,
                        y: 1.0_f64,
                        z: 1.0_f64,
                    },
                ],
            },
            Triangle {
                points: [
                    Point {
                        x: 0.0_f64,
                        y: 1.0_f64,
                        z: 0.0_f64,
                    },
                    Point {
                        x: 1.0_f64,
                        y: 1.0_f64,
                        z: 1.0_f64,
                    },
                    Point {
                        x: 1.0_f64,
                        y: 1.0_f64,
                        z: 0.0_f64,
                    },
                ],
            },
            // BOTTON
            Triangle {
                points: [
                    Point {
                        x: 1.0_f64,
                        y: 0.0_f64,
                        z: 1.0_f64,
                    },
                    Point {
                        x: 0.0_f64,
                        y: 0.0_f64,
                        z: 1.0_f64,
                    },
                    Point {
                        x: 0.0_f64,
                        y: 0.0_f64,
                        z: 0.0_f64,
                    },
                ],
            },
            Triangle {
                points: [
                    Point {
                        x: 1.0_f64,
                        y: 0.0_f64,
                        z: 1.0_f64,
                    },
                    Point {
                        x: 0.0_f64,
                        y: 0.0_f64,
                        z: 0.0_f64,
                    },
                    Point {
                        x: 1.0_f64,
                        y: 0.0_f64,
                        z: 0.0_f64,
                    },
                ],
            },
        ],
    };

    let height = 40.0;
    let width = 100.0;

    let fNear = 0.1;
    let fFar = 1000.0;
    let fFov = 90.0;
    let fAspectRatio = height / width as f64;
    let y = (fFov * 0.5 / 180.0 * 3.14159) as f64;
    let fFovRad = 1.0 / y.tan();

    let mut projection_matrix = Matrix4x4 { m: [[0.0; 4]; 4] };

    projection_matrix.m[0][0] = fAspectRatio * fFovRad;
    projection_matrix.m[1][1] = fFovRad;
    projection_matrix.m[2][2] = fFar / (fFar - fNear);
    projection_matrix.m[3][2] = (-fFar * fNear) / (fFar - fNear);
    projection_matrix.m[2][3] = 1.0;
    projection_matrix.m[3][3] = 0.0;

    for triangle in mesh_cube.tris {
        let mut triangle_projected = Triangle {
            points: [
                Point {
                    x: 0.0_f64,
                    y: 0.0_f64,
                    z: 0.0_f64,
                },
                Point {
                    x: 0.0_f64,
                    y: 0.0_f64,
                    z: 0.0_f64,
                },
                Point {
                    x: 0.0_f64,
                    y: 0.0_f64,
                    z: 0.0_f64,
                },
            ],
        };

        multiply_matrix_point(
            &triangle.points[0],
            &mut triangle_projected.points[0],
            &projection_matrix,
        );
        multiply_matrix_point(
            &triangle.points[1],
            &mut triangle_projected.points[1],
            &projection_matrix,
        );
        multiply_matrix_point(
            &triangle.points[2],
            &mut triangle_projected.points[2],
            &projection_matrix,
        );
    }

    // initializes a screen of 20x10 characters with a target of 3 frames per second
    // coordinates will range from [0,0] to [19,9]
    let mut engine = console_engine::ConsoleEngine::init(20, 10, 3).unwrap();
    let _value = 14;
    // main loop, be aware that you'll have to break it because ctrl+C is captured
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        engine.clear_screen(); // reset the screen

        draw_triangle(&mut engine, 0, 0, 0, 10, 10, 0);

        if engine.is_key_pressed(KeyCode::Char('q')) {
            // if the user presses 'q' :
            break; // exits app
        }

        engine.draw(); // draw the screen
    }
}
