---
splat_schema: 2
splat_id: 3cb6ef8d-c86d-5095-8559-a10f2dacd398
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_1_unarmored_defense
splat_name: 'Level 1: Unarmored Defense'
splat_origin: dnd2024:reference/pages/050
splat_sources:
- SRD 5.2.1, pp. 50, https://www.dndbeyond.com/srd
---

# Level 1: Unarmored Defense

## Source text

<!-- SRD page 50 -->

While you aren’t wearing armor or wielding a Shield, your base Armor Class equals 10 plus your Dexterity and Wisdom modifiers.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/monk_level_1_unarmored_defense ` |
| ` implementation ` | ` dnd2024:class_features_monk_level_1_unarmored_defense ` |

### Parameters

#### Requires

<!-- splat:values path=splat_rule/parameters/requires -->

| Field | Value |
| --- | --- |
| ` armor_worn ` | ` false ` |
| ` shield_wielded ` | ` false ` |

#### Base armor class

<!-- splat:yaml path=splat_rule/parameters/base_armor_class -->

```yaml
op: add
args:
- 10
- var: actor.abilities.dexterity.modifier
- var: actor.abilities.wisdom.modifier
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` monk ` |
| ` minimum_level ` | ` 1 ` |
