---
splat_schema: 1
splat_id: 0c6ccdcd-ebfb-580d-b367-214af7def135
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_alchemist_s_supplies_50_gp
splat_name: Alchemist’s Supplies (50 GP)
splat_origin:
  key: dnd2024:reference/pages/093
splat_sources:
- title: SRD 5.2.1
  locator: pp. 93
  url: https://www.dndbeyond.com/srd
---

# Alchemist’s Supplies (50 GP)

## Source text

<!-- SRD page 93 -->

Ability: Intelligence Weight: 8 lb.\
Utilize: Identify a substance (DC 15), or start a fire\
(DC 15)\
Craft: Acid, Alchemist’s Fire, Component Pouch, Oil,\
Paper, Perfume\

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
| ` value ` | ` 50 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 8 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/artisan_s_tools_alchemist_s_supplies_50_gp ` |
| ` implementation ` | ` dnd2024:equipment_artisan_s_tools_alchemist_s_supplies_50_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: intelligence
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: identify_substance
  check:
    ability: intelligence
    dc: 15
- action: utilize
  operation: ignite_fire
  check:
    ability: intelligence
    dc: 15
craftable:
- key: dnd2024:equipment/acid_25_gp
- key: dnd2024:equipment/alchemist_s_fire_50_gp
- key: dnd2024:equipment/component_pouch_25_gp
- key: dnd2024:equipment/oil_1_sp
- key: dnd2024:equipment/paper_2_sp
- key: dnd2024:equipment/perfume_5_gp
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
| ` value ` | ` 8 ` |
| ` unit ` | ` lb ` |
