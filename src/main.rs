extern crate render_engine;
use render_engine as re;

use re::app::App;
use re::exposed_tools::*;

fn main() {
    let mut app = App::new();

    let verts = [
        Vertex {
            position: [0.0, -1.0],
            color: [1.0, 0.0, 0.0, 1.0],
        },
        Vertex {
            position: [-1.0, 1.0],
            color: [0.0, 1.0, 0.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0],
            color: [0.0, 0.0, 1.0, 1.0],
        },
    ];

    app.new_vbuf_from_verts(
        &[
            verts[0].clone(),
            verts[1].clone(),
            verts[2].clone(),
        ]);

    loop {
        app.draw_frame();

        if app.done {
            break;
        }
    }
}
