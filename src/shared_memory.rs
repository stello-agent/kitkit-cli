use crate::api_client::authenticated_client;
use crate::cli::{SharedMemoryCommand, SharedMemoryUpsertArgs};
use crate::input::read_text_input;
use crate::output::{Output, Printer, ReadablePrinter, bool_text, kv_table, table_with_header};
use anyhow::Result;
use kitkit_sdk::shared_memory::{
    self as shared_memory_api, ListSharedMemoryResponse, UpsertSharedMemoryRequest,
    UpsertSharedMemoryResponse,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSharedMemoryOutput {
    pub ok: bool,
    pub space_id: String,
    pub slug: String,
}

pub async fn run(command: SharedMemoryCommand, base_url: &str) -> Result<Output> {
    let client = authenticated_client(base_url).await?;
    match command {
        SharedMemoryCommand::List { space_id } => Ok(Output::SharedMemoryList(
            shared_memory_api::list(&client, &space_id).await?,
        )),
        SharedMemoryCommand::Upsert(args) => {
            let input = read_shared_memory_body(args)?;
            Ok(Output::SharedMemoryUpsert(
                shared_memory_api::upsert(
                    &client,
                    &input.space_id,
                    UpsertSharedMemoryRequest {
                        slug: input.slug,
                        body: input.body,
                    },
                )
                .await?,
            ))
        }
        SharedMemoryCommand::Delete { space_id, slug } => {
            shared_memory_api::delete(&client, &space_id, &slug).await?;
            Ok(Output::SharedMemoryDelete(DeleteSharedMemoryOutput {
                ok: true,
                space_id,
                slug,
            }))
        }
    }
}

impl Printer for ReadablePrinter<ListSharedMemoryResponse> {
    fn print_output(self) -> Result<()> {
        let mut table = table_with_header(["slug", "body"]);
        for entry in &self.inner.entries {
            table.add_row([entry.slug.as_str(), entry.body.as_str()]);
        }
        println!("{table}");
        Ok(())
    }
}

impl Printer for ReadablePrinter<UpsertSharedMemoryResponse> {
    fn print_output(self) -> Result<()> {
        let out = &self.inner;
        let mut table = kv_table();
        table.add_row(["slug", out.entry.slug.as_str()]);
        table.add_row(["body", out.entry.body.as_str()]);
        println!("{table}");
        Ok(())
    }
}

impl Printer for ReadablePrinter<DeleteSharedMemoryOutput> {
    fn print_output(self) -> Result<()> {
        let out = &self.inner;
        let mut table = kv_table();
        table.add_row(["ok", bool_text(out.ok)]);
        table.add_row(["space_id", out.space_id.as_str()]);
        table.add_row(["slug", out.slug.as_str()]);
        println!("{table}");
        Ok(())
    }
}

#[derive(Debug)]
struct SharedMemoryInput {
    space_id: String,
    slug: String,
    body: String,
}

fn read_shared_memory_body(args: SharedMemoryUpsertArgs) -> Result<SharedMemoryInput> {
    Ok(SharedMemoryInput {
        space_id: args.space_id,
        slug: args.slug,
        body: read_text_input(
            args.body,
            args.body_file,
            args.stdin,
            "--body, --body-file, or --stdin",
        )?,
    })
}
