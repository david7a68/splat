---
splat_schema: 1
splat_id: 6820a578-3015-57bd-9d12-ad2876ffa85b
splat_kind: rule
splat_key: dnd2024:class_features/wizard_level_1_ritual_adept
splat_name: 'Level 1: Ritual Adept'
splat_origin:
  key: dnd2024:reference/pages/078
splat_sources:
- title: SRD 5.2.1
  locator: pp. 78
  url: https://www.dndbeyond.com/srd
---

# Level 1: Ritual Adept

## Source text

<!-- SRD page 78 -->

You can cast any spell as a Ritual if that spell has the Ritual tag and the spell is in your spellbook. You needn’t have the spell prepared, but you must read from the book to cast a spell in this way.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/wizard_level_1_ritual_adept ` |
| ` implementation ` | ` dnd2024:class_features_wizard_level_1_ritual_adept ` |

### Parameters

#### Ritual casting

<!-- splat:yaml path=splat_rule/parameters/ritual_casting -->

```yaml
eligible:
  ritual_tag: true
  in_actor_spellbook: true
must_be_prepared: false
must_read_from_book: true
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` wizard ` |
| ` minimum_level ` | ` 1 ` |
