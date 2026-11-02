# Example: the Heralds of the End Tournament

This is the acceptance test for part 3 of [World and house rules](../world-and-house-rules.md). It is a design
example and is not implemented, so nothing has validated it. To use it, copy the frontmatter below to the top of the
vault's `Tournament.md`.

Times, bells and rewards come from `Undercroft.md` and `Tournament.md`. Those notes are early drafts, so tune the
numbers there and here together.

```yaml
---
splat_schema: 1
splat_id: <a new UUID>
splat_kind: rule
splat_name: Tournament
splat_rule:
  mechanic: heralds:tournament
  mode: augment
  implementation: splat:house_rules
  parameters: {}
splat_members:
  trackers:
    race_clock:
      name: Elapsed race time
      start: 0
      goal: 120
      pin: true
      quick:
        - {label: Enter room, add: 10}
        - {label: Cistern or Sands, add: 20}
        - {label: Reach Kitchens seal, add: 10}
        - {label: Alcove rest, add: 10}
        - {label: Fight minute, add: 1}
        - label: Collect soul
          changes: [{add: 10}, {tracker: souls, add: 1}]
      marks:
        - at: 20
          say: Bell. A Brown Bear enters the Pit. Announce the standings. Roll each rival. Check d6 for wandering.
          tables: [rival_track, wander_early]
        - at: 40
          say: Bell. The Cistern floods. Announce the standings. Roll each rival. Check d6 for wandering.
          tables: [rival_track, wander_middle]
        - at: 60
          say: Bell. The Sands turn to mire. A Dire Wolf enters the Pit. Announce the standings. Roll each rival. Check d6 for wandering.
          tables: [rival_track, wander_middle]
        - at: 80
          say: Bell. The Prison floods, and anyone caged drowns. Announce the standings. Roll each rival. Check d6 for wandering.
          tables: [rival_track, wander_late]
        - at: 100
          say: Bell. The Pit overflows. Steam fills the Kitchens. An Owlbear enters the Ring. Announce the standings. Roll each rival. Check d6 for wandering.
          tables: [rival_track, wander_late]
        - at: 120
          say: Everything floods. Any team still inside is out of the race.
    favor:
      name: Crowd Favor
      start: 0
      min: 0
      pin: true
      quick:
        - {label: Crowd loves it, add: 1}
        - {label: Throw a gift, add: -1, table: gifts}
    souls:
      name: Souls in the jar
      start: 0
      min: 0
      pin: true
    seals:
      name: Seals
      per: [Party, Old Guard, Keyring, Convicts]
      start: 0
      min: 0
      pin: true
      quick:
        - {label: Take seal, add: 1}
        - {label: Lose seal, add: -1}
      marks:
        - {at: 1, say: First seal. The party reaches level 2. A rival uses its 1-2 seals group.}
        - {at: 3, say: Third seal. The party reaches level 3. A rival uses its 3 seals group.}
    members:
      name: Team members left
      per: [Old Guard, Keyring, Convicts]
      start: 3
      min: 0
      quick:
        - {label: Lose member, add: -1}
    activation:
      name: Practice Ring activation
      start: 0
      goal: 30
      pin: true
      quick:
        - {label: Touch controls, add: 1d6}
        - {label: Missed twice or Ring reset, set: 0}
      marks:
        - {at: 30, say: Warning round. It opens on the operator's next turn. Fire on the surge line.}
---
```

## Markers for `Undercroft Tables.md`

Put each marker directly above its table. The note needs no frontmatter.

| Table | Marker |
|---|---|
| Rival Track | `<!-- splat:rolls key=rival_track dice=1d6 -->` |
| Wandering, 0:00 to 0:39 | `<!-- splat:rolls key=wander_early dice=1d8 -->` |
| Wandering, 0:40 to 1:19 | `<!-- splat:rolls key=wander_middle dice=1d8 -->` |
| Wandering, 1:20 and later | `<!-- splat:rolls key=wander_late dice=1d8 -->` |
| Caches | `<!-- splat:rolls key=caches dice=1d8 -->` |
| Fallen Rivals | `<!-- splat:rolls key=fallen_rivals dice=1d6 -->` |
| Gifts | `<!-- splat:rolls key=gifts dice=1d6 -->` |

Change the Rival Track's `2 to 5` to `2–5`.

## Other setup

- In `Campaign.md`, add the note to the in-play list. **Make house rule note** does this by itself.

  ```yaml
  splat_in_play:
    - {id: <the same UUID>}
  ```

- Rival arrivals use the existing files in `Encounter Groups/`.

## What this leaves to the DM

- The map, the flood's effects, and who is standing where.
- The final sum: elapsed time − 15 × souls (up to 5) − 5 × Favor.
- Player levels in D&D Beyond.
- Note lines: seal sources on `seals`, "Coming for the party" on a rival, and the operator on `activation`.
- The d6 wandering check before a wandering table. The d6 can be rolled with the run's dice control or on the table.
- A rival whose team is out stays in the list. Its `members` value shows 0.
