use console_engine::pixel;
// use console_engine::Color;
use console_engine::KeyCode;

struct Point {
    x: f32,
    y: f32,
    z: f32,
}

struct Triangle {
    points: [Point; 3],
}

struct Mesh {
    tris: Vec<Triangle>,
}

struct Matrix4x4 {
    m: [[i32; 4]; 4],
}

fn draw_triangle(engine: &mut console_engine::ConsoleEngine ,x1: i32, y1: i32, x2:i32, y2:i32, x3:i32, y3:i32) {
    engine.line(x1, y1, x2, y2, pixel::pxl('#'));
    engine.line(x2, y2, x3, y3, pixel::pxl('#'));
    engine.line(x3, y3, x1, y1, pixel::pxl('#'));
}

fn main() {

    let meshCube = Mesh {
        tris: vec![
            // SOUTH
            Triangle {
                points: 
                    [
                     Point { x: 0.0_f32, y: 0.0_f32, z: 0.0_f32},
                     Point { x: 0.0_f32, y: 1.0_f32, z: 0.0_f32},
                     Point { x: 1.0_f32, y: 1.0_f32, z: 0.0_f32},
                    ]
            },
            Triangle {
                points: 
                    [
                     Point { x: 0.0_f32, y: 0.0_f32, z: 0.0_f32},
                     Point { x: 1.0_f32, y: 1.0_f32, z: 0.0_f32},
                     Point { x: 1.0_f32, y: 0.0_f32, z: 0.0_f32},
                    ]
            },
            // EAST
            Triangle {
                points: 
                    [
                     Point { x: 1.0_f32, y: 0.0_f32, z: 0.0_f32},
                     Point { x: 1.0_f32, y: 1.0_f32, z: 0.0_f32},
                     Point { x: 1.0_f32, y: 1.0_f32, z: 1.0_f32},
                    ]
            },
            Triangle {
                points: 
                    [
                     Point { x: 1.0_f32, y: 0.0_f32, z: 0.0_f32},
                     Point { x: 1.0_f32, y: 1.0_f32, z: 1.0_f32},
                     Point { x: 1.0_f32, y: 0.0_f32, z: 1.0_f32},
                    ]
            },
            // NORTH
            Triangle {
                points: 
                    [
                    Point { x: 1.0_f32, y: 0.0_f32, z: 1.0_f32},
                    Point { x: 1.0_f32, y: 1.0_f32, z: 1.0_f32},
                    Point { x: 0.0_f32, y: 1.0_f32, z: 1.0_f32},
                    ]
            },
            Triangle {
                points: 
                    [
                     Point { x: 1.0_f32, y: 0.0_f32, z: 1.0_f32},
                     Point { x: 0.0_f32, y: 1.0_f32, z: 1.0_f32},
                     Point { x: 0.0_f32, y: 0.0_f32, z: 1.0_f32},
                    ]
            },
            // WEST
            Triangle {
                points: 
                    [
                     Point { x: 0.0_f32, y: 0.0_f32, z: 1.0_f32},
                     Point { x: 0.0_f32, y: 1.0_f32, z: 1.0_f32},
                     Point { x: 0.0_f32, y: 1.0_f32, z: 0.0_f32},
                    ]
            },
            Triangle {
                points: 
                    [
                     Point { x: 0.0_f32, y: 0.0_f32, z: 1.0_f32},
                     Point { x: 0.0_f32, y: 1.0_f32, z: 0.0_f32},
                     Point { x: 0.0_f32, y: 0.0_f32, z: 0.0_f32},
                    ]
            },
            // TOP
            Triangle {
                points: 
                    [
                     Point { x: 0.0_f32, y: 1.0_f32, z: 0.0_f32},
                     Point { x: 0.0_f32, y: 1.0_f32, z: 1.0_f32},
                     Point { x: 1.0_f32, y: 1.0_f32, z: 1.0_f32},
                    ]
            },
            Triangle {
                points: 
                    [
                     Point { x: 0.0_f32, y: 1.0_f32, z: 0.0_f32},
                     Point { x: 1.0_f32, y: 1.0_f32, z: 1.0_f32},
                     Point { x: 1.0_f32, y: 1.0_f32, z: 0.0_f32},
                    ]
            },
            // BOTTON
            Triangle {
                points: 
                    [
                     Point { x: 1.0_f32, y: 0.0_f32, z: 1.0_f32},
                     Point { x: 0.0_f32, y: 0.0_f32, z: 1.0_f32},
                     Point { x: 0.0_f32, y: 0.0_f32, z: 0.0_f32},
                    ]
            },
            Triangle {
                points: 
                    [
                     Point { x: 1.0_f32, y: 0.0_f32, z: 1.0_f32},
                     Point { x: 0.0_f32, y: 0.0_f32, z: 0.0_f32},
                     Point { x: 1.0_f32, y: 0.0_f32, z: 0.0_f32},
                    ]
            },
        ],
    };

    // initializes a screen of 20x10 characters with a target of 3 frames per second
    // coordinates will range from [0,0] to [19,9]
    let mut engine = console_engine::ConsoleEngine::init(20, 10, 3).unwrap();
    let value = 14;
    // main loop, be aware that you'll have to break it because ctrl+C is captured
    loop {

        engine.wait_frame(); // wait for next frame + capture inputs
        engine.clear_screen(); // reset the screen
    
      
        draw_triangle(&mut engine, 0, 0, 0,10, 10, 0);
        

        if engine.is_key_pressed(KeyCode::Char('q')) { // if the user presses 'q' :
            break; // exits app
        }
    
        engine.draw(); // draw the screen
    }
}