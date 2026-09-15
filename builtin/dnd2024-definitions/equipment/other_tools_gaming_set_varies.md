---
splat_schema: 1
splat_id: 7e27ca57-e047-5c7c-a358-f31c2dae1961
splat_kind: item
splat_key: dnd2024:equipment/other_tools_gaming_set_varies
splat_name: Gaming Set (Varies)
splat_origin:
  key: dnd2024:reference/pages/094
splat_sources:
- title: SRD 5.2.1
  locator: pp. 94
  url: https://www.dndbeyond.com/srd
---

# Gaming Set (Varies)

## Source text

<!-- SRD page 94 -->

Ability: Wisdom   Weight: —\
Utilize: Discern whether someone is cheating (DC 10),\
or win the game (DC 20)\
Variants: Dice (1 SP), dragonchess (1 GP), playing cards\
(5 SP), three-dragon ante (1 GP)\

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` adventuring_gear ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

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

<!-- splat:rows path=splat_rule/parameters/variants -->

| Row | id |
| --- | --- |
| ` 0 ` | ` dice ` |
| ` 1 ` | ` dragonchess ` |
| ` 2 ` | ` playing_cards ` |
| ` 3 ` | ` three_dragon_ante ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/0/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` sp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/0/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 0 ` |
| ` unit ` | ` lb ` |
| ` negligible ` | ` true ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/1/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` gp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/1/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 0 ` |
| ` unit ` | ` lb ` |
| ` negligible ` | ` true ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/2/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 5 ` |
| ` unit ` | ` sp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/2/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 0 ` |
| ` unit ` | ` lb ` |
| ` negligible ` | ` true ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/3/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` gp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/3/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 0 ` |
| ` unit ` | ` lb ` |
| ` negligible ` | ` true ` |
