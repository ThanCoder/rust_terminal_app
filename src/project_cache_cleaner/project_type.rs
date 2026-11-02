#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ProjectType {
    Flutter,
    Rust,
    // Nodejs,
    // Go,
    // Python,
    // Zig,
    // CppVcpkg,
    // CppCmake,
    // Dotnet,
    // GoCache,
    // LinuxCache,
    // GradleCaches,
}

impl ProjectType {
    pub fn as_string(&self) -> &'static str {
        match self {
            ProjectType::Flutter => "Flutter",
            ProjectType::Rust => "Rust",
        }
    }
}
