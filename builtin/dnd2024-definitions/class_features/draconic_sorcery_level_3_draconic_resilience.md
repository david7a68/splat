---
splat_schema: 1
splat_id: 86d68e1c-4938-51ba-b368-a497f0c1432c
splat_kind: rule
splat_key: dnd2024:class_features/draconic_sorcery_level_3_draconic_resilience
splat_name: 'Level 3: Draconic Resilience'
splat_origin:
  key: dnd2024:reference/pages/069
splat_sources:
- title: SRD 5.2.1
  locator: pp. 69
  url: https://www.dndbeyond.com/srd
---

# Level 3: Draconic Resilience

## Source text

<!-- SRD page 69 -->

The magic in your body manifests physical traits of your draconic gift. Your Hit Point maximum increases by 3, and it increases by 1 whenever you gain another Sorcerer level.

Parts of you are also covered by dragon-like scales. While you aren’t wearing armor, your base Armor Class equals 10 plus your Dexterity and Charisma modifiers.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/draconic_sorcery_level_3_draconic_resilience ` |
| ` implementation ` | ` dnd2024:class_features_draconic_sorcery_level_3_draconic_resilience ` |

### Parameters

#### Hit point maximum bonus

<!-- splat:values path=splat_rule/parameters/hit_point_maximum_bonus -->

| Field | Value |
| --- | --- |
| ` at_sorcerer_level_3 ` | ` 3 ` |
| ` per_subsequent_sorcerer_level ` | ` 1 ` |

#### Base ac

<!-- splat:yaml path=splat_rule/parameters/base_ac -->

```yaml
when_armor_worn: false
value:
  op: add
  args:
  - 10
  - stat: abilities.dexterity.modifier
  - stat: abilities.charisma.modifier
```
