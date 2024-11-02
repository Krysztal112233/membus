use strum::{EnumIter, FromRepr};

#[repr(u8)]
#[derive(Debug, Default, Clone, EnumIter, FromRepr, PartialEq, Eq, PartialOrd, Ord)]
pub enum SyncSignal {
    /// Sending data package
    #[default]
    Sending,

    /// Completed sending data package
    Completed,

    /// Pulling data package
    Pulling,

    /// Pulled data package completed
    Pulled,
}
