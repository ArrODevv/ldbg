use rosemallow::renderer_type::RendererType;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, Default)]
pub struct Config {
    renderer_type: RendererType,
}
