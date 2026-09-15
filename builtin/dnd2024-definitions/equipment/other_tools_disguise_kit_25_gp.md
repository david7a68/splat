---
splat_schema: 1
splat_id: ae870cda-c505-58dc-a490-0803e3e55015
splat_kind: item
splat_key: dnd2024:equipment/other_tools_disguise_kit_25_gp
splat_name: Disguise Kit (25 GP)
splat_origin:
  key: dnd2024:reference/pages/094
splat_sources:
- title: SRD 5.2.1
  locator: pp. 94
  url: https://www.dndbeyond.com/srd
---

# Disguise Kit (25 GP)

## Source text

<!-- SRD page 94 -->

Ability:  Charisma  Weight: 3 lb.\
Utilize: Apply makeup (DC 10)\
Craft: Costume\

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
| ` value ` | ` 3 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/other_tools_disguise_kit_25_gp ` |
| ` implementation ` | ` dnd2024:equipment_other_tools_disguise_kit_25_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: charisma
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: apply_makeup
  check:
    ability: charisma
    dc: 10
craftable:
- key: dnd2024:equipment/costume_5_gp
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
| ` value ` | ` 3 ` |
| ` unit ` | ` lb ` |
