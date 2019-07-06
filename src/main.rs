extern crate render_engine;
use render_engine as re;

use re::app::App;
use re::exposed_tools::*;

fn main() {
    let mut app = App::new();
    app.enable_multisampling();

    let verts = [
        Vertex { // 0
            position: [-1.0, -1.0, -1.0],
            color: [1.0, 0.0, 0.0, 1.0],
        },
        Vertex { // 1
            position: [1.0, -1.0, -1.0],
            color: [0.0, 1.0, 0.0, 1.0],
        },
        Vertex { // 2
            position: [-1.0, 1.0, -1.0],
            color: [0.0, 0.0, 1.0, 1.0],
        },
        Vertex { // 3
            position: [-1.0, -1.0, 1.0],
            color: [0.0, 1.0, 1.0, 1.0],
        },
        Vertex { // 4
            position: [-1.0, 1.0, 1.0],
            color: [1.0, 0.0, 1.0, 1.0],
        },
        Vertex { // 5
            position: [1.0, -1.0, 1.0],
            color: [0.0, 1.0, 1.0, 1.0],
        },
        Vertex { // 6
            position: [1.0, 1.0, -1.0],
            color: [1.0, 1.0, 1.0, 1.0],
        },
        Vertex { // 7
            position: [1.0, 1.0, 1.0],
            color: [0.0, 0.0, 0.0, 1.0],
        },
    ];

    let indices: [usize; 36] = [0, 3, 2, 2, 3, 4, 0, 2, 1, 1, 6, 2, 1, 6, 5, 6, 7, 5, 0, 1, 3, 3, 1, 5, 2, 4, 6, 4, 6, 7, 3, 4, 5, 4, 7, 5];

    app.new_vbuf_from_verts(&indices.iter().map(|&idx| verts[idx].clone()).collect::<Vec<_>>());

    loop {
        app.draw_frame();

        if app.done {
            break;
        }
    }

    app.print_fps();
}
