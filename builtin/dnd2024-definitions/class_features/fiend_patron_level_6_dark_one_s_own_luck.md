---
splat_schema: 2
splat_id: ce9081a6-0245-5d7f-8d29-be9a3f5f2d46
splat_kind: rule
splat_key: dnd2024:class_features/fiend_patron_level_6_dark_one_s_own_luck
splat_name: 'Level 6: Dark One’s Own Luck'
splat_origin: dnd2024:reference/pages/076
splat_sources:
- SRD 5.2.1, pp. 76, https://www.dndbeyond.com/srd
---

# Level 6: Dark One’s Own Luck

## Source text

<!-- SRD page 76 -->

You can call on your fiendish patron to alter fate in your favor. When you make an ability check or a saving throw, you can use this feature to add 1d10 to your roll. You can do so after seeing the roll but before any of the roll’s effects occur.

You can use this feature a number of times equal to your Charisma modifier (minimum of once), but you can use it no more than once per roll. You regain all expended uses when you finish a Long Rest.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/fiend_patron_level_6_dark_one_s_own_luck ` |
| ` implementation ` | ` dnd2024:class_features_fiend_patron_level_6_dark_one_s_own_luck ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` roll_types ` | ` ["ability_check", "saving_throw"] ` |
| ` timing ` | ` after_roll_before_effects ` |
| ` maximum_uses_per_roll ` | ` 1 ` |

#### Optional bonus

<!-- splat:yaml path=splat_rule/parameters/optional_bonus -->

```yaml
dice:
  count: 1
  sides: 10
```

#### Uses

<!-- splat:yaml path=splat_rule/parameters/uses -->

```yaml
maximum:
  op: max
  args:
  - 1
  - stat: abilities.charisma.modifier
recover:
  event: rest.long_rest_completed
  amount: all
```
