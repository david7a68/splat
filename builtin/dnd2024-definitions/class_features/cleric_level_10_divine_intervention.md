---
splat_schema: 1
splat_id: e2d8b92f-cc35-512f-b6f9-1566b04c9e8c
splat_kind: rule
splat_key: dnd2024:class_features/cleric_level_10_divine_intervention
splat_name: 'Level 10: Divine Intervention'
splat_origin:
  key: dnd2024:reference/pages/038
splat_sources:
- title: SRD 5.2.1
  locator: pp. 38
  url: https://www.dndbeyond.com/srd
---

# Level 10: Divine Intervention

## Source text

<!-- SRD page 38 -->

You can call on your deity or pantheon to inter-\
vene on your behalf. As a Magic action, choose any\
Cleric spell of level 5 or lower that doesn’t require a\
Reaction to cast. As part of the same action, you cast\
that spell without expending a spell slot or needing\
Material components. You can’t use this feature\
again until you finish a Long Rest.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/cleric_level_10_divine_intervention ` |
| ` implementation ` | ` dnd2024:class_features_cleric_level_10_divine_intervention ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` cast_as_part_of_action ` | ` true ` |
| ` spell_slot_cost ` | ` 0 ` |
| ` material_components_required ` | ` false ` |

#### Spell choice

<!-- splat:values path=splat_rule/parameters/spell_choice -->

| Field | Value |
| --- | --- |
| ` list ` | ` cleric ` |
| ` maximum_level ` | ` 5 ` |
| ` exclude_casting_time ` | ` reaction ` |

#### Uses

<!-- splat:yaml path=splat_rule/parameters/uses -->

```yaml
maximum: 1
recover:
  event: rest.long_rest_completed
  amount: all
```
