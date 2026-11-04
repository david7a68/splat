---
splat_schema: 2
splat_id: dd0edd67-e5d3-5013-955d-b1161908efdf
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_2_paladin_s_smite
splat_name: 'Level 2: Paladin’s Smite'
splat_origin: dnd2024:reference/pages/054
splat_sources:
- SRD 5.2.1, pp. 54, https://www.dndbeyond.com/srd
---

# Level 2: Paladin’s Smite

## Source text

<!-- SRD page 54 -->

You always have the Divine Smite spell prepared. In addition, you can cast it without expending a spell slot, but you must finish a Long Rest before you can cast it in this way again.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/paladin_level_2_paladin_s_smite ` |
| ` implementation ` | ` dnd2024:class_features_paladin_level_2_paladin_s_smite ` |

### Parameters

#### Always prepared

<!-- splat:rows path=splat_rule/parameters/always_prepared -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/divine_smite ` |

#### Free cast

<!-- splat:yaml path=splat_rule/parameters/free_cast -->

```yaml
spell:
  key: dnd2024:spells/divine_smite
spell_slot_cost: 0
uses:
  maximum: 1
  recover:
    event: rest.long_rest_completed
    amount: all
```
