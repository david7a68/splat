---
splat_schema: 1
splat_id: 8f3a0f54-5093-5b76-bec2-1c9296d2b8ea
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_9_indomitable
splat_name: 'Level 9: Indomitable'
splat_origin:
  key: dnd2024:reference/pages/048
splat_sources:
- title: SRD 5.2.1
  locator: pp. 48
  url: https://www.dndbeyond.com/srd
---

# Level 9: Indomitable

## Source text

<!-- SRD page 48 -->

If you fail a saving throw, you can reroll it with a\
bonus equal to your Fighter level. You must use the\
new roll, and you can’t use this feature again until\
you finish a Long Rest.\
You can use this feature twice before a Long Rest\
starting at level 13 and three times before a Long\
Rest starting at level 17.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/fighter_level_9_indomitable ` |
| ` implementation ` | ` dnd2024:class_features_fighter_level_9_indomitable ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` self_saving_throw_failed ` |
| ` optional_reroll ` | ` true ` |
| ` must_use_new_roll ` | ` true ` |

#### Reroll bonus

<!-- splat:values path=splat_rule/parameters/reroll_bonus -->

| Field | Value |
| --- | --- |
| ` stat ` | ` classes.fighter.level ` |

#### Uses

<!-- splat:yaml path=splat_rule/parameters/uses -->

```yaml
maximum_by_fighter_level:
  '9': 1
  '13': 2
  '17': 3
recover:
  event: rest.long_rest_completed
  amount: all_expended
```
