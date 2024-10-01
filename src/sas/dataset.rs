use crate::sas::header::SasHeader;
use crate::sas::metadata::SasMetadata;
use crate::sas::page::SasPage;
use derive_builder::Builder;
use polars::prelude::*;

#[derive(Debug, Builder)]
pub struct SasDataset {
    header: SasHeader,
    pages: Vec<SasPage>,
    data: Option<LazyFrame>,
    metadata: SasMetadata,
}
