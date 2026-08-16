#![allow(dead_code)]

use std::time::{Instant, Duration};
use std::ffi::CString;
use std::cmp;
use glfw::Context;
use rand::RngExt;

mod mygl;
use mygl::*;

mod mat;
use mat::*;

mod camera;
use camera::*;

// game parameters
const FPS: u32 = 60;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const CELL_WIDTH_RATIO: f32 = 0.03125; // fraction of the horizontal size
const CELL_HEIGHT_RATIO: f32 = (WIDTH as f32 * CELL_WIDTH_RATIO)/HEIGHT as f32; // fraction of the vertical size
const CELL_WIDTH: f32 = WIDTH as f32 * CELL_WIDTH_RATIO;
const CELL_HEIGHT: f32 = HEIGHT as f32 * CELL_HEIGHT_RATIO;

// parameters for the projection matrix
const FOV_Y: f32 = std::f32::consts::FRAC_PI_4;
const ASPECT: f32 = WIDTH as f32 / HEIGHT as f32;
const NEAR: f32 = 0.1;
const FAR: f32 = 2000.0;

// objects
// border
const BORDER_WIDTH: f32 = HEIGHT as f32 * 0.05;
const BORDER_VERTICES: [[f32; 3]; 12] =
  [[-BORDER_WIDTH, -BORDER_WIDTH, 0.0], [WIDTH as f32 + BORDER_WIDTH, -BORDER_WIDTH, 0.0], [-BORDER_WIDTH, 0.0, 0.0],
  [0.0, 0.0, 0.0], [WIDTH as f32, 0.0, 0.0], [WIDTH as f32 + BORDER_WIDTH, 0.0, 0.0], [-BORDER_WIDTH, HEIGHT as f32, 0.0],
  [0.0, HEIGHT as f32, 0.0], [WIDTH as f32,  HEIGHT as f32, 0.0], [WIDTH as f32 + BORDER_WIDTH,  HEIGHT as f32, 0.0],
  [-BORDER_WIDTH, HEIGHT as f32 + BORDER_WIDTH, 0.0], [WIDTH as f32 + BORDER_WIDTH, HEIGHT as f32 + BORDER_WIDTH, 0.0]];

const BORDER_INDICES: [[u32; 3]; 8] =
  [[0, 1, 2], [2, 1, 5], [2, 3, 6], [6, 3, 7],
  [4, 5, 8], [8, 5, 9], [11, 10, 6], [11, 6, 9]];

// simple square, used for the snake and apple
const SQUARE_VERTICES: [[f32; 3]; 8] = [
    // back face (z = 0)
    [0.0,          0.0,           0.0],  // 0
    [CELL_WIDTH,   0.0,           0.0],  // 1
    [CELL_WIDTH,   CELL_HEIGHT,   0.0],  // 2
    [0.0,          CELL_HEIGHT,   0.0],  // 3
    // front face (z = CELL_DEPTH)
    [0.0,          0.0,           CELL_WIDTH],  // 4
    [CELL_WIDTH,   0.0,           CELL_WIDTH],  // 5
    [CELL_WIDTH,   CELL_HEIGHT,   CELL_WIDTH],  // 6
    [0.0,          CELL_HEIGHT,   CELL_WIDTH],  // 7
];

const SQUARE_INDICES: [[u32; 3]; 12] = [
    // back face (z = 0), matches original winding
    [0, 3, 1], [1, 3, 2],
    // front face (z = CELL_DEPTH), reversed winding (facing +z instead of -z)
    [4, 5, 7], [5, 6, 7],
    // bottom face (y = 0)
    [0, 1, 4], [1, 5, 4],
    // top face (y = CELL_HEIGHT)
    [3, 7, 2], [2, 7, 6],
    // left face (x = 0)
    [0, 4, 3], [3, 4, 7],
    // right face (x = CELL_WIDTH)
    [1, 2, 5], [2, 6, 5],
];

// simple shaders
const VERT_SHADER: &str = r#"#version 330 core
    uniform mat4 mvp;

    layout(location = 0) in vec3 position;

    void main() {
        gl_Position = mvp * vec4(position, 1.0);
    }
"#;

const FRAG_SHADER: &str = r#"#version 330 core
    uniform vec3 vColor;

    out vec4 final_color;

    void main() {
        final_color = vec4(vColor, 1.0);
    }
"#;

struct GLFWEnv {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>
}

