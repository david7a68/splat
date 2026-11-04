---
splat_schema: 2
splat_id: d3277324-ecf6-55cf-9a82-c019bc9e9372
splat_kind: rule
splat_key: dnd2024:class_features/wizard_level_20_signature_spells
splat_name: 'Level 20: Signature Spells'
splat_origin: dnd2024:reference/pages/079
splat_sources:
- SRD 5.2.1, pp. 79, https://www.dndbeyond.com/srd
---

# Level 20: Signature Spells

## Source text

<!-- SRD page 79 -->

Choose two level 3 spells in your spellbook as your signature spells. You always have these spells prepared, and you can cast each of them once at level 3 without expending a spell slot. When you do so, you can’t cast them in this way again until you finish a Short or Long Rest. To cast either spell at a higher level, you must expend a spell slot.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/wizard_level_20_signature_spells ` |
| ` implementation ` | ` dnd2024:class_features_wizard_level_20_signature_spells ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` always_prepared ` | ` true ` |
| ` counts_against_prepared_limit ` | ` false ` |

#### Choose spells

<!-- splat:values path=splat_rule/parameters/choose_spells -->

| Field | Value |
| --- | --- |
| ` count ` | ` 2 ` |
| ` level ` | ` 3 ` |
| ` in_spellbook ` | ` true ` |

#### Free cast

<!-- splat:values path=splat_rule/parameters/free_cast -->

| Field | Value |
| --- | --- |
| ` cost ` | ` 1 ` |
| ` cast_level ` | ` 3 ` |
| ` expend_slot ` | ` false ` |

##### Per spell resource

<!-- splat:values path=splat_rule/parameters/free_cast/per_spell_resource -->

| Field | Value |
| --- | --- |
| ` id ` | ` wizard.signature_spell ` |
| ` maximum ` | ` 1 ` |

###### Recovery

<!-- splat:table
path: splat_rule/parameters/free_cast/per_spell_resource/recovery
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Event:
    path: event
    type: string
  Amount:
    path: amount
    type: symbol
-->

| Key | Event | Amount |
| --- | --- | --- |
| ` row_1 ` | ` rest.short_completed ` | ` All ` |
| ` row_2 ` | ` rest.long_completed ` | ` All ` |

#### Higher level casting

<!-- splat:values path=splat_rule/parameters/higher_level_casting -->

| Field | Value |
| --- | --- |
| ` expend_slot ` | ` true ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` wizard ` |
| ` minimum_level ` | ` 20 ` |
