---
splat_schema: 1
splat_id: fafc4274-e260-5db6-8ba8-9f6932139a27
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_1_second_wind
splat_name: 'Level 1: Second Wind'
splat_origin:
  key: dnd2024:reference/pages/048
splat_sources:
- title: SRD 5.2.1
  locator: pp. 48
  url: https://www.dndbeyond.com/srd
---

# Level 1: Second Wind

## Source text

<!-- SRD page 48 -->

You have a limited well of physical and mental\
stamina that you can draw on. As a Bonus Action,\
you can use it to regain Hit Points equal to 1d10\
plus your Fighter level.\
You can use this feature twice. You regain one\
expended use when you finish a Short Rest, and you\
regain all expended uses when you finish a Long\
Rest.\
When you reach certain Fighter levels, you gain\
more uses of this feature, as shown in the Second\
Wind column of the Fighter Features table.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/fighter_level_1_second_wind ` |
| ` implementation ` | ` dnd2024:class_features_fighter_level_1_second_wind ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` bonus_action ` |

#### Heal self

<!-- splat:yaml path=splat_rule/parameters/heal_self -->

```yaml
op: add
args:
- dice:
    count: 1
    sides: 10
- stat: classes.fighter.level
```

#### Resource

<!-- splat:yaml path=splat_rule/parameters/resource -->

```yaml
maximum_by_fighter_level:
  '1': 2
  '4': 3
  '10': 4
cost: 1
recover:
- event: rest.short_rest_completed
  amount: 1
- event: rest.long_rest_completed
  amount: all_expended
```
