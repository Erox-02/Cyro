use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Model {
    General,
    Anime,
    AnimeVideo,
    RealEsrNet,
}

impl Default for Model {
    fn default() -> Self {
        Self::General
    }
}

impl Model {
    pub fn cli_name(self) -> &'static str {
        match self {
            Self::General => "realesrgan-x4plus",
            Self::Anime => "realesrgan-x4plus-anime",
            Self::AnimeVideo => "realesr-animevideov3",
            Self::RealEsrNet => "realesrnet-x4plus",
        }
    }
}

pub fn upscale(
    input: &Path,
    output: &Path,
    scale: u8,
    model: Model,
) -> Result<PathBuf, String> {
    if !input.is_file() {
        return Err(format!("input file unfound => {}", input.display()));
    }

    if !matches!(scale, 2 | 3 | 4) {
        return Err("scale must be 2, 3, or 4.".into());
    }

    if output.exists() {
        return Err(format!(
            "Output already exists: {}",
            output.display()
        ));
    }

    let result = Command::new("realesrgan-ncnn-vulkan")
        .arg("-i")
        .arg(input)
        .arg("-o")
        .arg(output)
        .arg("-s")
        .arg(scale.to_string())
        .arg("-n")
        .arg(model.cli_name())
        .arg("-f")
        .arg("png")
        .output()
        .map_err(|err| {
            if err.kind() == std::io::ErrorKind::NotFound {
                "realesrgan-ncnn-vulkan was not found in PATH.".to_string()
            } else {
                format!("failed to start realesrgan ++=> {err}")
            }
        })?;

    if !result.status.success() {
        let stderr = String::from_utf8_lossy(&result.stderr);
        let stdout = String::from_utf8_lossy(&result.stdout);

        let details = if stderr.trim().is_empty() {
            stdout.trim()
        } else {
            stderr.trim()
        };

        return Err(format!(
            "realesrgan failed ({}) => {}",
            result.status, details
        ));
    }

    if !output.is_file() {
        return Err("realesrgan exited successfully but produced no output.".into());
    }

    Ok(output.to_path_buf())
}
