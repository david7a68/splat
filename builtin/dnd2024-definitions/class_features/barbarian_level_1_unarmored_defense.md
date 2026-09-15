---
splat_schema: 1
splat_id: 01354f26-fae4-5485-9c32-4dec75ed9e20
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_1_unarmored_defense
splat_name: 'Level 1: Unarmored Defense'
splat_origin:
  key: dnd2024:reference/pages/029
splat_sources:
- title: SRD 5.2.1
  locator: pp. 29
  url: https://www.dndbeyond.com/srd
---

# Level 1: Unarmored Defense

## Source text

<!-- SRD page 29 -->

While you aren’t wearing any armor, your base Ar-\
mor Class equals 10 plus your Dexterity and Consti-\
tution modifiers. You can use a Shield and still gain\
this benefit.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/barbarian_level_1_unarmored_defense ` |
| ` implementation ` | ` dnd2024:class_features_barbarian_level_1_unarmored_defense ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` shield_permitted ` | ` true ` |

#### When

<!-- splat:values path=splat_rule/parameters/when -->

| Field | Value |
| --- | --- |
| ` armor_worn ` | ` false ` |

#### Base ac

<!-- splat:yaml path=splat_rule/parameters/base_ac -->

```yaml
op: add
args:
- 10
- stat: abilities.dexterity.modifier
- stat: abilities.constitution.modifier
```
