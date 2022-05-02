mod data;
mod qr;

use anyhow::Result;
use log::info;

use crate::verifier::data::verify_data_json;
use crate::verifier::qr::validate_signed_qrs;
use crate::AppConfig;

pub(crate) fn verify(config: AppConfig) -> Result<()> {
    verify_data_json(&config)?;
    info!("🎉 Data file is verified");
    validate_signed_qrs(&config.qr_dir, &config.verifier.public_key)?;
    info!("✅ Done");
    Ok(())
}
