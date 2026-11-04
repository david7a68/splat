---
splat_schema: 2
splat_id: 7e27ca57-e047-5c7c-a358-f31c2dae1961
splat_kind: item
splat_key: dnd2024:equipment/other_tools_gaming_set_varies
splat_name: Gaming Set (Varies)
splat_origin: dnd2024:reference/pages/094
splat_sources:
- SRD 5.2.1, pp. 94, https://www.dndbeyond.com/srd
---

# Gaming Set (Varies)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 94 -->

Ability: Wisdom Weight: —\
Utilize: Discern whether someone is cheating (DC 10), or win the game (DC 20)\
Variants: Dice (1 SP), dragonchess (1 GP), playing cards (5 SP), three-dragon ante (1 GP)





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/other_tools_gaming_set_varies ` |
| ` implementation ` | ` dnd2024:equipment_other_tools_gaming_set_varies ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: wisdom
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: detect_cheating
  check:
    ability: wisdom
    dc: 10
- action: utilize
  operation: win_game
  check:
    ability: wisdom
    dc: 20
craftable: []
monster_proficient_if_tool_in_stat_block: true
proficiency_rule:
  key: dnd2024:rules/tools_tool_proficiency
crafting_rule:
  key: dnd2024:rules/crafting_nonmagical_items_tools
```

#### Variants

<!-- splat:table
path: splat_rule/parameters/variants
key: Variant
order:
- dice
- dragonchess
- playing_cards
- three_dragon_ante
columns:
  Variant:
    path: id
    type: symbol
  Cost:
    path: price
    type: quantity
  Mass value:
    path: mass/value
    type: integer
  Mass unit:
    path: mass/unit
    type: symbol
  Mass negligible:
    path: mass/negligible
    type: boolean
-->

| Variant | Cost | Mass value | Mass unit | Mass negligible |
| --- | --- | --- | --- | --- |
| ` Dice ` | ` 1 SP ` | ` 0 ` | ` Lb ` | true |
| ` Dragonchess ` | ` 1 GP ` | ` 0 ` | ` Lb ` | true |
| ` Playing cards ` | ` 5 SP ` | ` 0 ` | ` Lb ` | true |
| ` Three dragon ante ` | ` 1 GP ` | ` 0 ` | ` Lb ` | true |
