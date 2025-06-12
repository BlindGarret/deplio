# Deplio CLI Tool

A setup tool for deplio projects that handles configuration, project initialization, GitHub actions, and configuration version updates.

## Debugging

Using the tool locally to debug:

```bash
cargo run --package cli -- [PARAMETERS]
```

## Commands

### `config`

Creates and manages the deplio configuration file for the user.

```bash
deplio config [OPTIONS]
```

**Options:**
- `-e, --edit` - Open the configuration file in the default editor
- `-o, --overwrite` - Forces the overwrite of the config file with a new empty template

**Examples:**
```bash
# Create a new configuration file
deplio config

# Create and immediately edit the configuration file
deplio config --edit

# Overwrite existing configuration with a new template
deplio config --overwrite
```

### `init`

Initializes the project files for a given repository using the latest version. This includes the project configuration and GitHub actions.

```bash
deplio init [OPTIONS]
```

**Options:**
- `-a, --app-name <APP_NAME>` - The name of the application to initialize
- `-o, --owner <OWNER>` - The owner of the project to initialize

**Examples:**
```bash
# Initialize with prompts for required information
deplio init

# Initialize with specified app name and owner
deplio init --app-name my-app --owner my-username
```

### `update`

Updates the project to a specified version.

```bash
deplio update [OPTIONS]
```

**Options:**
- `-v, --version <VERSION>` - The version to upgrade to. If not included 'latest' is assumed

**Examples:**
```bash
# Update to the latest version
deplio update

# Update to a specific version
deplio update --version 1.2.3
```

### `debug`

A set of debug commands useful for development on the project.

```bash
deplio debug [SUBCOMMAND]
```

#### Debug Subcommands

##### `proj-backup`

Creates a backup of generated project files for quick restore when testing upgrade features.

```bash
deplio debug proj-backup
```

##### `proj-restore`

Restores a backup of generated project files.

```bash
deplio debug proj-restore [OPTIONS]
```

**Options:**
- `-p, --purge` - Clear the backup when restored

**Examples:**
```bash
# Restore backup and keep it
deplio debug proj-restore

# Restore backup and delete it
deplio debug proj-restore --purge
```

## Configuration

The tool uses a configuration file stored in your home directory. Use `deplio config` to create and manage this file. Configuration values can provide defaults for commands, reducing the need for manual input during project initialization.

