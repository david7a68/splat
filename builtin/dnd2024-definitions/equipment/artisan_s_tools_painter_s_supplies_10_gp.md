---
splat_schema: 1
splat_id: f9673160-cb1c-5e46-9c4e-eefcc9259355
splat_kind: item
splat_key: dnd2024:equipment/artisan_s_tools_painter_s_supplies_10_gp
splat_name: Painter’s Supplies (10 GP)
splat_origin:
  key: dnd2024:reference/pages/093
splat_sources:
- title: SRD 5.2.1
  locator: pp. 93
  url: https://www.dndbeyond.com/srd
---

# Painter’s Supplies (10 GP)

## Source text

<!-- SRD page 93 -->

Ability: Wisdom   Weight: 5 lb.\
Utilize: Paint a recognizable image of something you’ve\
seen (DC 10)\
Craft: Druidic Focus, Holy Symbol\

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
| ` mechanic ` | ` dnd2024:equipment/artisan_s_tools_painter_s_supplies_10_gp ` |
| ` implementation ` | ` dnd2024:equipment_artisan_s_tools_painter_s_supplies_10_gp ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: wisdom
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: paint_recognizable_image
  check:
    ability: wisdom
    dc: 10
  requires: subject_previously_seen
craftable:
- key: dnd2024:equipment/druidic_focus_varies
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
| ` value ` | ` 5 ` |
| ` unit ` | ` lb ` |
