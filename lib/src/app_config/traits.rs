/// Upgrader trait for handling data version upgrades between config file versions.
pub trait Upgrader: Sync {
    /// Upgrades the given data from an old version to a new version.
    fn upgrade(&self, data: &str) -> String;

    /// Returns the version that this upgrader creates.
    fn version(&self) -> &str;

    /// Returns if the upgrader can handle the given version.
    fn can_upgrade(&self, version: &str) -> bool;

    /// Returns a message indicating the breaking change, if applicable.
    fn breaking_change_message(&self) -> Option<String>;
}
