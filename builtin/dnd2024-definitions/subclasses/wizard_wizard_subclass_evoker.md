---
splat_schema: 1
splat_id: 74c1cc78-2a18-533e-bc9f-7f2d3b211559
splat_kind: rule_group
splat_key: dnd2024:subclasses/wizard_wizard_subclass_evoker
splat_name: 'Wizard Subclass: Evoker'
splat_origin:
  key: dnd2024:reference/pages/082
splat_sources:
- title: SRD 5.2.1
  locator: pp. 82
  url: https://www.dndbeyond.com/srd
---

# Wizard Subclass: Evoker

## Source text

<!-- SRD page 82 -->

### Create Explosive Elemental Effects

Your studies focus on magic that creates powerful elemental effects such as bitter cold, searing flame, rolling thunder, crackling lightning, and burning acid. Some Evokers find employment in military forces, serving as artillery to blast armies from afar. Others use their power to protect others, while some seek their own gain.

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` subclass ` |

### Members

<!-- splat:rows path=splat_rule_group/members -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/evoker_level_10_empowered_evocation ` |
| ` 1 ` | ` dnd2024:class_features/evoker_level_14_overchannel ` |
| ` 2 ` | ` dnd2024:class_features/evoker_level_3_evocation_savant ` |
| ` 3 ` | ` dnd2024:class_features/evoker_level_3_potent_cantrip ` |
| ` 4 ` | ` dnd2024:class_features/evoker_level_6_sculpt_spells ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:subclasses/wizard_wizard_subclass_evoker ` |
| ` implementation ` | ` dnd2024:subclasses_wizard_wizard_subclass_evoker ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` wizard ` |
| ` subclass ` | ` evoker ` |

#### Feature grants

<!-- splat:table
path: splat_rule/parameters/feature_grants
key: Feature
order:
- evoker_level_10_empowered_evocation
- evoker_level_14_overchannel
- evoker_level_3_evocation_savant
- evoker_level_3_potent_cantrip
- evoker_level_6_sculpt_spells
columns:
  Feature:
    path: feature
    type: reference
  Minimum class level:
    path: minimum_class_level
    type: integer
-->

| Feature | Minimum class level |
| --- | --- |
| [Level 10: Empowered Evocation](<../class_features/evoker_level_10_empowered_evocation.md> "dnd2024:class_features/evoker_level_10_empowered_evocation") | ` 10 ` |
| [Level 14: Overchannel](<../class_features/evoker_level_14_overchannel.md> "dnd2024:class_features/evoker_level_14_overchannel") | ` 14 ` |
| [Level 3: Evocation Savant](<../class_features/evoker_level_3_evocation_savant.md> "dnd2024:class_features/evoker_level_3_evocation_savant") | ` 3 ` |
| [Level 3: Potent Cantrip](<../class_features/evoker_level_3_potent_cantrip.md> "dnd2024:class_features/evoker_level_3_potent_cantrip") | ` 3 ` |
| [Level 6: Sculpt Spells](<../class_features/evoker_level_6_sculpt_spells.md> "dnd2024:class_features/evoker_level_6_sculpt_spells") | ` 6 ` |
