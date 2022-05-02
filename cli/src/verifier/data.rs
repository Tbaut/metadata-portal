use crate::collector::export::export_specs;
use crate::export::read_export_file;
use crate::fetch::RpcFetcher;
use crate::AppConfig;
use anyhow::{ensure, Result};


pub(crate) fn verify_data_json(config: &AppConfig) -> Result<()> {
    let current = read_export_file(config)?;
    let expected = export_specs(config, RpcFetcher)?;
    ensure!(
        current == expected,
        "Exported data does not match the expected data"
    );
    Ok(())
}
