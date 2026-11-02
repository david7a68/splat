---
splat_schema: 1
splat_id: 1391bf67-98a7-54ef-ae03-29cb242ada2c
splat_kind: rule
splat_key: dnd2024:rules/combat_moving_around_other_creatures
splat_name: Moving around Other Creatures
splat_origin:
  key: dnd2024:reference/pages/014
splat_sources:
- title: SRD 5.2.1
  locator: pp. 14
  url: https://www.dndbeyond.com/srd
---

# Moving around Other Creatures

## Source text

<!-- SRD page 14 -->

During your move, you can pass through the space of an ally, a creature that has the Incapacitated condition (see “Rules Glossary”), a Tiny creature, or a creature that is two sizes larger or smaller than you.

Another creature’s space is Difficult Terrain for you unless that creature is Tiny or your ally.

You can’t willingly end a move in a space occupied by another creature. If you somehow end a turn in a space with another creature, you have the Prone condition (see “Rules Glossary”) unless you are Tiny or are of a larger size than the other creature.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_moving_around_other_creatures ` |
| ` implementation ` | ` dnd2024:rules_combat_moving_around_other_creatures ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` willing_end_move_in_occupied_space ` | ` false ` |

#### Pass through

<!-- splat:yaml path=splat_rule/parameters/pass_through -->

```yaml
any:
- ally
- occupant_incapacitated
- occupant_tiny
- absolute_size_category_difference_minimum: 2
```

#### Difficult terrain

<!-- splat:values path=splat_rule/parameters/difficult_terrain -->

| Field | Value |
| --- | --- |
| ` unless_any ` | ` ["occupant_tiny", "ally"] ` |

#### Occupied space turn end

<!-- splat:values path=splat_rule/parameters/occupied_space_turn_end -->

| Field | Value |
| --- | --- |
| ` condition ` | ` prone ` |
| ` unless_any ` | ` ["mover_tiny", "mover_larger_than_occupant"] ` |
