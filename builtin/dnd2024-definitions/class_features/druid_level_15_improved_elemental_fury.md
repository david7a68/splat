---
splat_schema: 1
splat_id: 51f2a3e4-a78f-5c14-8f17-5186074a5ebe
splat_kind: rule
splat_key: dnd2024:class_features/druid_level_15_improved_elemental_fury
splat_name: 'Level 15: Improved Elemental Fury'
splat_origin:
  key: dnd2024:reference/pages/043
splat_sources:
- title: SRD 5.2.1
  locator: pp. 43
  url: https://www.dndbeyond.com/srd
---

# Level 15: Improved Elemental Fury

## Source text

<!-- SRD page 43 -->

The option you chose for Elemental Fury grows\
more powerful, as detailed below.\
Potent Spellcasting. When you cast a Druid can-\
trip with a range of 10 feet or greater, the spell’s\
range increases by 300 feet.\
Primal Strike. The extra damage of your Primal\
Strike increases to 2d8.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/druid_level_15_improved_elemental_fury ` |
| ` implementation ` | ` dnd2024:class_features_druid_level_15_improved_elemental_fury ` |

### Parameters

#### By elemental fury choice

<!-- splat:yaml path=splat_rule/parameters/by_elemental_fury_choice -->

```yaml
potent_spellcasting:
  spell_class: druid
  spell_level: 0
  minimum_base_range_feet: 10
  range_bonus_feet: 300
primal_strike:
  replace_extra_damage:
    dice:
      count: 2
      sides: 8
```
