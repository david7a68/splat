---
splat_schema: 1
splat_id: 06649811-baac-5864-9a11-e2a7d49c340c
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_brewer_s_supplies_20_gp
splat_name: Brewer’s Supplies (20 GP)
splat_origin:
  key: dnd2024:reference/pages/093
splat_sources:
- title: SRD 5.2.1
  locator: pp. 93
  url: https://www.dndbeyond.com/srd
---

# Brewer’s Supplies (20 GP)

## Source text

<!-- SRD page 93 -->

Ability: Intelligence Weight: 9 lb.\
Utilize: Detect poisoned drink (DC 15), or identify alco-\
hol (DC 10)\
Craft: Antitoxin\

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
| ` value ` | ` 20 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 9 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/artisan_s_tools_brewer_s_supplies_20_gp ` |
| ` implementation ` | ` dnd2024:equipment_artisan_s_tools_brewer_s_supplies_20_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: intelligence
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: detect_poisoned_drink
  check:
    ability: intelligence
    dc: 15
- action: utilize
  operation: identify_alcohol
  check:
    ability: intelligence
    dc: 10
craftable:
- key: dnd2024:equipment/antitoxin_50_gp
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
| ` value ` | ` 9 ` |
| ` unit ` | ` lb ` |
