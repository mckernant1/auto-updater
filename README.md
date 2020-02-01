# Auto Updater

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
