use crate::api_client::authenticated_client;
use crate::cli::{InsightCommand, InsightPutArgs};
use crate::input::read_text_input;
use crate::output::{Output, Printer, ReadablePrinter, bool_text, kv_table, status_text};
use anyhow::Result;
use kitkit_sdk::sessions::{
    self as sessions_api, GetSessionDigestResponse, PutInsightRequest, PutInsightResponse,
};

pub async fn run_digest(space_id: String, session_id: String, base_url: &str) -> Result<Output> {
    let client = authenticated_client(base_url).await?;
    Ok(Output::Digest(
        sessions_api::digest(&client, &space_id, &session_id).await?,
    ))
}

pub async fn run_insight(command: InsightCommand, base_url: &str) -> Result<Output> {
    let client = authenticated_client(base_url).await?;
    match command {
        InsightCommand::Put(args) => {
            let input = read_insight_content(args)?;
            Ok(Output::InsightPut(
                sessions_api::put_insight(
                    &client,
                    &input.space_id,
                    &input.session_id,
                    PutInsightRequest {
                        content: input.content,
                    },
                )
                .await?,
            ))
        }
    }
}

impl Printer for ReadablePrinter<GetSessionDigestResponse> {
    fn print_output(self) -> Result<()> {
        let digest = &self.inner.digest;
        let mut table = kv_table();
        table.add_row(["id", digest.id.as_str()]);
        table.add_row(["label", digest.label.as_str()]);
        table.add_row(["status", status_text(digest.status)]);
        table.add_row(["memory", digest.memory.as_deref().unwrap_or("")]);
        table.add_row(["insight", digest.insight.as_deref().unwrap_or("")]);
        println!("{table}");
        Ok(())
    }
}

impl Printer for ReadablePrinter<PutInsightResponse> {
    fn print_output(self) -> Result<()> {
        let out = &self.inner;
        let mut table = kv_table();
        table.add_row(["ok", bool_text(out.ok)]);
        table.add_row(["session_id", out.session_id.as_str()]);
        table.add_row(["label", out.label.as_str()]);
        table.add_row(["content_length", &out.content_length.to_string()]);
        println!("{table}");
        Ok(())
    }
}

#[derive(Debug)]
struct InsightInput {
    space_id: String,
    session_id: String,
    content: String,
}

fn read_insight_content(args: InsightPutArgs) -> Result<InsightInput> {
    Ok(InsightInput {
        space_id: args.space_id,
        session_id: args.session_id,
        content: read_text_input(
            args.content,
            args.content_file,
            args.stdin,
            "--content, --content-file, or --stdin",
        )?,
    })
}
