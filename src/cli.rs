use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

use crate::data_export::resource::ExportFormat;
use crate::data_export::service::ExportService;
use crate::permissions::setup::PermissionSetup;

#[derive(ValueEnum, Clone)]
enum ExportDomain {
    Connection,
    Packet,
    Bandwidth,
    Process,
}

#[derive(ValueEnum, Clone)]
enum ExportFormatArgs {
    Json,
    Csv,
}

#[derive(Subcommand)]
enum Commands {
    Setup,
    Teardown,
    Version,
    Export {
        #[arg(long = "domain", short = 'd', value_enum)]
        domain: ExportDomain,

        #[arg(long = "format", short = 'f', value_enum, default_value = "json")]
        format: ExportFormatArgs,

        #[arg(long = "output", short = 'o')]
        output: PathBuf,
    },
}

#[derive(Parser)]
#[command(name = "mewn", version)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    pub async fn process(&self) -> Result<bool> {
        match &self.command {
            Some(Commands::Version) => {
                println!("mewn {}", env!("CARGO_PKG_VERSION"));
                Ok(true)
            }
            Some(Commands::Setup) => {
                PermissionSetup::run_setup()?;
                Ok(true)
            }
            Some(Commands::Teardown) => {
                PermissionSetup::run_teardown()?;
                Ok(true)
            }
            Some(Commands::Export { domain, format, output }) => {
                let export_format = match format {
                    ExportFormatArgs::Csv => ExportFormat::Csv,
                    ExportFormatArgs::Json => ExportFormat::Json,
                };
                let service = ExportService::new(export_format, output);
                match domain {
                    ExportDomain::Packet => service.packet().await?,
                    ExportDomain::Connection => service.connections().await?,
                    ExportDomain::Bandwidth => service.bandwidth().await?,
                    ExportDomain::Process => service.process().await?,
                }
                Ok(true)
            }
            None => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn given_no_subcommand_then_process_returns_false() {
        let cli = Cli { command: None };
        let handled = cli.process().await.unwrap();
        assert!(!handled, "None command should return false to launch dashboard");
    }

    #[test]
    fn given_no_args_then_command_is_none() {
        let cli = Cli::parse_from(["mewn"]);
        assert!(cli.command.is_none());
    }

    #[test]
    fn given_setup_arg_then_parses_as_setup_command() {
        let cli = Cli::parse_from(["mewn", "setup"]);
        assert!(matches!(cli.command, Some(Commands::Setup)));
    }

    #[test]
    fn given_teardown_arg_then_parses_as_teardown_command() {
        let cli = Cli::parse_from(["mewn", "teardown"]);
        assert!(matches!(cli.command, Some(Commands::Teardown)));
    }

    #[test]
    fn given_version_arg_then_parses_as_version_command() {
        let cli = Cli::parse_from(["mewn", "version"]);
        assert!(matches!(cli.command, Some(Commands::Version)));
    }

    #[test]
    fn given_export_connections_json_with_short_flags_then_parses_correctly() {
        let cli = Cli::parse_from(["mewn", "export", "-d", "connection", "-f", "json", "-o", "/tmp/out.json"]);
        match cli.command {
            Some(Commands::Export { domain, format, output }) => {
                assert!(matches!(domain, ExportDomain::Connection));
                assert!(matches!(format, ExportFormatArgs::Json));
                assert_eq!(output, PathBuf::from("/tmp/out.json"));
            }
            _ => panic!("expected Export command"),
        }
    }

    #[test]
    fn given_export_packets_csv_with_long_flags_then_parses_correctly() {
        let cli = Cli::parse_from(["mewn", "export", "--domain", "packet", "--format", "csv", "--output", "/tmp/out"]);
        match cli.command {
            Some(Commands::Export { domain, format, output }) => {
                assert!(matches!(domain, ExportDomain::Packet));
                assert!(matches!(format, ExportFormatArgs::Csv));
                assert_eq!(output, PathBuf::from("/tmp/out"));
            }
            _ => panic!("expected Export command"),
        }
    }

    #[test]
    fn given_export_bandwidth_no_format_flag_then_defaults_to_json() {
        let cli = Cli::parse_from(["mewn", "export", "-d", "bandwidth", "-o", "/tmp/out.json"]);
        match cli.command {
            Some(Commands::Export { domain, format, output }) => {
                assert!(matches!(domain, ExportDomain::Bandwidth));
                assert!(matches!(format, ExportFormatArgs::Json));
                assert_eq!(output, PathBuf::from("/tmp/out.json"));
            }
            _ => panic!("expected Export command"),
        }
    }

    #[test]
    fn given_export_process_csv_then_parses_all_four_domains() {
        let cli_conn = Cli::parse_from(["mewn", "export", "-d", "connection", "-f", "csv", "-o", "/tmp/out"]);
        assert!(matches!(
            cli_conn.command,
            Some(Commands::Export {
                domain: ExportDomain::Connection,
                ..
            })
        ));

        let cli_pkt = Cli::parse_from(["mewn", "export", "-d", "packet", "-f", "csv", "-o", "/tmp/out"]);
        assert!(matches!(cli_pkt.command, Some(Commands::Export { domain: ExportDomain::Packet, .. })));

        let cli_bw = Cli::parse_from(["mewn", "export", "-d", "bandwidth", "-f", "csv", "-o", "/tmp/out"]);
        assert!(matches!(
            cli_bw.command,
            Some(Commands::Export {
                domain: ExportDomain::Bandwidth,
                ..
            })
        ));

        let cli_proc = Cli::parse_from(["mewn", "export", "-d", "process", "-f", "csv", "-o", "/tmp/out"]);
        assert!(matches!(
            cli_proc.command,
            Some(Commands::Export {
                domain: ExportDomain::Process,
                ..
            })
        ));
    }
}
