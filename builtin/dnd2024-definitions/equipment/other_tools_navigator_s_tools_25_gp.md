---
splat_schema: 1
splat_id: ad393109-a0dd-5a4e-b750-83ea5e55abc6
splat_kind: item
splat_key: dnd2024:equipment/other_tools_navigator_s_tools_25_gp
splat_name: Navigator’s Tools (25 GP)
splat_origin:
  key: dnd2024:reference/pages/094
splat_sources:
- title: SRD 5.2.1
  locator: pp. 94
  url: https://www.dndbeyond.com/srd
---

# Navigator’s Tools (25 GP)

## Source text

<!-- SRD page 94 -->

Ability: Wisdom   Weight: 2 lb.\
Utilize: Plot a course (DC 10), or determine position by\
stargazing (DC 15)\

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

### Price

<!-- splat:values path=splat_item/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 25 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 2 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/other_tools_navigator_s_tools_25_gp ` |
| ` implementation ` | ` dnd2024:equipment_other_tools_navigator_s_tools_25_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: wisdom
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: plot_course
  check:
    ability: wisdom
    dc: 10
- action: utilize
  operation: determine_position_by_stargazing
  check:
    ability: wisdom
    dc: 15
craftable: []
monster_proficient_if_tool_in_stat_block: true
proficiency_rule:
  key: dnd2024:rules/tools_tool_proficiency
crafting_rule:
  key: dnd2024:rules/crafting_nonmagical_items_tools
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 2 ` |
| ` unit ` | ` lb ` |
