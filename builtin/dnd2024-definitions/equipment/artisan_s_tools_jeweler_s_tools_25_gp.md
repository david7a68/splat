---
splat_schema: 1
splat_id: 59d7c58e-2545-588d-bfdf-13318952f1a4
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_jeweler_s_tools_25_gp
splat_name: Jeweler’s Tools (25 GP)
splat_origin:
  key: dnd2024:reference/pages/093
splat_sources:
- title: SRD 5.2.1
  locator: pp. 93
  url: https://www.dndbeyond.com/srd
---

# Jeweler’s Tools (25 GP)

## Source text

<!-- SRD page 93 -->

Ability: Intelligence Weight: 2 lb.\
Utilize: Discern a gem’s value (DC 15)\
Craft: Arcane Focus, Holy Symbol\

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
| ` mechanic ` | ` dnd2024:equipment/artisan_s_tools_jeweler_s_tools_25_gp ` |
| ` implementation ` | ` dnd2024:equipment_artisan_s_tools_jeweler_s_tools_25_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: intelligence
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: appraise_gem
  check:
    ability: intelligence
    dc: 15
craftable:
- key: dnd2024:equipment/arcane_focus_varies
- key: dnd2024:equipment/holy_symbol_varies
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
