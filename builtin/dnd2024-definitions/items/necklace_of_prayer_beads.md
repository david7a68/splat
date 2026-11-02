---
splat_schema: 1
splat_id: 0a38a9c2-4dbf-5f8b-8ed9-1ea2346a6e28
splat_kind: item
splat_key: dnd2024:items/necklace_of_prayer_beads
splat_name: Necklace of Prayer Beads
splat_origin:
  key: dnd2024:reference/magic_item/necklace_of_prayer_beads
splat_sources:
- title: SRD 5.2.1
  locator: pp. 233
  url: https://www.dndbeyond.com/srd
---

# Necklace of Prayer Beads

## Source text

<!-- SRD page 233 -->

Wondrous Item, Rare (Requires Attunement by a Cleric, Druid, or Paladin)

This necklace has 1d4 + 2 magic beads made from aquamarine, black pearl, or topaz. It also has many nonmagical beads made from stones such as amber, bloodstone, citrine, coral, jade, pearl, or quartz. If a magic bead is removed from the necklace, that bead loses its magic.

Six types of magic beads exist. The GM decides the type of each bead on the necklace or determines it randomly by rolling on the table below. A necklace can have more than one bead of the same type. To use one, you must be wearing the necklace. Each bead contains a spell that you can cast from it as a Bonus Action (using your spell save DC if a save is necessary). Once a magic bead’s spell is cast, that bead can’t be used again until the next dawn.

| 1d20 | Bead | Spell |
| --- | --- | --- |
| 1–6 | Bead of Blessing | Bless |
| 7–12 | Bead of Curing | Cure Wounds (level 2 version) |
| 13–16 | Bead of Favor | Greater Restoration |
| 17–18 | Bead of Smiting | Shining Smite |
| 19 | Bead of Summons | Guardian of Faith |
| 20 | Bead of Wind Walking | Wind Walk |

Original layout: [[../../srd-5.2.1/pages/233|page 233]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |
| ` attunement_requirement_text ` | ` a Cleric, Druid, or Paladin ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/necklace_of_prayer_beads ` |
| ` implementation ` | ` dnd2024:item_necklace_of_prayer_beads ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` attunement_classes ` | ` ["cleric", "druid", "paladin"] ` |
| ` removed_magic_bead ` | ` nonmagical ` |
| ` bead_type_selection ` | ` gm_choice_or_roll ` |
| ` duplicate_types_allowed ` | ` true ` |
| ` bead_die ` | ` 20 ` |
| ` attunement_required ` | ` true ` |

#### Magic bead count

<!-- splat:yaml path=splat_rule/parameters/magic_bead_count -->

```yaml
roll:
  count: 1
  sides: 4
  add: 2
```

#### Beads

<!-- splat:table
path: splat_rule/parameters/beads
key: Name
order:
- blessing
- curing
- favor
- smiting
- summons
- wind_walking
columns:
  Name:
    path: name
    type: symbol
  Roll:
    path: roll
    type: yaml
  Cast spell:
    path: cast/spell
    type: reference
  Cast action:
    path: cast/action
    type: symbol
  Cast save dc:
    path: cast/save_dc
    type: string
  Cast level:
    path: cast/level
    type: integer
  Uses maximum:
    path: uses/maximum
    type: integer
  Uses recovery event:
    path: uses/recovery_event
    type: string
  Usage pool:
    path: usage_pool
    type: symbol
-->

| Name | Roll | Cast spell | Cast action | Cast save dc | Cast level | Uses maximum | Uses recovery event | Usage pool |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` Blessing ` | ` [1, 6] ` | [Bless](<../spells/bless.md> "dnd2024:spells/bless") | ` Bonus action ` | ` user.spell_save_dc ` | ` 1 ` | ` 1 ` | ` time.dawn ` | ` Per bead ` |
| ` Curing ` | ` [7, 12] ` | [Cure Wounds](<../spells/cure_wounds.md> "dnd2024:spells/cure_wounds") | ` Bonus action ` | ` user.spell_save_dc ` | ` 2 ` | ` 1 ` | ` time.dawn ` | ` Per bead ` |
| ` Favor ` | ` [13, 16] ` | [Greater Restoration](<../spells/greater_restoration.md> "dnd2024:spells/greater_restoration") | ` Bonus action ` | ` user.spell_save_dc ` | ` 5 ` | ` 1 ` | ` time.dawn ` | ` Per bead ` |
| ` Smiting ` | ` [17, 18] ` | [Shining Smite](<../spells/shining_smite.md> "dnd2024:spells/shining_smite") | ` Bonus action ` | ` user.spell_save_dc ` | ` 2 ` | ` 1 ` | ` time.dawn ` | ` Per bead ` |
| ` Summons ` | ` [19, 19] ` | [Guardian of Faith](<../spells/guardian_of_faith.md> "dnd2024:spells/guardian_of_faith") | ` Bonus action ` | ` user.spell_save_dc ` | ` 4 ` | ` 1 ` | ` time.dawn ` | ` Per bead ` |
| ` Wind walking ` | ` [20, 20] ` | [Wind Walk](<../spells/wind_walk.md> "dnd2024:spells/wind_walk") | ` Bonus action ` | ` user.spell_save_dc ` | ` 6 ` | ` 1 ` | ` time.dawn ` | ` Per bead ` |
