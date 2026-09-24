# Library and campaign content UI

`library-sections.png` shows the section sidebar and the Ability Check reader in
the built-in SRD. It was captured from the running app at 1440 × 960 on an isolated
X11 display, with temporary settings and no desktop keyring connection. The reader
opens the source text first and keeps rule details and identifiers expandable.

`library-refined.png` shows the later hierarchy and density changes, with the
overview reflowed as prose and Rule data open in a compact label/value layout.
It uses the same isolated capture setup and built-in entry.

These images were captured from the running Rust/egui desktop app on an isolated
X11 display. The Northreach module and campaign are sample data in a temporary
settings directory. No user campaign or credentials were used.

- `library-module.png`: reusable module reader and copy control.
- `campaign-modules.png`: saved partial module selection and required dependency reasons.
- `copy-review.png`: independent copy preview with retained shared dependency.
- `campaign-entry.png`: saved campaign-only edits to the copy.

The live checks copied Watch Captain to Captain Elara, saved module selections,
and edited the campaign copy while preserving the module original.

The reverse flow also copied Captain Elara back to the shared module as Veteran
Captain, retaining the shared Frost Ward reference and creating another UUID.
