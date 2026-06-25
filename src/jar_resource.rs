use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JarResource {
    path: PathBuf,
}

impl JarResource {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn jar_url(&self) -> String {
        format!("jar:{}!/", file_url(self.path()))
    }

    pub fn is_running_from_packaged_resource(resource: Option<&str>) -> bool {
        resource.is_some_and(|value| value.ends_with("plugin.yml"))
    }
}

fn file_url(path: &Path) -> String {
    let mut value = path.to_string_lossy().replace('\\', "/");
    if !value.starts_with('/') {
        value = format!("/{value}");
    }

    format!("file://{value}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_jar_url_like_java_helper() {
        let resource = JarResource::new("/opt/roseau/server.jar");

        assert_eq!(resource.jar_url(), "jar:file:///opt/roseau/server.jar!/");
    }

    #[test]
    fn detects_packaged_resource_marker() {
        assert!(JarResource::is_running_from_packaged_resource(Some(
            "jar:file:/server.jar!/plugin.yml"
        )));
        assert!(!JarResource::is_running_from_packaged_resource(None));
        assert!(!JarResource::is_running_from_packaged_resource(Some(
            "roseau.properties"
        )));
    }
}
