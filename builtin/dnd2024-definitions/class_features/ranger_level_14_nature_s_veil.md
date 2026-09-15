---
splat_schema: 1
splat_id: af35c9a0-4813-5351-87fd-ba4bb06ae5b8
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_14_nature_s_veil
splat_name: 'Level 14: Nature’s Veil'
splat_origin:
  key: dnd2024:reference/pages/059
splat_sources:
- title: SRD 5.2.1
  locator: pp. 59
  url: https://www.dndbeyond.com/srd
---

# Level 14: Nature’s Veil

## Source text

<!-- SRD page 59 -->

You invoke spirits of nature to magically hide your-\
self. As a Bonus Action, you can give yourself the\
Invisible condition until the end of your next turn.\
You can use this feature a number of times equal\
to your Wisdom modifier (minimum of once), and\
you regain all expended uses when you finish a\
Long Rest.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/ranger_level_14_nature_s_veil ` |
| ` implementation ` | ` dnd2024:class_features_ranger_level_14_nature_s_veil ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` bonus_action ` |
| ` condition ` | ` invisible ` |
| ` until ` | ` source.next_turn_end ` |

#### Uses

<!-- splat:yaml path=splat_rule/parameters/uses -->

```yaml
maximum:
  op: max
  args:
  - 1
  - stat: abilities.wisdom.modifier
recover:
  event: rest.long_rest_completed
  amount: all
```
