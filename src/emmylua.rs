use std::fs;
use zed::lsp::CompletionKind;
use zed::{CodeLabel, CodeLabelSpan, LanguageServerId};
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{self as zed, Command, Result};

struct EmmyLuaExtension {
    cached_binary_path: Option<String>,
}

impl EmmyLuaExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Ok(lsp_settings) = LspSettings::for_worktree("emmylua", worktree) {
            if let Some(binary) = lsp_settings.binary {
                if let Some(path) = binary.path {
                    return Ok(path);
                }
            }
        }

        if let Some(path) = worktree.which("emmylua_ls") {
            return Ok(path);
        }

        if let Some(path) = worktree.which("emmylua") {
            return Ok(path);
        }

        // Check cached path
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
        let mut env = worktree.shell_env();
        let mut args = Default::default();

        if let Ok(lsp_settings) = LspSettings::for_worktree("emmylua", worktree) {
            if let Some(binary) = lsp_settings.binary {
                if let Some(binary_arguments) = binary.arguments {
                    args = binary_arguments;
                }
                if let Some(binary_env) = binary.env {
                    env.extend(binary_env);
                }
            }
        }

        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args,
            env,
        })
    }

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<CodeLabel> {
        let name = &completion.label;

        match completion.kind? {
            CompletionKind::Method | CompletionKind::Function => {
                let name_len = name.find('(').unwrap_or(name.len());
                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(0..name.len())],
                    filter_range: (0..name_len).into(),
                    code: name.clone(),
                })
            }
            CompletionKind::Field | CompletionKind::Property => Some(CodeLabel {
                spans: vec![CodeLabelSpan::literal(
                    name.clone(),
                    Some("property".into()),
                )],
                filter_range: (0..name.len()).into(),
                code: Default::default(),
            }),
            CompletionKind::Variable => Some(CodeLabel {
                spans: vec![CodeLabelSpan::literal(
                    name.clone(),
                    Some("variable".into()),
                )],
                filter_range: (0..name.len()).into(),
                code: Default::default(),
            }),
            CompletionKind::Class => Some(CodeLabel {
                spans: vec![CodeLabelSpan::literal(name.clone(), Some("type".into()))],
                filter_range: (0..name.len()).into(),
                code: Default::default(),
            }),
            CompletionKind::Module => Some(CodeLabel {
                spans: vec![CodeLabelSpan::literal(name.clone(), Some("module".into()))],
                filter_range: (0..name.len()).into(),
                code: Default::default(),
            }),
            CompletionKind::Keyword => Some(CodeLabel {
                spans: vec![CodeLabelSpan::literal(name.clone(), Some("keyword".into()))],
                filter_range: (0..name.len()).into(),
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
        let name = &symbol.name;

        match symbol.kind {
            zed::lsp::SymbolKind::Method | zed::lsp::SymbolKind::Function => {
                let code = format!("function {}()", name);
                let start = code.find(name)?;
                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(start..start + name.len())],
                    filter_range: (0..name.len()).into(),
                    code,
                })
            }
            zed::lsp::SymbolKind::Class | zed::lsp::SymbolKind::Module => Some(CodeLabel {
                spans: vec![CodeLabelSpan::literal(name.clone(), Some("type".into()))],
                filter_range: (0..name.len()).into(),
                code: name.clone(),
            }),
            zed::lsp::SymbolKind::Variable | zed::lsp::SymbolKind::Constant => Some(CodeLabel {
                spans: vec![CodeLabelSpan::literal(
                    name.clone(),
                    Some("variable".into()),
                )],
                filter_range: (0..name.len()).into(),
                code: name.clone(),
            }),
            _ => Some(CodeLabel {
                spans: vec![CodeLabelSpan::code_range(0..name.len())],
                filter_range: (0..name.len()).into(),
                code: name.clone(),
            }),
        }
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = zed::settings::LspSettings::for_worktree("emmylua", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        Ok(Some(settings))
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = zed::settings::LspSettings::for_worktree("emmylua", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_default();

        Ok(Some(settings))
    }
}

zed::register_extension!(EmmyLuaExtension);
