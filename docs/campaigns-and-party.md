# Campaigns and the party

## Add or switch a campaign

- **Local campaign.** In **Settings**, select a data folder and choose
  **Add campaign**. You can use an existing folder in an Obsidian vault.
- **D&D Beyond campaign.** After connecting, expand **Add from D&D Beyond** and
  select the campaign. Splat fills in its name. Choose its data folder, then
  select **Add campaign**. Each D&D Beyond campaign can link to one local campaign.
- **Switch or remove.** Use the sidebar switcher to change campaigns. To remove
  a registration, open **More actions** on its row. The campaign files stay.
- **Change a link.** Choose another D&D Beyond campaign or select **Not linked**.

Splat reads the campaign manifest on startup or when you select **Reload**.
Opening an existing campaign folder keeps its identity, settings and notes.
If you copied a campaign folder, use **Register as a new campaign** to give it
its own identity. The result offers **Undo register campaign**.

## D&D Beyond connection

1. Open **Settings**, then open the D&D Beyond sign-in link.
2. Sign in to D&D Beyond in your browser.
3. In the browser's developer tools, open **Application** or **Storage**. Under
   cookies for `dndbeyond.com`, copy the value of `CobaltSession`.
4. Paste the value into Splat's masked field and select **Connect**.

Keep the cookie and tokens out of Git. Select **Remember session in desktop
keyring** to reconnect at startup. If the keyring is unavailable, you can
connect for the current app session. Splat tells you if it could not save the cookie.

**Disconnect** removes local credentials and leaves your browser sign-in active.
If keyring removal fails, retry from the error.

## Read or refresh character sheets

1. Open **Party** and select a character.
2. Choose **Overview**, **Actions**, **Spells**, **Equipment** or **Features**.
3. Select **Refresh party** for the latest sheets. If signed out, select
   **Connect to refresh** or **Connect D&D Beyond**.

Splat keeps local copies for offline use and encounter building. To learn
which changes can go back to the sheet, see
[D&D Beyond combat sync](encounters-and-runs.md#dd-beyond-combat-sync).

### Replace a locally edited sheet

Editing a generated character note pauses updates and marks the sheet
**Local edits**. Use **Show file** to inspect it. To resume updates, select
**Replace with D&D Beyond sheet**. Sign in if prompted.

Splat keeps your previous version in a `.splat-archive` file beside the note.
It carries over personal notes it can still find. Select **Show file** in
the result to open the archived copy.

### Share a vault between computers

One computer at a time saves automatic sheet updates to a shared vault.
`Characters/Sheet writer.md` records which one. Other computers can still
read sheets. **Refresh party** can save from any computer.

A new claim waits one minute before saving. Another computer can take over
about 10 minutes after the writer stops, if the clocks agree. If Splat warns
that the writer note already belongs to another purpose, move that note to
let automatic saves resume.

## Repair old settings

Splat stores local settings in `$XDG_CONFIG_HOME/splat/settings.json`, or
`~/.config/splat/settings.json` on Linux when that variable is unset.
Credentials stay in the keyring and memory.

Versions 1 and 2 can migrate to version 3. If Splat reports a conflict, leave
at most one D&D Beyond ID per old entry and make each ID unique. Reload before
saving. Splat keeps the original file until a save succeeds. If you edit the
settings file outside Splat, reload before saving from the app.

[Back to the user guide](user-guide.md).
