# Auto Updater
Add to your bashrc or zshrc to automatically check for updates on your set package managers

## Install
```bash
brew tap mckernant1/tools
brew install auto-updater
```

## Add
To add a command do this. Then follow the prompts

```bash
auto-updater add <NAME>
```

```bash
auto-updater add brew
```



### Json structure
```json5
{
  brew: {
    howOften: "1d", // how often to automatically update
    lastUpdated: "", // timestamp
    commands: [
        "brew update",
        "brew upgrade",
        "brew cask upgrade",
        "brew cleanup"
      ]
  }
}
```
