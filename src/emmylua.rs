use std::fs;
use zed::lsp::CompletionKind;
use zed::{CodeLabel, CodeLabelSpan, LanguageServerId};
use zed_extension_api::{self as zed, Result};

struct EmmyLuaExtension {
    cached_binary_path: Option<String>,
}

impl EmmyLuaExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which("emmylua") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            "EmmyLuaLs/emmylua-analyzer-rust",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let asset_name = format!(
            "emmylua_ls-{os}-{arch}{glibc}.{extension}",
            os = match platform {
                zed::Os::Mac => "darwin",
                zed::Os::Linux => "linux",
                zed::Os::Windows => "win32",
            },
            arch = match arch {
                zed::Architecture::Aarch64 => match platform {
                    zed::Os::Linux => "aarch64",
                    zed::Os::Windows | zed::Os::Mac => "arm64",
                },
                zed::Architecture::X8664 => "x64",
                zed::Architecture::X86 => return Err("unsupported platform x86".into()),
            },
            glibc = match platform {
                zed::Os::Linux => "-glibc.2.17",
                zed::Os::Windows | zed::Os::Mac => "",
            },
            extension = match platform {
                zed::Os::Mac | zed::Os::Linux => "tar.gz",
                zed::Os::Windows => "zip",
            },
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("emmylua-{}", release.version);
        let binary_path = format!(
            "{version_dir}/emmylua_ls{extension}",
            extension = match platform {
                zed::Os::Mac | zed::Os::Linux => "",
                zed::Os::Windows => ".exe",
            },
        );

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                match platform {
                    zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
                    zed::Os::Windows => zed::DownloadedFileType::Zip,
                },
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for EmmyLuaExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: Default::default(),
            env: Default::default(),
        })
    }

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<CodeLabel> {
        match completion.kind? {
            CompletionKind::Method | CompletionKind::Function => {
                let name_len = completion.label.find('(').unwrap_or(completion.label.len());
                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(0..completion.label.len())],
                    filter_range: (0..name_len).into(),
                    code: completion.label,
                })
            }
            CompletionKind::Field => Some(CodeLabel {
                spans: vec![CodeLabelSpan::literal(
                    completion.label.clone(),
                    Some("property".into()),
                )],
                filter_range: (0..completion.label.len()).into(),
                code: Default::default(),
            }),
            _ => None,
        }
    }

    fn label_for_symbol(
        &self,
        _language_server_id: &LanguageServerId,
        symbol: zed::lsp::Symbol,
    ) -> Option<CodeLabel> {
        let prefix = "let a = ";
        let suffix = match symbol.kind {
            zed::lsp::SymbolKind::Method => "()",
            _ => "",
        };
        let code = format!("{prefix}{}{suffix}", symbol.name);
        Some(CodeLabel {
            spans: vec![CodeLabelSpan::code_range(
                prefix.len()..code.len() - suffix.len(),
            )],
            filter_range: (0..symbol.name.len()).into(),
            code,
        })
    }
}

zed::register_extension!(EmmyLuaExtension);
