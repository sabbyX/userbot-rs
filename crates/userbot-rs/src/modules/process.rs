
use anyhow::Result;
use userbot_rs_macros::handler;
use clap::{Clap, AppSettings, ArgSettings};
use grammers_client::{types::Message, InputMessage};
use tokio::process::Command;
use kantex_rs::{Document, Sections, FormattedText, KeyValueItem};
use which::which;
use std::ffi::OsStr;
use crate::modules::core::UpdateData;

/// Plugin to execute shell commands
#[derive(Clap)]
#[clap(name = "process", setting = AppSettings::NoBinaryName, version = "0.1.0")]
struct Arguments {
    /// program to execute
    #[clap(setting = ArgSettings::AllowHyphenValues)]
    command: Vec<String>,
}

#[handler(command = "proc")]
pub async fn process_command(mut message: Message, _: UpdateData) -> Result<()> {
    let parsed = Arguments::try_parse_from(message.text().trim_start_matches("*proc").split_whitespace());
    if let Ok(args) = parsed {
        // inject arguments
        let mut arguments: Vec<String> = vec!["-c".into()];
        arguments.append(&mut args.command.clone());
        // find OS shell
        let shell = if cfg!(target_os = "windows") {
            let program_path = which("powershell").unwrap_or(which("cmd")?);
            program_path.file_name().unwrap().to_str().unwrap().to_owned()
        } else {
            "sh".into()
        };
        // execute
        let output = Command::new(OsStr::new(shell.as_str()))
            .args(arguments)
            .output()
            .await?;

        let mut doc = Document::new()
            .add_section(
                Sections::new("Output")
                    .include(KeyValueItem::new(FormattedText::bold("Exit Code"), output.status.code().unwrap_or(1).to_string()))
            );
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.is_empty() {
            doc.add_section(
                Sections::new("Stderr")
                    .include(FormattedText::monospace(stderr.as_ref()))
            );
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.is_empty() {
            doc.add_section(
                Sections::new("Stdout")
                    .include(FormattedText::monospace(stdout.as_ref()))
            );
        }
        Ok(
            message.reply(
                InputMessage::html(doc.to_string())
            )
                .await?
        )
    } else {
        Ok(message.reply(InputMessage::text(parsed.err().unwrap().to_string())).await?)
    }
}
