#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerListenEffect {
    CreateCachedWorkerPools,
    InstallPipelineStage { name: &'static str },
    BindAddress { address: String },
}
