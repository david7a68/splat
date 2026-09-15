---
splat_schema: 1
splat_id: 60fb432f-51b6-5ec4-aaeb-410e3bfc7dcb
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_potter_s_tools_10_gp
splat_name: Potter’s Tools (10 GP)
splat_origin:
  key: dnd2024:reference/pages/093
splat_sources:
- title: SRD 5.2.1
  locator: pp. 93
  url: https://www.dndbeyond.com/srd
---

# Potter’s Tools (10 GP)

## Source text

<!-- SRD page 93 -->

Ability: Intelligence Weight: 3 lb.\
Utilize: Discern what a ceramic object held in the past\
24 hours (DC 15)\
Craft: Jug, Lamp\

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
| ` value ` | ` 10 ` |
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
| ` mechanic ` | ` dnd2024:equipment/artisan_s_tools_potter_s_tools_10_gp ` |
| ` implementation ` | ` dnd2024:equipment_artisan_s_tools_potter_s_tools_10_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: intelligence
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: identify_previous_contents
  check:
    ability: intelligence
    dc: 15
  material: ceramic
  lookback:
    value: 24
    unit: hour
craftable:
- key: dnd2024:equipment/jug_2_cp
- key: dnd2024:equipment/lamp_5_sp
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
