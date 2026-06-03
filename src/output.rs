use crate::auth::{AuthStatusOutput, OkOutput};
use crate::shared_memory::DeleteSharedMemoryOutput;
use anyhow::Result;
use comfy_table::{Table, presets::ASCII_FULL_CONDENSED};
use kitkit_sdk::sessions::{GetSessionDigestResponse, PutInsightResponse, SessionStatus};
use kitkit_sdk::shared_memory::{ListSharedMemoryResponse, UpsertSharedMemoryResponse};
use kitkit_sdk::spaces::{ForkSessionResponse, ListSpacesResponse, SessionTreeNode, Space};
use serde::Serialize;

pub trait Printer {
    fn print_output(self) -> Result<()>;
}

pub struct JsonPrinter<T> {
    inner: T,
}

impl<T> JsonPrinter<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: Serialize> Printer for JsonPrinter<T> {
    fn print_output(self) -> Result<()> {
        println!("{}", serde_json::to_string_pretty(&self.inner)?);
        Ok(())
    }
}

pub struct ReadablePrinter<T> {
    pub(crate) inner: T,
}

impl<T> ReadablePrinter<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

pub fn print_output<T>(out: T, is_json: bool) -> Result<()>
where
    T: Serialize,
    ReadablePrinter<T>: Printer,
{
    if is_json {
        JsonPrinter::new(out).print_output()
    } else {
        ReadablePrinter::new(out).print_output()
    }
}

#[derive(Debug)]
pub enum Output {
    AuthStatus(AuthStatusOutput),
    Ok(OkOutput),
    SpacesList(ListSpacesResponse),
    Space(Space),
    Topology(SessionTreeNode),
    Digest(GetSessionDigestResponse),
    InsightPut(PutInsightResponse),
    SharedMemoryList(ListSharedMemoryResponse),
    SharedMemoryUpsert(UpsertSharedMemoryResponse),
    SharedMemoryDelete(DeleteSharedMemoryOutput),
    Fork(ForkSessionResponse),
}

impl Output {
    pub fn print_output(self, is_json: bool) -> Result<()> {
        match self {
            Output::AuthStatus(out) => print_output(out, is_json),
            Output::Ok(out) => print_output(out, is_json),
            Output::SpacesList(out) => print_output(out, is_json),
            Output::Space(out) => print_output(out, is_json),
            Output::Topology(out) => print_output(out, is_json),
            Output::Digest(out) => print_output(out, is_json),
            Output::InsightPut(out) => print_output(out, is_json),
            Output::SharedMemoryList(out) => print_output(out, is_json),
            Output::SharedMemoryUpsert(out) => print_output(out, is_json),
            Output::SharedMemoryDelete(out) => print_output(out, is_json),
            Output::Fork(out) => print_output(out, is_json),
        }
    }
}

pub(crate) fn table_with_header<const N: usize>(headers: [&str; N]) -> Table {
    let mut table = Table::new();
    table.load_preset(ASCII_FULL_CONDENSED);
    table.set_header(headers);
    table
}

pub(crate) fn kv_table() -> Table {
    table_with_header(["field", "value"])
}

pub(crate) fn bool_text(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}

pub(crate) fn empty_text(value: &str) -> &str {
    if value.trim().is_empty() { "" } else { value }
}

pub(crate) fn status_text(status: SessionStatus) -> &'static str {
    match status {
        SessionStatus::Active => "active",
        SessionStatus::Archived => "archived",
    }
}
