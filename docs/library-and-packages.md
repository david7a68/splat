# Library and packages

## Find an entry

1. Open **Library**. The active campaign stays selected.
2. Use the scope menu to browse what the campaign can use, all entries, one
   package, or **Campaign entries**.
3. Search by name, path or section. Clear the search to return to the selected section.

The built-in D&D 2024 SRD is read-only. Open **Source and identifiers** for
source details and **What Splat applies**. Open **Edit Markdown** to edit local
entries. [Built-in SRD source package](srd-import.md)

Readers accept links such as `[[Name]]`, `[[Name#Heading]]` and `[[Name|text]]`.
If several entries have the same name, choose one. Use Alt+Left and Alt+Right
to go back and forward.

## Register or create a package

1. Open **Packages → Manage packages…**.
2. To register a package, choose a folder with one manifest. Keep package and
   campaign folders separate.
3. To create a package, enter its name. Check the new path or use **Browse…**
   to choose its parent folder.

New packages use `Package.md`. Splat also accepts existing `Module.md`
manifests. **Unregister** keeps the files and offers **Undo unregister**.
If a package moves, use **Find the folder…** from Campaign to locate it. If
you copied the folder, use **Register as a new package** to keep both packages.

### Repair a manifest

Open the package's **About** page for its warning. **Write the manifest again**
restores a missing manifest. If the file reads as text but has invalid
properties, repair it in the Library editor. You can also choose the basic
replacement after reviewing which fields it will drop. Replacing a manifest
offers Undo. If Splat cannot read the file as UTF-8 text, use **Show file** to
locate it and **Try again** after fixing it.

Splat leaves plain README text in `Package.md` or `Module.md` in place. If
both names hold notes, free one of the names before writing a new manifest.

## Create, choose or share an entry

- Select **New entry** and check the kind and path before saving. Use **Choose
  for this campaign** to make the entry available from encounter search.
- In **Campaign → Packages**, choose the whole package or individual entries.
  A whole package includes future additions. Required entries appear with the
  entries that need them. Select **Save selections** to update `Campaign.md`.
- Open **Needed by your entries** in Campaign to find missing dependencies.
  Select one to open it in Lookup.
- Select **Copy to…** to make an independent copy. Check the destination and
  dependencies before selecting **Create copy**. If a package copy requires a
  campaign entry, choose **Copy both** to keep that link between the new copies.

Splat routes imported D&D Beyond definitions to enabled package notes that list
their IDs in `splat_dndbeyond_ids`. Other definitions go into the **Campaign
Package** folder. Review proposed moves under **Campaign → Packages**.
[Imported character content](design/character-content.md)

## Repair an entry or recover a draft

- For invalid Markdown or YAML, select **Fix in Edit Markdown** to go to the
  error line. If Splat cannot read the file at all, use **Show file** and
  **Try again**.
- If a file changed on disk, compare the draft and disk versions. Choose
  **Overwrite with my draft** or **Reload and lose my draft**. **Discard**
  offers **Undo discard** until the next edit or disk change.
- If a file was deleted, use **Save my draft again** or **Remove entry and
  lose draft**. If it moved, saving follows its new path.
- If two files share an identity, select **Make `filename` a separate entry**.
  Review which encounters would change source. The repair offers Undo.

Drafts stay while you change pages. Save or discard them before switching
campaigns or closing Splat. To check several files at once, use the
[read-only file checker](obsidian-file-layouts.md#check-files).

[Back to the user guide](user-guide.md).
