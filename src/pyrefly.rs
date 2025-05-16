use zed_extension_api::{
    register_extension, serde_json, settings::LspSettings, Command, Extension, LanguageServerId,
    Result, Worktree,
};

struct Pyrefly;

impl Extension for Pyrefly {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let env = worktree.shell_env();

        if let Ok(lsp_settings) = LspSettings::for_worktree("pyrefly", worktree) {
            if let Some(binary) = lsp_settings.binary {
                if let Some(path) = binary.path {
                    let args = binary.arguments.unwrap_or(vec!["lsp".to_string()]);
                    return Ok(Command {
                        command: path,
                        args,
                        env,
                    });
                }
            }
        }

        let path = worktree
            .which("pyrefly")
            .ok_or_else(|| "pyrefly must be installed and available in $PATH.".to_string())?;
        Ok(Command {
            command: path,
            args: vec!["lsp".to_string()],
            env: env,
        })
    }

    // ref https://github.com/zed-industries/zed/blob/main/extensions/ruff/src/ruff.rs
    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

register_extension!(Pyrefly);
