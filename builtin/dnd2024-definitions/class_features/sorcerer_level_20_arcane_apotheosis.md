---
splat_schema: 2
splat_id: e5333c93-9a9d-59e5-978c-651643bcd39c
splat_kind: rule
splat_key: dnd2024:class_features/sorcerer_level_20_arcane_apotheosis
splat_name: 'Level 20: Arcane Apotheosis'
splat_origin: dnd2024:reference/pages/066
splat_sources:
- SRD 5.2.1, pp. 66, https://www.dndbeyond.com/srd
---

# Level 20: Arcane Apotheosis

## Source text

<!-- SRD page 66 -->

While your Innate Sorcery feature is active, you can use one Metamagic option on each of your turns without spending Sorcery Points on it.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/sorcerer_level_20_arcane_apotheosis ` |
| ` implementation ` | ` dnd2024:class_features_sorcerer_level_20_arcane_apotheosis ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` requires_innate_sorcery_active ` | ` true ` |

#### Free metamagic

<!-- splat:values path=splat_rule/parameters/free_metamagic -->

| Field | Value |
| --- | --- |
| ` maximum_options ` | ` 1 ` |
| ` period ` | ` each_own_turn ` |
| ` sorcery_point_cost ` | ` 0 ` |
