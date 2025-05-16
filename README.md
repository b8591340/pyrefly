# Pyrefly Extension for Zed

This extension adds support for [Pyrefly](https://pyrefly.org/), a static type checker for Python, to Zed.

## Installation

### 1. Install Pyrefly

`pyrefly` must be installed on your system, and available in your `$PATH` in order to use in Zed.

You can install Pyrefly globally on your system using `uv`

```sh
uv tool install pyrefly
```

See the [Pyrefly documentation](https://pyrefly.org/en/docs/installation/) for other installation methods.

### 2. Install the Extension

Search for `pyrefly` in the Zed extensions panel and click to install.

## Enable

Disable `pyright` (or other Python language servers) and enable `pyrefly` in your Zed settings:

```jsonc
{
  "languages": {
    "Python": {
      "language_servers": ["pyrefly", "!pyright", "!pylsp"]
    },
  }
}
```

## Configure

Configure under `lsp.pyrefly.settings` as required.

The "binary" setting is optional. If not set, `pyrefly` will be searched for in your `$PATH`.

```jsonc
{
  "lsp": {
    "pyrefly": {
      "binary": {
        "path": ".venv/bin/pyrefly",
        "arguments": ["lsp"]
      },
      "settings": {
        "python": {
          "pythonPath": ".venv/bin/python"
        },
        "pyrefly": {
          "project_includes": ["src/**/*.py", "tests/**/*.py"],
          "project_excludes": ["**/.[!/.]*", "**/*venv/**"],
          "search_path": ["src"],
          "ignore_errors_in_generated_code": true
        }
      }
    }
  }
}
```

## Configuration Options

Pyrefly offers a variety of configuration options, which can be specified in your Zed settings or in a `pyrefly.toml` or `pyproject.toml` file in your project:

- `project_includes`: List of glob patterns for files to type check
- `project_excludes`: List of glob patterns for files to exclude from type checking
- `search_path`: Roots for import resolution
- `python_version`: Python version to use for type checking
- `python_platform`: Platform to use for type checking
- `ignore_errors_in_generated_code`: Whether to ignore errors in generated code

For a complete list of configuration options, see the [Pyrefly documentation](https://pyrefly.org/en/docs/configuration/).

## Using with Virtual Environments

For virtual environments, specify your Python interpreter in your project settings (`zed: open project settings`):

```jsonc
{
  "lsp": {
    "pyrefly": {
      "settings": {
        "python": {
          "pythonPath": ".venv/bin/python"
        },
        "pyrefly": {
          "python_interpreter": ".venv/bin/python"
        }
      }
    }
  }
}
```

## Learn More

- [Pyrefly Documentation](https://pyrefly.org/en/docs/)
- [Python Typing Guide](https://pyrefly.org/en/docs/typing-for-python-developers/)
