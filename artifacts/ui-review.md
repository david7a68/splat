# Claude Fable UI review

Reviewed with Claude CLI model `claude-fable-5-1`. Fable inspected four screenshots
and the UI source. This was a static review, not a live test or approval of the
subsequent fixes.

## Changes after review

- Invalid campaign notes have repair editors; valid manifests and entries can load.
  Selection saves stay disabled until malformed notes are repaired.
- Partial module selection uses an indeterminate checkbox and explicit counts.
  Clearing the last direct entry removes the selection. Toggling a module off and
  back on restores its selection during the current session.
- Global required dependencies appear above the module list. Campaign entries scroll.
- Settings cannot modify campaigns while campaign drafts are pending.
- Copy dialogs block underlying controls, provide Close/Copy another, support Escape,
  and refresh clean destination workspaces.
- Unrelated parse diagnostics no longer block independent copies. Required references
  still resolve and are checked before writing.
- Library reload preserves the selected path. Creation sorts and selects the new note,
  clears filters, and reports name conflicts. Creation-only fields are labeled.
- Library saves show confirmation and support Ctrl/Cmd+S. Campaign status clears when
  a draft changes or the content tab changes.
- Library reading uses the frontmatter parser, including BOM/CRLF handling.
  Markdown now preserves ordered-list numbering, indentation and hard breaks,
  and gives strong text more contrast.

## Remaining review suggestions

Duplicate UUIDs, keys and namespaces still fail catalogue loading rather than
quarantining all conflicting modules. This preserves ambiguity protection but can
block unrelated content. Inline code formatting, full nested emphasis, campaign
selection retention on reload, unregister impact prompts, and broader keyboard
shortcuts remain follow-up work. Large-library rendering and per-frame dependency
resolution need measurement before performance changes.

## Verification

80 tests passed with `devenv shell -- cargo test --locked`.
`devenv shell -- check` passed formatting and Clippy with warnings denied.
`git diff --check` passed. The app was run on an isolated X11 display with sample
data after the fixes; the updated campaign screenshot shows partial selection
and the global dependency panel. Other screenshots record the earlier live
copy/edit checks, before these review fixes.

## Original review

I inspected all four screenshots and read all seven source files plus the README. Findings below are ordered by severity. "Confirmed" means the behaviour follows directly from the code or is visible in a screenshot. "Hypothesis" means I could not verify it without running the app.

## Findings

**1. High, confirmed: one invalid campaign note locks the entire Campaign Content tab.**
- Where: `src/content_workspace.rs:434-440` returns an error from `CampaignDocument::load` on any parse error, and `src/campaign_content.rs:140` returns before rendering either sub-tab when there is no snapshot.
- Impact: a single bad frontmatter line in the campaign folder hides module selection and every other campaign entry, including the editor you would use to fix it. The README says repair externally, but the Library keeps invalid notes editable, so the two workspaces behave differently.
- Fix: make the load tolerant, carry the diagnostics into the snapshot, disable only "Save selections" while errors exist, and list invalid notes in "Campaign entries" for repair as the Library does.

**2. Medium, confirmed: the module checkbox cannot distinguish "all", "some", or "none".**
- Where: `src/campaign_content.rs:249-263` draws the checkbox as checked whenever any selection record exists. "Clear direct selection" at line 318 leaves an empty record, so the box stays checked with nothing selected. The campaign-modules screenshot shows Northreach Almanac checked while only two of four entries are chosen.
- Impact: users cannot tell from the module list whether future additions are included, and unchecking a module silently deletes a hand-picked list. Re-checking restores "All", not the previous picks.
- Fix: use `Checkbox::indeterminate` for partial selections, show "All", "2 of 4", or "None" next to each module, and remove the record when the last direct entry is cleared.

**3. Medium, confirmed by layout, not visually verified: "Required dependencies" is pushed off-screen for large modules.**
- Where: the entry list at `src/campaign_content.rs:350` uses a vertical scroll area with no height cap, and the required-dependencies header follows it at line 402. The SRD is the default module shown, with about two thousand rows.
- Impact: on first open of the tab, the dependency reasons are unreachable, which undermines the "required dependencies included with reasons" workflow. The list is also global but rendered under the current module's heading.
- Fix: render the required section above the list or in a bottom panel, and label it as spanning all modules.

**4. Medium, confirmed: the "Campaign entries" list does not scroll.**
- Where: `src/campaign_content.rs:439-463` draws the entry buttons straight into the side panel with no scroll area, unlike the Library list at `src/library.rs:463`.
- Impact: once the campaign has more entries than fit vertically, the later ones are unreachable.
- Fix: wrap the loop in a vertical scroll area.

**5. Medium, confirmed: status messages go stale and success feedback is inconsistent.**
- Where: `message` and `error` in `src/campaign_content.rs:120-125` are only cleared in `reload`. "Campaign selections saved." stays visible after further checkbox changes, and "Campaign entry saved." carries over to the Modules tab. The Library save at `src/library.rs:657-677` shows no success message at all.
- Impact: users see "saved" while changes are pending, and get no confirmation after a Library save beyond the Save button greying out.
- Fix: clear the message whenever selections or the draft change, scope it to the sub-tab that produced it, and add a matching "Entry saved" message to the Library.

