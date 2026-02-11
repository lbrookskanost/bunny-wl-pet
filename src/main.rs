mod wayland;
mod types;
mod render;
mod animation;
mod input;

fn main() {
    wayland::run();
}
