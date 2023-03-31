use crate::xhc::device_manager::device::Device;

/// デバイスの初期化フェーズを表わします。
#[derive(Debug)]
pub enum InitializePhase {
    NotPrepared,
    Phase1,
    Phase2,
    Phase3,
    Completed,
}
