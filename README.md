# Auto Updater
Add to your bashrc or zshrc to automatically check for updates on your set package managers

```bash
auto-updater upgrade
```

```bash
echo 'auto-updater upgrade' >> .zshrc

OR

echo 'auto-updater upgrade' >> .bashrc
```

## Install
```bash
brew tap mckernant1/tap
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

## Upgrade
To upgrade a specific manager or ignore the timestamps
```bash
auto-updater upgrade [-f] [NAME]
```
You then get prompted to update not update or skip.
- Yes runs the commands and updates the last updated time
- No does not run the commands and does not update the last updated time
- Skip does not run the commands and updates the last updated time
```
It's time to update brew, would you like to update now (y/N/s): 
```

### Json structure
Here is my example JSON structure
```json5
{
  "brew": {
    "frequency": "2d",
    "lastUpdated": "2020-05-02T20:24:40.095050+00:00",
    "commands": [
      "brew update",
      "brew upgrade",
      "brew cask upgrade",
      "brew cleanup"
    ]
  },
  "rustup": {
    "frequency": "1m",
    "lastUpdated": "2020-05-03T23:43:07.023288+00:00",
    "commands": [
      "rustup update"
    ]
  },
  "npm": {
    "frequency": "1w",
    "lastUpdated": "2020-04-27T17:02:15.277517+00:00",
    "commands": [
      "npm i -g npm",
      "npm update -g"
    ]
  }
}

```