impl GLFWEnv {
    // create a new SDL environment that contains a window with an opengl context
    pub fn new(window_name: &str, width: u32, height: u32) -> Self {
        use glfw::fail_on_errors;
        use glfw::Context;

        let mut glfw = glfw::init(fail_on_errors!()).expect("GLFW failed on init.");

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(glfw::WindowHint::Resizable(false));

        let (mut window, events) = glfw.create_window(width, height, window_name, glfw::WindowMode::Windowed).expect("GLFW: Failed on window creation.");

        // get the actual screen resulution size
        let (screen_width, screen_height) = window.get_framebuffer_size();

        window.make_current();
        window.set_key_polling(true);
        gl::load_with(|s| window.get_proc_address(s).expect(&format!("get_proc_address() could not find function {}", s)) as *const _);

        unsafe {
            gl::Viewport(0, 0, screen_width, screen_height);
        }

        Self {
            glfw,
            window,
            events
        }
    }
}

struct Game {
    program: Program,
    square: Object,
    border: Object,
    snake: [[u32; 2]; 576],
    snake_size: usize,
    apple: [u32; 2],
    score: u64,
}

fn init_game() -> Game {
    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        
        // create the vertex array object and make it active
        let mut square = Object::new();
        square.bind();
        square.buffer_data(gl::ARRAY_BUFFER, bytemuck::cast_slice(&SQUARE_VERTICES), gl::STATIC_DRAW);
        square.buffer_data(gl::ELEMENT_ARRAY_BUFFER, bytemuck::cast_slice(&SQUARE_INDICES), gl::STATIC_DRAW);
        Object::vertex_attribute_simple(3,gl::FLOAT);

        let mut border = Object::new();
        border.bind();
        border.buffer_data(gl::ARRAY_BUFFER, bytemuck::cast_slice(&BORDER_VERTICES), gl::STATIC_DRAW);
        border.buffer_data(gl::ELEMENT_ARRAY_BUFFER, bytemuck::cast_slice(&BORDER_INDICES), gl::STATIC_DRAW);
        Object::vertex_attribute_simple(3,gl::FLOAT);

        // create the program that contains the shaders and check for success
        let program = Program::from_vert_frag(VERT_SHADER, FRAG_SHADER);
        program.use_program();

        let mut snake: [[u32; 2]; 576] = [[0, 0]; 576];
        snake[0] = [0, 0];
        snake[1] = [1, 0];
        snake[2] = [2, 0];

        Game {
            program,
            square,
            border,
            snake,
            snake_size: 3,
            apple: [5, 5],
            score: 0
        }
    }
}

