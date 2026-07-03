use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct BandwidthStatistic {
    pub name: String,
    pub address: String,
    pub maximum_transmission_unit: String,
    pub upload: String,
    pub upload_rate: u64,
    pub download: String,
    pub download_rate: u64,
    pub total: String,
}

#[derive(Default)]
pub struct TotalBytesTransferredEntry {
    pub total_upload_bytes: u64,
    pub total_download_bytes: u64,
}
