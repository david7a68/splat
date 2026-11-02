# World and rules: later

These parts were cut from [World and house rules](world-and-house-rules.md). They are not rejected. Build each one only
when play shows the need. They are sketches, and reviews found the gaps listed here, so each needs its own design
pass first.

A third-round reviewer listed these as open needs. The Heralds notes do not need them for the Tournament.

## Amendments

The first candidate, and the smallest. An **amendment** attaches a ruling to an entry without copying it.

```yaml
amends:
  - entry: {key: dnd2024:spells/sending}
    say: Reaches the same planet. With a 5th level slot or higher, the same star system.
      It crosses between systems only through an open connection.
    see: "#Normal Magic"
  - entry: {key: dnd2024:species/human}
    absent: true
    say: There are no humans on Thaorivel.
```

The Reader shows a callout at the top of that entry, wherever it is read. `absent` adds a mark in lists and blocks
nothing.

- **Why it waits.** The named spells are above what level 1 to 3 characters cast, and search finds the rule. The risk
  is that a hurried DM reads the SRD spell first and misses the campaign's limit.
- **Known gap.** An `entry: {key}` counts as a required reference (`src/content/mod.rs:504-529`), so a ruling could pull
  an unselected entry into the campaign. Treat these targets as optional.

## Unlock lists

Research: a list of things a power buys with a tracker. The marker goes above the existing table.

```markdown
<!-- splat:unlocks key=engineering spend=rp buyers=power tiers=tech_tiers -->

| Node | Tier | Requires | Effect |
| --- | --- | --- | --- |
| E1 Tighter Tolerances | 1 | None | Failure range becomes 1–2. |
```

**Buy** chooses a buyer, shows the cost and any missing requirement, and writes the ledger. The DM can **Buy anyway**, and
can pay half for stolen plans. The Reader shows beside each node who has it. Start with "who has what" only, because the
`Effect` column already states the number.

**Known gaps:**

- "The first word is the key" fails on `Soul Jars` (`Tech Tree.md:96`, `:158`). The carrying table has no Tier or Requires
  column.
- Typed `sets` change a tracker that no quick action reads. A quick action's numbers are literal, so an old button gives the
  old result with no warning. Either let `add` read a tracker, or leave the number to the DM.

## Procedures

A typed, guided walk through one rule. It asks, rolls and proposes, and it writes nothing until the end. The design this
replaces used about 30 lines for one gate-trip case, and a DM walkthrough said it would not write them. Build this only if
quick actions and roll tables leave a slow step at the table. Gate trip is the test case.

**Known gaps:** the step schema, `key` on nested steps, what a later step reads after an earlier proposed change, mixed
stores of fuel, and `run` on a notice.

## Fuel by place, and percent changes

A tracker per store (home reliquary, a team's jar, a gate), so a total in the wrong place cannot look spendable. A `percent`
change for leaks: 10% of 101, rounded up, loses 11. It is worked out once and stored as a number.

**Known gaps:** no store creation at the table, no transfer action, and a displayed value that does not show open leak items.
The first dormant gate is the test. A daily leak also makes hundreds of due rows, so it needs batching.

## Voyage handling

**Run all in order**, same-hour order, a batch undo, and **Review** on periods that no longer match the clock. Today a voyage is
one Travel event and one **Next** per month. A mark for Home's calls also stops a voyage every 14 days on a ship that cannot
take calls, so it needs a way to pause one mark.

## Local time

```yaml
splat_world:
  type: planet
  local_time: {day_hours: 31, year_days: 412}
```

The planet's page shows "1 local day = 1.3 standard days". **Set local hour** records a reference, and the page then estimates
the hour from the clock, marked "about". For now, the planet's note says this in prose. This is display only, and it never moves
the clock.

## Smaller items

- **Marks on falling values**, for low fuel, supplies and reputation.
- **One-off reminders at the table:** "the duel is in 3 days", or a gate disabled for 1d4 days. A title and a duration from now,
  stored in the ledger.
- **`per` from world entries**, so instance values show on an entry's page, and a new instance needs no note edit.
- **Run trackers**, if a tracker's value staying after a run becomes a problem.
- **Saved ticks** on a notice's checklist, if text and **Done** do not survive interrupted bells.
- **`why` on a quick action**, if a note line is too weak for seal sources.
- **NPC link in a run**, with **Mark dead** on the ended view. Today the side panel uses a name match.
- **Per-campaign NPC status in a shared package**, through the ledger.
- **Starting a fight from an NPC** with no run open.
- **Inline controls** in prose, such as `` `splat:roll forced_trip` ``. The convention is in [packages](packages.md).
- **Value pairs**, such as who knows which gate address, or reputation per character per faction.
- **Hexes and routes.** `in` means "inside" and not "next to".

## Further out

- **Transcript help.** World names and aliases go to the LLM's cleaning pass as a word list. A marker can suggest a tracker change
  or a new NPC, and it takes effect only when the DM adds it.
- **Campaign simulation.** Trackers and procedures are typed data. The notes already quote results such as "42–57% by month 60".
- **A shared world.** A second campaign moves notes into a package, and each campaign keeps its own ledger. To inherit a death or a
  treaty, the DM edits the notes.
- **Time dilation.** Separate work, and it must not change the campaign clock.