pub fn main() {
    let mut env = GLFWEnv::new("teste", WIDTH, HEIGHT);

    let mut game = init_game();
    let mut rng = rand::rng();

    let eye = Vec4::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0, 870.0, 0.0);
    let target = Vec4::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0, 0.0, 0.0);
    let up = Vec4::new(0.0, 1.0,0.0 , 0.0);
    let mut camera = Camera::new(&eye, &target, &up);

    let mut current_direction = glfw::Key::Up;
    let mut previous_direction = glfw::Key::Up;
    let speed: u64 = FPS as u64/4;
    let max_speed: u64 = FPS as u64/8;
    let mut frame_count: u64 = 0;
    let npf: u128 = 1_000_000_000u128 / FPS as u128;

    let mvp = unsafe { gl::GetUniformLocation(game.program.get_handle(), CString::new("mvp").unwrap().as_ptr()) };
    let color = unsafe { gl::GetUniformLocation(game.program.get_handle(), CString::new("vColor").unwrap().as_ptr()) };

    // no support for resizing/zooming, so only compute this once
    let proj_m = projection_matrix(FOV_Y, ASPECT, NEAR, FAR);

    // main loop
    while !env.window.should_close() {
        use glfw::WindowEvent as Event;
        use glfw::Key;
        use glfw::Action;

        let start = Instant::now();

        env.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&env.events) {
            match event {
                Event::Close | Event::Key(Key::Escape, _, Action::Press, _) => {
                    env.window.set_should_close(true);
                },
                Event::Key(key @ (Key::Up | Key::Left | Key::Down | Key::Right), _, Action::Press | Action::Repeat, _)  => {
                    current_direction = key;
                },
                Event::Key(key @ (Key::W | Key::A | Key::S | Key::D | Key::Q | Key::E | Key::I | Key::J | Key::K | Key:: L | Key::O | Key::U), _, Action::Press | Action::Repeat, _)  => {
                    match key {
                        Key::W => { camera.move_vertical(12.0); },
                        Key::A => { camera.move_horizontal(-12.0); },
                        Key::S => { camera.move_vertical(-12.0); },
                        Key::D => { camera.move_horizontal(12.0); },
                        Key::Q => { camera.move_depth(-12.0); },
                        Key::E => { camera.move_depth(12.0); },
                        Key::I => { camera.rotate_vertical(std::f32::consts::FRAC_1_PI/15.0); },
                        Key::J => { camera.rotate_horizontal(std::f32::consts::FRAC_1_PI/15.0); },
                        Key::K => { camera.rotate_vertical(-std::f32::consts::FRAC_1_PI/15.0); },
                        Key::L => { camera.rotate_horizontal(-std::f32::consts::FRAC_1_PI/15.0); },
                        Key::U => { camera.rotate_roll(std::f32::consts::FRAC_1_PI/15.0); },
                        Key::O => { camera.rotate_roll(-std::f32::consts::FRAC_1_PI/15.0); },
                        _ => unreachable!(),
                    }
                }
                _ => {}
            }
        }
        
        // update the snake if needed
        if frame_count%cmp::max(speed - game.score/4, max_speed) == 0 {
            for i in (1..game.snake_size).rev() {
                game.snake[i] = game.snake[i - 1];
            }

            // reject inversion of the direction
            let dir_sum = current_direction.get_scancode().unwrap() + previous_direction.get_scancode().unwrap();
            if dir_sum == glfw::Key::Up.get_scancode().unwrap() + glfw::Key::Down.get_scancode().unwrap()
            || dir_sum == glfw::Key::Left.get_scancode().unwrap() + glfw::Key::Right.get_scancode().unwrap() {
                current_direction = previous_direction;
            }

            match current_direction {
                Key::Up => {
                    game.snake[0][1] = if game.snake[0][1] == HEIGHT/CELL_HEIGHT as u32 - 1 {0} else {game.snake[0][1] + 1};
                },
                Key::Left => {
                    game.snake[0][0] = if game.snake[0][0] == 0 {WIDTH/CELL_WIDTH as u32 - 1} else {game.snake[0][0] - 1};
                },
                Key::Down => {
                    game.snake[0][1] = if game.snake[0][1] == 0 {HEIGHT/CELL_HEIGHT as u32 - 1 } else {game.snake[0][1] - 1};
                },
                Key::Right => {
                    game.snake[0][0] = if game.snake[0][0] == WIDTH/CELL_WIDTH as u32 - 1 {0} else {game.snake[0][0] + 1};
                },
                _ => {}
            }

            // check for collisions with itself
            for i in 1..game.snake_size {
                if game.snake[0] == game.snake[i] {
                    env.window.set_should_close(true);
                }
            }

            if game.snake[0] == game.apple {
                game.snake[game.snake_size] = game.snake[game.snake_size - 1];
                game.snake_size += 1;
                game.score += 1;

                loop {
                    game.apple[0] = rng.random_range(0..32);
                    game.apple[1] = rng.random_range(0..18);

                    let mut valid = true;
                    for i in 0..game.snake_size {
                        if game.snake[i] == game.apple {
                            valid = false;
                            break;
                        }
                    }

                    if valid {
                        break;
                    }
                }
            }

            previous_direction = current_direction;
        }

        let vp = proj_m * *camera.get_view_matrix();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // draw the border first
            game.border.bind();
            gl::Uniform3f(color, 0.0, 0.0, 1.0);
            gl::UniformMatrix4fv(mvp, 1, gl::FALSE, vp.as_ptr());
            gl::DrawElements(gl::TRIANGLES, 24, gl::UNSIGNED_INT, 0 as *const _);

            game.square.bind();

            // draw the apple
            let model_matrix = Mat4::identity().scale_by(0.5, 0.5, 0.5).translate_by(game.apple[0] as f32 * CELL_WIDTH + CELL_WIDTH/4.0, game.apple[1] as f32 * CELL_HEIGHT + CELL_HEIGHT/4.0, 0.0);
            gl::Uniform3f(color, 1.0, 0.0,0.0);
            gl::UniformMatrix4fv(mvp, 1, gl::FALSE, (vp * model_matrix).as_ptr());
            gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, 0 as *const _);

            // draw the snake
            gl::Uniform3f(color, 0.2, 0.83, 0.36);
            for i in 0..game.snake_size {
                let model_matrix = Mat4::identity().translate_by(game.snake[i][0] as f32 * CELL_WIDTH, game.snake[i][1] as f32 * CELL_HEIGHT, 0.0);
                gl::UniformMatrix4fv(mvp, 1, gl::FALSE, (vp * model_matrix).as_ptr());
                gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, 0 as *const _);
            }
        }

        env.window.swap_buffers();

        let elapsed = start.elapsed().as_nanos();
        if elapsed < npf {
            std::thread::sleep(Duration::new(0, (npf - elapsed) as u32));
        }

        frame_count += 1;
    }
}
