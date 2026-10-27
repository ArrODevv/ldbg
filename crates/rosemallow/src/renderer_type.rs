use std::fmt::Display;

#[non_exhaustive]
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, Default)]
pub enum RendererType {
    OpenGL,
    #[default]
    Vulkan,
    // More?
}

impl RendererType {
    pub fn as_str(&self) -> &'static str {
        match self {
            RendererType::OpenGL => "OpenGL",
            RendererType::Vulkan => "Vulkan",
        }
    }
}

impl Display for RendererType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
