# Design-system consistency

For any change to layout, wording, controls or `docs/design-system.md`. That document is the source of truth;
these are the rules reviews most often found broken. For each one the diff touches, grep for sibling callers:
the same thing must look and behave the same on every screen (Lookup's `Show all` and Arrivals' `Show all`).

## Targets and placement

- Every app target is at least 24 × 24 px and overlaps no other target, no plain words on another line, no
  splitter and no scroll bar. Prove geometry at 960 and 1920.
- A quiet action that starts a line uses `line_start`: its words start at the text edge above (an icon must
  not push them right), and its whole target stays clear of every splitter. A pane that holds one keeps an
  `L` gutter. In a wrapping row, only the first action (or one that wraps to its own line) uses it.
- Choices after a message go through `message_with_actions`: the first at the message text edge, safe first,
  exactly `L` (16 px) before a destructive choice, measured between drawn targets.
- Things do not move between states or owners: a title, a numeric column, an input field, a result slot.
  Reserve space for what will appear (World results sit below the file line; run AC and HP keep their x for
  long names; the Library title has one height for every owner).
- One accent-filled primary per composed screen, counted across every pane and banner in view.

## Shapes and words

- Tokens only; icons from `design::icons`.
- An Undo after a message is `undo_message`: quiet, with its icon.
- A status is an icon, a word and a colour; warning-coloured text without its icon is a fault.
- A link underlines only its words; an icon before it is drawn as its own glyph (`status_link`). A short link
  reserves its 24 px target in the layout; a focus ring sits outside the glyphs and inside the clip; link ids
  survive layout changes and other readers.
- A disabled control says why in visible text; one cause gets one sentence; one word means one thing.
- Cut text ends on a whole word with `…`, keeps its action, and gives the full text in hover and the
  accessible name.
- UI text is plain, short, active English. Text that names a thing uses curly quotes (`New NPC “Miller”`);
  file content keeps straight quotes. No `{:?}` of user values in UI text.
- Every control has an accessible name; a renamed one means a grep of the scenarios for the old name.

## Rule changes

When the change alters a design rule, name the rule it could break and check both at one boundary case.
Two loop rulings failed this: "paragraph layout stays unchanged" broke the 24 px target rule, and "the focus
ring stays inside the paragraph" made the ring cover a letter. `docs/design-system.md` must say what the code
now does, in the right paragraph, with no per-iteration sections.
