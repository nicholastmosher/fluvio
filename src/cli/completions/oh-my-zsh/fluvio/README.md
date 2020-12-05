# Fluvio Plugin for Oh-My-Zsh

This plugin provides aliases and tab-completions for the Fluvio CLI.

## Installation

To use this plugin, you must have [oh-my-zsh](https://ohmyz.sh/)
installed. Then, copy this directory (`completions/oh-my-zsh/fluvio/`)
into `~/.oh-my-zsh/custom/plugins/`, like so

```
# Move to the oh-my-zsh directory
$ cd fluvio/src/cli/completions/oh-my-zsh
$ cp -r ./fluvio ~/.oh-my-zsh/custom/plugins/
```

Then, you'll need to update your `.zshrc` file to load the plugin.
Your `.zshrc` file should have a line like this

```
plugins=(foo)    # It won't actually say foo, it's a list of active plugins
```

Just add the word `fluvio` to the end of that list to activate the plugin

```
plugins=(foo fluvio)
```
