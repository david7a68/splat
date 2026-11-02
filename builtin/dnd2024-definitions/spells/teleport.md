---
splat_schema: 1
splat_id: 9819640f-61b7-5f9b-8573-b391e88f1193
splat_kind: spell
splat_key: dnd2024:spells/teleport
splat_name: Teleport
splat_origin:
  key: dnd2024:reference/spell/teleport
splat_sources:
- title: SRD 5.2.1
  locator: pp. 168
  url: https://www.dndbeyond.com/srd
---

# Teleport

## Source text

<!-- SRD page 168 -->

Level 7 Conjuration (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 10 feet\
Components: V\
Duration: Instantaneous

This spell instantly transports you and up to eight willing creatures that you can see within range, or a single object that you can see within range, to a destination you select. If you target an object, it must be Large or smaller, and it can’t be held or carried by an unwilling creature.

The destination you choose must be known to you, and it must be on the same plane of existence as you. Your familiarity with the destination determines whether you arrive there successfully. The GM rolls 1d100 and consults the Teleportation Outcome table and the explanations after it.

**Teleportation Outcome**

| Familiarity | Mishap | Similar Area | Off Target | On Target |
| --- | --- | --- | --- | --- |
| Permanent circle | — | — | — | 01–00 |
| Linked object | — | — | — | 01–00 |
| Very familiar | 01–05 | 06–13 | 14–24 | 25–00 |
| Seen casually | 01–33 | 34–43 | 44–53 | 54–00 |
| Viewed once or described | 01–43 | 44–53 | 54–73 | 74–00 |
| False destination | 01–50 | 51–00 | — | — |

Familiarity. Here are the meanings of the terms in the table’s Familiarity column:

•  “Permanent circle” means a permanent teleportation circle whose sigil sequence you know.

•  “Linked object” means you possess an object taken from the desired destination within the last six months, such as a book from a wizard’s library.

•  “Very familiar” is a place you have visited often, a place you have carefully studied, or a place you can see when you cast the spell.

•  “Seen casually” is a place you have seen more than once but with which you aren’t very familiar.

•  “Viewed once or described” is a place you have seen once, possibly using magic, or a place you know through someone else’s description, perhaps from a map.

•  “False destination” is a place that doesn’t exist. Perhaps you tried to scry an enemy’s sanctum but instead viewed an illusion, or you are attempting to teleport to a location that no longer exists.

Mishap. The spell’s unpredictable magic results in a difficult journey. Each teleporting creature (or the target object) takes 3d10 Force damage, and the GM rerolls on the table to see where you wind up (multiple mishaps can occur, dealing damage each time).

Similar Area. You and your group (or the target object) appear in a different area that’s visually or thematically similar to the target area. You appear in the closest similar place. If you are heading for your home laboratory, for example, you might appear in another person’s laboratory in the same city.

Off Target. You and your group (or the target object) appear 2d12 miles away from the destination in a random direction. Roll 1d8 for the direction: 1, east; 2, southeast; 3, south; 4, southwest; 5, west; 6, northwest; 7, north; or 8, northeast.

On Target. You and your group (or the target object) appear where you intended.

Original layout: [[../../srd-5.2.1/pages/168|page 168]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 7 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["bard", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 10 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/teleport ` |
| ` implementation ` | ` dnd2024:spell_teleport ` |

### Parameters

#### Targets

<!-- splat:yaml path=splat_rule/parameters/targets -->

```yaml
modes:
  creatures:
    caster: true
    additional_maximum: 8
    willing: true
    visible: true
    within_spell_range: true
  object:
    count: 1
    visible: true
    within_spell_range: true
    maximum_size: large
    held_or_carried_by_unwilling_creature: false
    caster_transported: false
```

#### Destination

<!-- splat:values path=splat_rule/parameters/destination -->

| Field | Value |
| --- | --- |
| ` known_to_caster ` | ` true ` |
| ` same_plane_as_caster ` | ` true ` |

#### Familiarity

<!-- splat:yaml path=splat_rule/parameters/familiarity -->

```yaml
permanent_circle:
  known_sigil_sequence: true
linked_object:
  caster_possesses_object_from_destination: true
  taken_within_months: 6
very_familiar:
  any:
  - visited_often
  - carefully_studied
  - visible_at_cast
seen_casually:
  seen_more_than_once: true
  very_familiar: false
viewed_once_or_described:
  any:
  - seen_once
  - seen_once_magically
  - known_from_description
  - known_from_map
false_destination:
  exists: false
```

#### Outcome roll

<!-- splat:yaml path=splat_rule/parameters/outcome_roll -->

```yaml
roller: gm
dice:
  dice:
    count: 1
    sides: 100
    add: 0
tables:
  permanent_circle:
    on_target:
    - 1
    - 100
  linked_object:
    on_target:
    - 1
    - 100
  very_familiar:
    mishap:
    - 1
    - 5
    similar_area:
    - 6
    - 13
    off_target:
    - 14
    - 24
    on_target:
    - 25
    - 100
  seen_casually:
    mishap:
    - 1
    - 33
    similar_area:
    - 34
    - 43
    off_target:
    - 44
    - 53
    on_target:
    - 54
    - 100
  viewed_once_or_described:
    mishap:
    - 1
    - 43
    similar_area:
    - 44
    - 53
    off_target:
    - 54
    - 73
    on_target:
    - 74
    - 100
  false_destination:
    mishap:
    - 1
    - 50
    similar_area:
    - 51
    - 100
```

#### Outcomes

<!-- splat:yaml path=splat_rule/parameters/outcomes -->

```yaml
mishap:
  damage_to_each_teleporting_target:
    dice:
      count: 3
      sides: 10
      add: 0
    type: force
  reroll_same_table: true
  repeated_mishaps_each_deal_damage: true
similar_area:
  arrival: closest_visually_or_thematically_similar_area
  chooser: gm
  constraints:
  - closest
  - visually_or_thematically_similar
off_target:
  distance_miles:
    dice:
      count: 2
      sides: 12
      add: 0
  direction:
    dice:
      dice:
        count: 1
        sides: 8
        add: 0
    faces:
      '1': east
      '2': southeast
      '3': south
      '4': southwest
      '5': west
      '6': northwest
      '7': north
      '8': northeast
on_target:
  arrival: intended_destination
```
