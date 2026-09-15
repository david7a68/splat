---
splat_schema: 1
splat_id: fa9184fd-e422-593a-9aab-2fa65d6b36e5
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_5_faithful_steed
splat_name: 'Level 5: Faithful Steed'
splat_origin:
  key: dnd2024:reference/pages/055
splat_sources:
- title: SRD 5.2.1
  locator: pp. 55
  url: https://www.dndbeyond.com/srd
---

# Level 5: Faithful Steed

## Source text

<!-- SRD page 55 -->

You can call on the aid of an otherworldly steed. You\
always have the Find Steed spell prepared.\
You can also cast the spell once without expend-\
ing a spell slot, and you regain the ability to do so\
when you finish a Long Rest.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/paladin_level_5_faithful_steed ` |
| ` implementation ` | ` dnd2024:class_features_paladin_level_5_faithful_steed ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` counts_against_prepared_limit ` | ` false ` |

#### Always prepared

<!-- splat:rows path=splat_rule/parameters/always_prepared -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/find_steed ` |

#### Free cast

<!-- splat:yaml path=splat_rule/parameters/free_cast -->

```yaml
resource:
  id: paladin.faithful_steed
  maximum: 1
  recovery:
  - event: rest.long_completed
    amount: all
cost: 1
expend_slot: false
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` paladin ` |
| ` minimum_level ` | ` 5 ` |