**6. Medium, confirmed: unrelated content problems block copying and campaign loading.**
- Where: `src/content_workspace.rs:586-591` refuses any copy if any registered module has any error. `Catalogue::load` is fatal on a duplicate namespace or duplicate entry identity across modules at lines 128, 167 and 180.
- Impact: a broken note in an unrelated module disables "Copy to…" everywhere, and a manually duplicated note file makes the whole Campaign Content tab fail with one line naming the entry but not the module.
- Fix: only block copies whose dependency closure touches an errored entry, and turn cross-module duplicates into per-module diagnostics that exclude the offending module instead of failing the load.

**7. Medium, confirmed: the copy workflow ends in a dead end.**
- Where: `src/copy_dialog.rs:64` shows "Reload the destination workspace to see the copy." with no close button and no further action. `src/main.rs:191-198` replaces an open dialog silently if "Copy to…" is clicked again, because the window is not modal.
- Impact: the user has to find the X, then find and press the right Reload, which in the Library also drops the current selection. A second "Copy to…" discards a reviewed plan without warning.
- Fix: have the dialog return a completion event so the app reloads the destination when it is the loaded module or campaign and not dirty, add "Close" and "Copy another", and ignore or confirm new copy requests while a dialog is open.

**8. Medium, confirmed: Settings bypasses the unsaved-campaign-content guard.**
- Where: `src/main.rs:136` disables the sidebar selector while dirty, but `src/main.rs:160` renders Settings unguarded. Adding a campaign at `src/campaigns.rs:347` or removing one at line 301 changes the active campaign, which lands the user on the "Discard and switch campaign" screen at `src/campaign_content.rs:76-83`.
- Impact: removing the active campaign while an entry draft is unsaved leaves the draft with no route back except discard.
- Fix: block remove and add in Settings while campaign content is dirty, or keep the previous document loaded and offer "Save pending changes" on that screen.

**9. Low, confirmed: Library Read view strips frontmatter by hand and breaks on CRLF or BOM notes.**
- Where: `src/library.rs:617-622` splits on a literal `\n---\n`, while `src/campaign_content.rs:524` and `Entry::parse` normalise line endings and the BOM.
- Impact: notes saved with Windows line endings show raw YAML above the body in Read mode.
- Fix: use `Entry::parse` with a fallback to the raw draft, as the campaign editor does.

**10. Low, confirmed: Markdown reader loses common formatting.**
- Where: `src/markdown_view.rs:101-121` renders bold with the same size and colour as body text, so it is invisible. Lines 91-95 render ordered lists as bullets and do not indent nested lists. Line 96 turns hard breaks into spaces.
- Impact: SRD stat blocks that rely on bold labels and numbered steps read flat.
- Fix: use the strong text colour for bold, track list numbering from the `List` tag, indent by nesting depth, and emit a newline for hard breaks.

**11. Low, confirmed: new Library entries land in odd places.**
- Where: `src/library.rs:525-531` appends the new note to the end of the sorted list, and the kind filter at line 447 can hide it. A name clash surfaces the raw tempfile OS error from `src/library.rs:160`.
- Fix: re-sort after insertion, clear the kind filter on create, and map the clobber error to "A note with this name already exists."

**12. Low, confirmed: Reload and Manage modules lose context or lack safeguards.**
- Where: `src/library.rs:254-255` clears the selection on every reload. In the form at lines 337-395, Name and Namespace only apply to "Create module" without saying so, and "Unregister module" runs with no confirmation even though it makes campaign selections show "Unavailable module".
- Fix: reselect the previous entry by path after reload, group the create-only fields under a "Create" heading, and confirm unregister with the count of campaigns that select the module.

**13. Low, confirmed: accessibility gaps.**
- Where: disabled controls with no reason at `src/main.rs:136`, `src/campaign_content.rs:111` and `src/copy_dialog.rs:76`. Colour is the only signal for errors and success at `src/library.rs:400`, `src/campaign_content.rs:121-125` and `src/copy_dialog.rs:64-66`. No Ctrl+S to save, no Escape to close the copy dialog, and the close-confirmation window at `src/main.rs:88-112` offers no Save.
- Fix: add `on_disabled_hover_text` explanations, prefix status text with "Error:" or "Saved:", bind Ctrl+S and Escape, and add "Save and close".

**14. Low, hypothesis: Library list performance at SRD scale.**
- Where: `src/library.rs:463-479` lays out every entry each frame as a two-line button, while the campaign list uses `show_rows`. The closure at `src/campaign_content.rs:156` also re-walks all selected entries every frame.
- Impact if true: sluggish scrolling and typing in the search box on the SRD module. Not measured.
- Fix: use `show_rows` in the Library and cache the closure until selections change.

Trivial text nit: "1 dependencies remain references" at `src/copy_dialog.rs:88`.

## Overall assessment

The core design holds up well. Selection by reference, required dependencies with reasons, new UUIDs on copy, no reference rewriting, the private-dependency check, and baseline checks before every write are all implemented as advertised, and the screenshots show the happy path working for a small module. The main risks are lockouts from strict loading, ambiguous module selection state, and layout overflow that only appears at SRD scale, which the screenshots do not exercise. Nothing here requires changing the design constraints.

## Limitations

This was a static review. I did not run the app or the test suite, so the layout claims in findings 3 and 4 rest on egui's layout rules rather than a rendered check, and finding 14 is untested. The screenshots cover a five-entry module and a one-entry campaign, so none of them show the large-module cases. I did not review `campaigns.rs`, `persistence.rs` or the content parser beyond the functions these screens call.
