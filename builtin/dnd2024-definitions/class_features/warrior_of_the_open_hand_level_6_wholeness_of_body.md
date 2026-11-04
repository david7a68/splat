---
splat_schema: 2
splat_id: 69d6f033-3559-59a4-805c-875a03cca778
splat_kind: rule
splat_key: dnd2024:class_features/warrior_of_the_open_hand_level_6_wholeness_of_body
splat_name: 'Level 6: Wholeness of Body'
splat_origin: dnd2024:reference/pages/052
splat_sources:
- SRD 5.2.1, pp. 52, https://www.dndbeyond.com/srd
---

# Level 6: Wholeness of Body

## Source text

<!-- SRD page 52 -->

You gain the ability to heal yourself. As a Bonus Action, you can roll your Martial Arts die. You regain a number of Hit Points equal to the number rolled plus your Wisdom modifier (minimum of 1 Hit Point regained).

You can use this feature a number of times equal to your Wisdom modifier (minimum of once), and you regain all expended uses when you finish a Long Rest.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/warrior_of_the_open_hand_level_6_wholeness_of_body ` |
| ` implementation ` | ` dnd2024:class_features_warrior_of_the_open_hand_level_6_wholeness_of_body ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` bonus_action ` |

#### Heal self

<!-- splat:yaml path=splat_rule/parameters/heal_self -->

```yaml
op: max
args:
- 1
- op: add
  args:
  - dice:
      count: 1
      sides:
        stat: martial_arts_die.sides
  - stat: abilities.wisdom.modifier
```

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
