---
splat_schema: 1
splat_id: 0be75d37-c22a-5cda-a7cf-bdae9ce4f67d
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_cartographer_s_tools_15_gp
splat_name: Cartographer’s Tools (15 GP)
splat_origin:
  key: dnd2024:reference/pages/093
splat_sources:
- title: SRD 5.2.1
  locator: pp. 93
  url: https://www.dndbeyond.com/srd
---

# Cartographer’s Tools (15 GP)

## Source text

<!-- SRD page 93 -->

Ability: Wisdom Weight: 6 lb.\
Utilize: Draft a map of a small area (DC 15)\
Craft: Map

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
| ` value ` | ` 15 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 6 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/artisan_s_tools_cartographer_s_tools_15_gp ` |
| ` implementation ` | ` dnd2024:equipment_artisan_s_tools_cartographer_s_tools_15_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: wisdom
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: draft_map
  check:
    ability: wisdom
    dc: 15
  area_size: small
craftable:
- key: dnd2024:equipment/map_1_gp
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
| ` value ` | ` 6 ` |
| ` unit ` | ` lb ` |
