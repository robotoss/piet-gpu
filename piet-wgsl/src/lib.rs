mod debug;
mod engine;
mod pico_svg;
mod ramp;
mod render;
mod shaders;
mod test_scene;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::{debug::*, engine::*, pico_svg::*, ramp::*, render::*, shaders::*};
}
