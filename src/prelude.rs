#[cfg(feature = "amazon-s3")]
pub use crate::backends::amazon_s3::AmazonS3Storage;
#[cfg(feature = "google-drive")]
pub use crate::backends::google_drive::GoogleDriveStorage;
#[cfg(feature = "localdisk")]
pub use crate::backends::localdisk::LocalDiskStorage;
pub use crate::ObjectMetadata;
pub use crate::Storage;
