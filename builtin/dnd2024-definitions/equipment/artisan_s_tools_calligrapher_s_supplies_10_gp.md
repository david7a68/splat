---
splat_schema: 1
splat_id: 21f16989-a8e1-5ad8-b1f0-0e98edd9cb8b
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_calligrapher_s_supplies_10_gp
splat_name: Calligrapher’s Supplies (10 GP)
splat_origin:
  key: dnd2024:reference/pages/093
splat_sources:
- title: SRD 5.2.1
  locator: pp. 93
  url: https://www.dndbeyond.com/srd
---

# Calligrapher’s Supplies (10 GP)

## Source text

<!-- SRD page 93 -->

Ability: Dexterity Weight: 5 lb.\
Utilize: Write text with impressive flourishes that guard against forgery (DC 15)\
Craft: Ink, Spell Scroll

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
| ` value ` | ` 5 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/artisan_s_tools_calligrapher_s_supplies_10_gp ` |
| ` implementation ` | ` dnd2024:equipment_artisan_s_tools_calligrapher_s_supplies_10_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: dexterity
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: write_forgery_resistant_flourishes
  check:
    ability: dexterity
    dc: 15
craftable:
- key: dnd2024:equipment/ink_10_gp
- key: dnd2024:equipment/spell_scroll_cantrip_30_gp_level_1_50_gp
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
| ` value ` | ` 5 ` |
| ` unit ` | ` lb ` |
