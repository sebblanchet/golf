use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderDevice;
use bevy::render::RenderApp;

pub struct Gpu;

impl Plugin for Gpu {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) {
        let Some(sapp) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        let dev = sapp.world().resource::<RenderDevice>();

        //dev.configure_surface(surface, config);

        // check if the device support the required feature. if not, exit the example.
        // in a real application, you should setup a fallback for the missing feature
        if !dev
            .features()
            .contains(WgpuFeatures::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING)
        {
            error!("Render device doesn't support feature");
        }
    }
}
