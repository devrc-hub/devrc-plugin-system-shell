#[macro_use]
extern crate devrc_plugins;
use std::process::Command;

use std::str;

use devrc_plugins::{
    config::{Config, ExecutionConfig},
    errors::DevrcPluginResult,
    execution::ExecutionPlugin,
    options::PluginOption,
    plugin::Plugin,
    DevrcPluginError,
};

pub const DEFAULT_SHELL: &str = "sh";
pub const DEFAULT_SHELL_ARG: &str = "-c";

#[derive(Debug, Default)]
pub struct SystemShell {
    config: Config,
}

impl Plugin for SystemShell {
    fn name(&self) -> &'static str {
        "system-shell"
    }

    fn on_plugin_load(&mut self, config: Config) {
        self.config = config;
    }

    fn on_plugin_unload(&self) {}
}

impl ExecutionPlugin for SystemShell {
    fn execute(
        &self,
        config: ExecutionConfig,
        code: &str,
        environment: &indexmap::IndexMap<String, String>,
    ) -> DevrcPluginResult<i32> {
        let shell = match config.options.get("shell") {
            Some(PluginOption::String(shell)) => (*shell).clone(),
            _ => DEFAULT_SHELL.to_string(),
        };

        let mut command = Command::new(&shell);
        command.export_environment(environment)?;

        if let Some(value) = &config.current_dir {
            command.current_dir(value);
        }

        for arg in &config.args {
            command.arg(arg);
        }

        command.arg(code);

        // Handle signals
        match command.status() {
            Ok(exit_status) => {
                if let Some(code) = exit_status.code() {
                    if code != 0 {
                        // Raise runtime error
                        return Err(DevrcPluginError::Code { code });
                    }
                } else {
                    self.config.logger.error(
                        "\nProcess terminated by signal",
                        &self.config.designer.error(),
                    );
                    return Err(DevrcPluginError::Signal);
                }
            }
            Err(io_error) => {
                return Err(DevrcPluginError::IoError(io_error));
            }
        }
        Ok(0)
    }
}

declare_execution_plugin!(SystemShell, SystemShell::default);

pub trait CommandExt {
    fn export_environment(
        &mut self,
        environment: &indexmap::IndexMap<String, String>,
    ) -> DevrcPluginResult<()>;
}

impl CommandExt for Command {
    fn export_environment(
        &mut self,
        environment: &indexmap::IndexMap<String, String>,
    ) -> DevrcPluginResult<()> {
        for (key, value) in environment.into_iter() {
            self.env(key, value);
        }

        Ok(())
    }
}
