use piston_window::*;
use piston_window;
use find_folder;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("piston: image", [300, 300])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();

    let red_king_image = assets.join("red_king.png");
    let red_king_image: G2dTexture = Texture::from_path(
            &mut window.create_texture_context(),
            &red_king_image,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();
    window.set_lazy(true);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);
            image(&red_king_image, c.transform, g);
        });
    }
}