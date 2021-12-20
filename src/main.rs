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

fn multiply_matrix_point(i: &Point, o: &mut Point, m: &[[f64; 4]; 4]) {
    o.x = i.x * m[0][0] + i.y * m[1][0] + i.z * m[2][0] + m[3][0];
    o.y = i.x * m[0][1] + i.y * m[1][1] + i.z * m[2][1] + m[3][1];
    o.z = i.x * m[0][2] + i.y * m[1][2] + i.z * m[2][2] + m[3][2];
    let w = (i.x * m[0][3] + i.y * m[1][3] + i.z * m[2][3] + m[3][3]) as f64;

    if w != 0.0 {
        o.x /= w;
        o.y /= w;
        o.z /= w;
    }
}

fn main() {
    let height = 240.0;
    let width = 600.0;

    let f_near = 0.1;
    let f_far = 1000.0;
    let f_fov = 90.0;
    let f_aspect_ratio = height / width as f64;
    let y = (f_fov * 0.5 / 180.0 * 3.14159) as f64;
    let f_fov_rad = 1.0 / y.tan();

    let mut projection_matrix = Matrix4x4 { m: [[0.0; 4]; 4] };

    projection_matrix.m[0][0] = f_aspect_ratio * f_fov_rad;
    projection_matrix.m[1][1] = f_fov_rad;
    projection_matrix.m[2][2] = f_far / (f_far - f_near);
    projection_matrix.m[3][2] = (-f_far * f_near) / (f_far - f_near);
    projection_matrix.m[2][3] = 1.0;
    projection_matrix.m[3][3] = 0.0;

    let mut engine = console_engine::ConsoleEngine::init(600, 240, 6).unwrap();
    let _value = 14;
    // main loop, be aware that you'll have to break it because ctrl+C is captured
    let mut f_theta = 0.0;

    loop {
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
        engine.wait_frame();
        // wait for next frame + capture inputs
        engine.clear_screen();

        f_theta += 0.5;

        // ROTATION Z
        let mut rotation_matrix_z = [[0.0; 4]; 4];
        let temp = (f_theta * 0.5) as f64;
        rotation_matrix_z[0][0] = temp.cos();
        rotation_matrix_z[0][1] = temp.sin();
        rotation_matrix_z[1][0] = -temp.sin();
        rotation_matrix_z[1][1] = temp.cos();
        rotation_matrix_z[2][2] = 1.0;
        rotation_matrix_z[3][3] = 1.0;

        // ROTATION X
        let mut rotation_matrix_x = [[0.0; 4]; 4];
        let temp = (f_theta * 0.5) as f64;
        rotation_matrix_x[0][0] = 1.0;
        rotation_matrix_x[1][1] = temp.cos();
        rotation_matrix_x[1][2] = temp.sin();
        rotation_matrix_x[2][1] = -temp.sin();
        rotation_matrix_x[2][2] = temp.cos();
        rotation_matrix_x[3][3] = 1.0;

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
            let mut triangle_rotated_z = Triangle {
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
            let mut triangle_rotated_x = Triangle {
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
            // ROTATE Z
            multiply_matrix_point(
                &triangle.points[0],
                &mut triangle_rotated_z.points[0],
                &rotation_matrix_z,
            );

            multiply_matrix_point(
                &triangle.points[1],
                &mut triangle_rotated_z.points[1],
                &rotation_matrix_z,
            );

            multiply_matrix_point(
                &triangle.points[2],
                &mut triangle_rotated_z.points[2],
                &rotation_matrix_z,
            );

            // ROTATE X
            multiply_matrix_point(
                &triangle_rotated_z.points[0],
                &mut triangle_rotated_x.points[0],
                &rotation_matrix_x,
            );

            multiply_matrix_point(
                &triangle_rotated_z.points[1],
                &mut triangle_rotated_x.points[1],
                &rotation_matrix_x,
            );

            multiply_matrix_point(
                &triangle_rotated_z.points[2],
                &mut triangle_rotated_x.points[2],
                &rotation_matrix_x,
            );

            let mut triangle_translated = triangle_rotated_x;
            triangle_translated.points[0].z += 3.0;
            triangle_translated.points[1].z += 3.0;
            triangle_translated.points[2].z += 3.0;

            multiply_matrix_point(
                &triangle_translated.points[0],
                &mut triangle_projected.points[0],
                &projection_matrix.m,
            );
            multiply_matrix_point(
                &triangle_translated.points[1],
                &mut triangle_projected.points[1],
                &projection_matrix.m,
            );
            multiply_matrix_point(
                &triangle_translated.points[2],
                &mut triangle_projected.points[2],
                &projection_matrix.m,
            );
            triangle_projected.points[0].x += 1.0;
            triangle_projected.points[0].y += 1.0;
            triangle_projected.points[1].x += 1.0;
            triangle_projected.points[1].y += 1.0;
            triangle_projected.points[2].x += 1.0;
            triangle_projected.points[2].y += 1.0;
            triangle_projected.points[0].x *= 0.5 * width;
            triangle_projected.points[0].y *= 0.5 * height;
            triangle_projected.points[1].x *= 0.5 * width;
            triangle_projected.points[1].y *= 0.5 * height;
            triangle_projected.points[2].x *= 0.5 * width;
            triangle_projected.points[2].y *= 0.5 * height;

            draw_triangle(
                &mut engine,
                triangle_projected.points[0].x as i32,
                triangle_projected.points[0].y as i32,
                triangle_projected.points[1].x as i32,
                triangle_projected.points[1].y as i32,
                triangle_projected.points[2].x as i32,
                triangle_projected.points[2].y as i32,
            );
        }

        if engine.is_key_pressed(KeyCode::Char('q')) {
            // if the user presses 'q' :
            break; // exits app
        }

        if engine.is_key_pressed(KeyCode::Char('a')) {
            engine.line(30, 34, 40, 31, pixel::pxl('#'));
        }

        engine.draw(); // draw the screen
    }
}
