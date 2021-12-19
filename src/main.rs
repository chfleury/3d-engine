use console_engine::pixel;
use console_engine::Color;
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
    
        engine.line(0, 0, 19, 9, pixel::pxl('#')); // draw a line of '#' from [0,0] to [19,9]
        engine.print(0, 4, format!("Result: {}", value).as_str()); // prints some value at [0,4]
    
        engine.set_pxl(4, 0, pixel::pxl_fg('O', Color::Cyan)); // write a majestic cyan 'O' at [4,0]

        if engine.is_key_pressed(KeyCode::Char('q')) { // if the user presses 'q' :
            break; // exits app
        }
    
        engine.draw(); // draw the screen
    }
}