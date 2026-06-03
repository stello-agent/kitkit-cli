use crate::api_client::{anonymous_client, refresh_auth_if_needed};
use crate::auth_store::{StoredAuth, StoredUser, auth_path, load_auth, remove_auth, save_auth};
use crate::cli::AuthCommand;
use crate::input::{prompt_line, read_stdin};
use crate::output::{Output, Printer, ReadablePrinter, bool_text, kv_table};
use anyhow::Result;
use kitkit_sdk::auth::{self as auth_api, LoginRequest};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthStatusOutput {
    pub authenticated: bool,
    pub base_url: String,
    pub token_cache_path: String,
    pub user: Option<StoredUser>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OkOutput {
    pub ok: bool,
    pub message: String,
}

pub async fn run(command: AuthCommand, base_url: &str) -> Result<Output> {
    match command {
        AuthCommand::Login(args) => {
            let email = match args.email {
                Some(email) => email,
                None => prompt_line("Email: ")?,
            };
            let password = if args.password_stdin {
                read_stdin()?.trim_end_matches(['\r', '\n']).to_string()
            } else {
                rpassword::prompt_password("Password: ")?
            };

            let client = anonymous_client(base_url)?;
            let login = auth_api::login(&client, LoginRequest { email, password }).await?;
            let stored = StoredAuth {
                base_url: base_url.to_string(),
                user: login.user.into(),
                access_token: login.access_token,
                refresh_token: login.refresh_token,
            };
            save_auth(&stored)?;
            Ok(Output::AuthStatus(AuthStatusOutput {
                authenticated: true,
                base_url: stored.base_url,
                token_cache_path: auth_path()?.display().to_string(),
                user: Some(stored.user),
            }))
        }
        AuthCommand::Status => {
            let stored = match load_auth()? {
                Some(stored) => Some(refresh_auth_if_needed(base_url, stored).await?),
                None => None,
            };
            Ok(Output::AuthStatus(AuthStatusOutput {
                authenticated: stored.is_some(),
                base_url: base_url.to_string(),
                token_cache_path: auth_path()?.display().to_string(),
                user: stored.map(|auth| auth.user),
            }))
        }
        AuthCommand::Logout => {
            remove_auth()?;
            Ok(Output::Ok(OkOutput {
                ok: true,
                message: "logged out".to_string(),
            }))
        }
    }
}

impl Printer for ReadablePrinter<AuthStatusOutput> {
    fn print_output(self) -> Result<()> {
        let out = &self.inner;
        let mut table = kv_table();
        table.add_row(["authenticated", bool_text(out.authenticated)]);
        table.add_row(["base_url", out.base_url.as_str()]);
        table.add_row(["token_cache_path", out.token_cache_path.as_str()]);
        if let Some(user) = &out.user {
            table.add_row(["user", user.nickname.as_str()]);
            table.add_row(["email", user.email.as_str()]);
            table.add_row(["role", user.role.as_str()]);
        }
        println!("{table}");
        Ok(())
    }
}

impl Printer for ReadablePrinter<OkOutput> {
    fn print_output(self) -> Result<()> {
        let out = &self.inner;
        let mut table = kv_table();
        table.add_row(["ok", bool_text(out.ok)]);
        table.add_row(["message", out.message.as_str()]);
        println!("{table}");
        Ok(())
    }
}
