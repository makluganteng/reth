/// Stage IDs for all known stages.
///
/// For custom stages, use [`StageId::Other`]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StageId {
    /// Static File stage in the process.
    #[deprecated(
        note = "Static Files are generated outside of the pipeline and do not require a separate stage"
    )]
    StaticFile,
    /// Header stage in the process.
    Headers,
    /// Bodies stage in the process.
    Bodies,
    /// Sender recovery stage in the process.
    SenderRecovery,
    /// Execution stage in the process.
    Execution,
    /// Merkle unwind stage in the process.
    MerkleUnwind,
    /// Account hashing stage in the process.
    AccountHashing,
    /// Storage hashing stage in the process.
    StorageHashing,
    /// Merkle execute stage in the process.
    MerkleExecute,
    /// Transaction lookup stage in the process.
    TransactionLookup,
    /// Index storage history stage in the process.
    IndexStorageHistory,
    /// Index account history stage in the process.
    IndexAccountHistory,
    /// Finish stage in the process.
    Finish,
    /// Other custom stage with a provided string identifier.
    Other(&'static str),
}

impl StageId {
    /// All supported Stages
    pub const ALL: [StageId; 12] = [
        StageId::Headers,
        StageId::Bodies,
        StageId::SenderRecovery,
        StageId::Execution,
        StageId::MerkleUnwind,
        StageId::AccountHashing,
        StageId::StorageHashing,
        StageId::MerkleExecute,
        StageId::TransactionLookup,
        StageId::IndexStorageHistory,
        StageId::IndexAccountHistory,
        StageId::Finish,
    ];

    /// Stages that require state.
    pub const STATE_REQUIRED: [StageId; 7] = [
        StageId::Execution,
        StageId::MerkleUnwind,
        StageId::AccountHashing,
        StageId::StorageHashing,
        StageId::MerkleExecute,
        StageId::IndexStorageHistory,
        StageId::IndexAccountHistory,
    ];

    /// Return stage id formatted as string.
    pub fn as_str(&self) -> &str {
        match self {
            #[allow(deprecated)]
            StageId::StaticFile => "StaticFile",
            StageId::Headers => "Headers",
            StageId::Bodies => "Bodies",
            StageId::SenderRecovery => "SenderRecovery",
            StageId::Execution => "Execution",
            StageId::MerkleUnwind => "MerkleUnwind",
            StageId::AccountHashing => "AccountHashing",
            StageId::StorageHashing => "StorageHashing",
            StageId::MerkleExecute => "MerkleExecute",
            StageId::TransactionLookup => "TransactionLookup",
            StageId::IndexAccountHistory => "IndexAccountHistory",
            StageId::IndexStorageHistory => "IndexStorageHistory",
            StageId::Finish => "Finish",
            StageId::Other(s) => s,
        }
    }

    /// Returns true if it's a downloading stage [StageId::Headers] or [StageId::Bodies]
    pub fn is_downloading_stage(&self) -> bool {
        matches!(self, StageId::Headers | StageId::Bodies)
    }

    /// Returns `true` if it's [TransactionLookup](StageId::TransactionLookup) stage.
    pub fn is_tx_lookup(&self) -> bool {
        matches!(self, StageId::TransactionLookup)
    }

    /// Returns true indicating if it's the finish stage [StageId::Finish]
    pub fn is_finish(&self) -> bool {
        matches!(self, StageId::Finish)
    }
}

impl std::fmt::Display for StageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stage_id_as_string() {
        assert_eq!(StageId::Headers.to_string(), "Headers");
        assert_eq!(StageId::Bodies.to_string(), "Bodies");
        assert_eq!(StageId::SenderRecovery.to_string(), "SenderRecovery");
        assert_eq!(StageId::Execution.to_string(), "Execution");
        assert_eq!(StageId::MerkleUnwind.to_string(), "MerkleUnwind");
        assert_eq!(StageId::AccountHashing.to_string(), "AccountHashing");
        assert_eq!(StageId::StorageHashing.to_string(), "StorageHashing");
        assert_eq!(StageId::MerkleExecute.to_string(), "MerkleExecute");
        assert_eq!(StageId::IndexAccountHistory.to_string(), "IndexAccountHistory");
        assert_eq!(StageId::IndexStorageHistory.to_string(), "IndexStorageHistory");
        assert_eq!(StageId::TransactionLookup.to_string(), "TransactionLookup");
        assert_eq!(StageId::Finish.to_string(), "Finish");

        assert_eq!(StageId::Other("Foo").to_string(), "Foo");
    }

    #[test]
    fn is_downloading_stage() {
        assert!(StageId::Headers.is_downloading_stage());
        assert!(StageId::Bodies.is_downloading_stage());

        assert!(!StageId::Execution.is_downloading_stage());
    }

    // Multiple places around the codebase assume headers is the first stage.
    // Feel free to remove this test if the assumption changes.
    #[test]
    fn stage_all_headers_first() {
        assert_eq!(*StageId::ALL.first().unwrap(), StageId::Headers);
    }
}
