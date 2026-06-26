pub trait HostResolver {
    fn resolve_host(&self, host: &str) -> Result<String, String>;
}
