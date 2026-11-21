# Splat

Use Splat to prepare and run D&D 2024 encounters as a Dungeon Master.

## Start Splat

On Linux, install Nix and devenv. Open a terminal in this repository and run:

```sh
devenv shell -- cargo run --locked
```

## Set up a campaign

Use one of these options in **Settings**.

- **Local campaign.** Open the **Add campaign** form if it is closed. Enter
  a **Campaign name** and a **Folder**, then select **Add campaign**. You can
  use a folder in an Obsidian vault.
- **D&D Beyond campaign.** Connect your account, then expand **Add from D&D Beyond**.
  Select **Add…** beside the campaign, choose a folder and select **Add campaign**.
  [Connection steps](docs/campaigns-and-party.md#dd-beyond-connection)

Use the campaign switcher at the top of the sidebar to select your campaign.

## Prepare and run a fight

1. In **Encounters**, select **New encounter**, add creatures to the roster and
   select **Save encounter**.
2. Select **Start fight**, enter or roll initiative, then select **Begin combat**.
3. Select **Next turn** to advance through the roster.

Open **Session** to write notes and plan fights. Press **Ctrl+K** to look up
creatures, rules and notes from any page.

## Find more instructions

- [User guide](docs/user-guide.md)
- [Work with Splat files in Obsidian](docs/obsidian-file-layouts.md)
