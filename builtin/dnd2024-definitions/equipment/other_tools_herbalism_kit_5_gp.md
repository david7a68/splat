---
splat_schema: 1
splat_id: 521d7211-427e-54e2-a572-884bc3010e49
splat_kind: item
splat_key: dnd2024:equipment/other_tools_herbalism_kit_5_gp
splat_name: Herbalism Kit (5 GP)
splat_origin:
  key: dnd2024:reference/pages/094
splat_sources:
- title: SRD 5.2.1
  locator: pp. 94
  url: https://www.dndbeyond.com/srd
---

# Herbalism Kit (5 GP)

## Source text

<!-- SRD page 94 -->

Ability: Intelligence Weight: 3 lb.\
Utilize: Identify a plant (DC 10)\
Craft: Antitoxin, Candle, Healer’s Kit, Potion of Healing

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
| ` value ` | ` 5 ` |
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
| ` mechanic ` | ` dnd2024:equipment/other_tools_herbalism_kit_5_gp ` |
| ` implementation ` | ` dnd2024:equipment_other_tools_herbalism_kit_5_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: intelligence
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: identify_plant
  check:
    ability: intelligence
    dc: 10
craftable:
- key: dnd2024:equipment/antitoxin_50_gp
- key: dnd2024:equipment/candle_1_cp
- key: dnd2024:equipment/healer_s_kit_5_gp
- key: dnd2024:equipment/potion_of_healing_50_gp
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
