---
splat_schema: 1
splat_id: 1ae96ba8-215a-5d00-96e2-94883ecaf10b
splat_kind: item
splat_key: dnd2024:equipment/other_tools_thieves_tools_25_gp
splat_name: Thieves’ Tools (25 GP)
splat_origin:
  key: dnd2024:reference/pages/094
splat_sources:
- title: SRD 5.2.1
  locator: pp. 94
  url: https://www.dndbeyond.com/srd
---

# Thieves’ Tools (25 GP)

## Source text

<!-- SRD page 94 -->

Ability: Dexterity Weight: 1 lb.\
Utilize: Pick a lock (DC 15), or disarm a trap (DC 15)

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
| ` value ` | ` 1 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/other_tools_thieves_tools_25_gp ` |
| ` implementation ` | ` dnd2024:equipment_other_tools_thieves_tools_25_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: dexterity
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: pick_lock
  check:
    ability: dexterity
    dc: 15
- action: utilize
  operation: disarm_trap
  check:
    ability: dexterity
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
| ` value ` | ` 1 ` |
| ` unit ` | ` lb ` |
