---
splat_schema: 2
splat_id: 0af4acc0-690b-54fe-9d36-1d97d04b31ea
splat_kind: rule
splat_key: dnd2024:class_features/sorcerer_level_2_font_of_magic
splat_name: 'Level 2: Font of Magic'
splat_origin: dnd2024:reference/pages/066
splat_sources:
- SRD 5.2.1, pp. 66, https://www.dndbeyond.com/srd
---

# Level 2: Font of Magic

## Source text

<!-- SRD page 66 -->

You can tap into the wellspring of magic within yourself. This wellspring is represented by Sorcery Points, which allow you to create a variety of magical effects.

You have 2 Sorcery Points, and you gain more as you reach higher levels, as shown in the Sorcery Points column of the Sorcerer Features table. You can’t have more Sorcery Points than the number shown in the table for your level. You regain all expended Sorcery Points when you finish a Long Rest.

You can use your Sorcery Points to fuel the options below, along with other features, such as Metamagic, that use those points.

Converting Spell Slots to Sorcery Points. You can expend a spell slot to gain a number of Sorcery Points equal to the slot’s level (no action required).

Creating Spell Slots. As a Bonus Action, you can transform unexpended Sorcery Points into one spell slot. The Creating Spell Slots table shows the cost of creating a spell slot of a given level, and it lists the minimum Sorcerer level you must be to create a slot. You can create a spell slot no higher than level 5.

Any spell slot you create with this feature vanishes when you finish a Long Rest.

**Creating Spell Slots**

| Spell Slot Level | Sorcery Point Cost | Min. Sorcerer Level |
| --- | --- | --- |
| 1 | 2 | 2 |
| 2 | 3 | 3 |
| 3 | 5 | 5 |
| 4 | 6 | 7 |
| 5 | 7 | 9 |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/sorcerer_level_2_font_of_magic ` |
| ` implementation ` | ` dnd2024:class_features_sorcerer_level_2_font_of_magic ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cannot_exceed_maximum ` | ` true ` |

#### Resource

<!-- splat:yaml path=splat_rule/parameters/resource -->

```yaml
id: sorcerer.sorcery_points
maximum:
  var: actor.class_levels.sorcerer
recovery:
- event: rest.long_completed
  amount: all
```

#### Slot to points

<!-- splat:yaml path=splat_rule/parameters/slot_to_points -->

```yaml
action: none
expend_slot_count: 1
restore_points:
  var: slot.level
```

#### Points to slot

<!-- splat:values path=splat_rule/parameters/points_to_slot -->

| Field | Value |
| --- | --- |
| ` action ` | ` bonus_action ` |
| ` create_slot_count ` | ` 1 ` |
| ` slot_expires ` | ` actor.rest.long_completed ` |

##### Options

<!-- splat:table
path: splat_rule/parameters/points_to_slot/options
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
columns:
  Key:
    type: key
  Slot level:
    path: slot_level
    type: integer
  Point cost:
    path: point_cost
    type: integer
  Minimum sorcerer level:
    path: minimum_sorcerer_level
    type: integer
-->

| Key | Slot level | Point cost | Minimum sorcerer level |
| --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 2 ` | ` 2 ` |
| ` row_2 ` | ` 2 ` | ` 3 ` | ` 3 ` |
| ` row_3 ` | ` 3 ` | ` 5 ` | ` 5 ` |
| ` row_4 ` | ` 4 ` | ` 6 ` | ` 7 ` |
| ` row_5 ` | ` 5 ` | ` 7 ` | ` 9 ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` sorcerer ` |
| ` minimum_level ` | ` 2 ` |
