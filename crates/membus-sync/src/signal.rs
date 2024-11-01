use strum::EnumIter;

#[repr(u8)]
#[derive(Debug, Default, EnumIter)]
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
