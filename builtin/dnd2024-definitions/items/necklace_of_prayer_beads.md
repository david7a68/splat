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

Wondrous Item, Rare (Requires Attunement by a\
Cleric, Druid, or Paladin)\
This necklace has 1d4 + 2 magic beads made from\
aquamarine, black pearl, or topaz. It also has many\
nonmagical beads made from stones such as amber,\
bloodstone, citrine, coral, jade, pearl, or quartz. If a\
magic bead is removed from the necklace, that bead\
loses its magic.\
Six types of magic beads exist. The GM decides the\
type of each bead on the necklace or determines it\
randomly by rolling on the table below. A necklace\
can have more than one bead of the same type. To\
use one, you must be wearing the necklace. Each\
bead contains a spell that you can cast from it as a\
Bonus Action (using your spell save DC if a save is\
necessary). Once a magic bead’s spell is cast, that\
bead can’t be used again until the next dawn.\
1d20 Bead Spell\
1–6 Bead of Blessing Bless\
7–12 Bead of Curing Cure Wounds (level 2\
version)\
13–16 Bead of Favor Greater Restoration\
17–18 Bead of Smiting Shining Smite\
19 Bead of Summons Guardian of Faith\
20 Bead of Wind Wind Walk\
Walking\

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

<!-- splat:rows path=splat_rule/parameters/beads -->

| Row | roll | name | usage_pool |
| --- | --- | --- | --- |
| ` 0 ` | ` [1, 6] ` | ` blessing ` | ` per_bead ` |
| ` 1 ` | ` [7, 12] ` | ` curing ` | ` per_bead ` |
| ` 2 ` | ` [13, 16] ` | ` favor ` | ` per_bead ` |
| ` 3 ` | ` [17, 18] ` | ` smiting ` | ` per_bead ` |
| ` 4 ` | ` [19, 19] ` | ` summons ` | ` per_bead ` |
| ` 5 ` | ` [20, 20] ` | ` wind_walking ` | ` per_bead ` |

##### Cast

<!-- splat:yaml path=splat_rule/parameters/beads/0/cast -->

```yaml
spell:
  key: dnd2024:spells/bless
action: bonus_action
save_dc: user.spell_save_dc
level: 1
```

##### Uses

<!-- splat:values path=splat_rule/parameters/beads/0/uses -->

| Field | Value |
| --- | --- |
| ` maximum ` | ` 1 ` |
| ` recovery_event ` | ` time.dawn ` |

##### Cast

<!-- splat:yaml path=splat_rule/parameters/beads/1/cast -->

```yaml
spell:
  key: dnd2024:spells/cure_wounds
action: bonus_action
save_dc: user.spell_save_dc
level: 2
```

##### Uses

<!-- splat:values path=splat_rule/parameters/beads/1/uses -->

| Field | Value |
| --- | --- |
| ` maximum ` | ` 1 ` |
| ` recovery_event ` | ` time.dawn ` |

##### Cast

<!-- splat:yaml path=splat_rule/parameters/beads/2/cast -->

```yaml
spell:
  key: dnd2024:spells/greater_restoration
action: bonus_action
save_dc: user.spell_save_dc
level: 5
```

##### Uses

<!-- splat:values path=splat_rule/parameters/beads/2/uses -->

| Field | Value |
| --- | --- |
| ` maximum ` | ` 1 ` |
| ` recovery_event ` | ` time.dawn ` |

##### Cast

<!-- splat:yaml path=splat_rule/parameters/beads/3/cast -->

```yaml
spell:
  key: dnd2024:spells/shining_smite
action: bonus_action
save_dc: user.spell_save_dc
level: 2
```

##### Uses

<!-- splat:values path=splat_rule/parameters/beads/3/uses -->

| Field | Value |
| --- | --- |
| ` maximum ` | ` 1 ` |
| ` recovery_event ` | ` time.dawn ` |

##### Cast

<!-- splat:yaml path=splat_rule/parameters/beads/4/cast -->

```yaml
spell:
  key: dnd2024:spells/guardian_of_faith
action: bonus_action
save_dc: user.spell_save_dc
level: 4
```

##### Uses

<!-- splat:values path=splat_rule/parameters/beads/4/uses -->

| Field | Value |
| --- | --- |
| ` maximum ` | ` 1 ` |
| ` recovery_event ` | ` time.dawn ` |

##### Cast

<!-- splat:yaml path=splat_rule/parameters/beads/5/cast -->

```yaml
spell:
  key: dnd2024:spells/wind_walk
action: bonus_action
save_dc: user.spell_save_dc
level: 6
```

##### Uses

<!-- splat:values path=splat_rule/parameters/beads/5/uses -->

| Field | Value |
| --- | --- |
| ` maximum ` | ` 1 ` |
| ` recovery_event ` | ` time.dawn ` |
