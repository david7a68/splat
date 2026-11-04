---
splat_schema: 2
splat_id: fafc4274-e260-5db6-8ba8-9f6932139a27
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_1_second_wind
splat_name: 'Level 1: Second Wind'
splat_origin: dnd2024:reference/pages/048
splat_sources:
- SRD 5.2.1, pp. 48, https://www.dndbeyond.com/srd
---

# Level 1: Second Wind

## Source text

<!-- SRD page 48 -->

You have a limited well of physical and mental stamina that you can draw on. As a Bonus Action, you can use it to regain Hit Points equal to 1d10 plus your Fighter level.

You can use this feature twice. You regain one expended use when you finish a Short Rest, and you regain all expended uses when you finish a Long Rest.

When you reach certain Fighter levels, you gain more uses of this feature, as shown in the Second Wind column of the Fighter Features table.

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

<!-- splat:values path=splat_rule/parameters/heal_self -->

| Field | Value |
| --- | --- |
| ` op ` | ` add ` |

##### Args

<!-- splat:table
path: splat_rule/parameters/heal_self/args
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Dice count:
    path: dice/count
    type: integer
    optional: true
  Dice sides:
    path: dice/sides
    type: integer
    optional: true
  Stat:
    path: stat
    type: string
    optional: true
-->

| Key | Dice count | Dice sides | Stat |
| --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 10 ` |  |
| ` row_2 ` |  |  | ` classes.fighter.level ` |

#### Resource

<!-- splat:values path=splat_rule/parameters/resource -->

| Field | Value |
| --- | --- |
| ` cost ` | ` 1 ` |

##### Maximum by fighter level

<!-- splat:yaml path=splat_rule/parameters/resource/maximum_by_fighter_level -->

```yaml
'1': 2
'4': 3
'10': 4
```

##### Recover

<!-- splat:table
path: splat_rule/parameters/resource/recover
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
  Details:
    type: section
    optional: true
-->

| Key | Event | Details |
| --- | --- | --- |
| ` row_1 ` | ` rest.short_rest_completed ` | [Rules](#^splat-113f3e54-row_1 "section:row_1") |
| ` row_2 ` | ` rest.long_rest_completed ` | [Rules](#^splat-113f3e54-row_2 "section:row_2") |

###### Row 1

^splat-113f3e54-row_1

###### Amount

<!-- splat:yaml path=splat_rule/parameters/resource/recover/@row_1/amount -->

```yaml
1
```

###### Row 2

^splat-113f3e54-row_2

###### Amount

<!-- splat:yaml path=splat_rule/parameters/resource/recover/@row_2/amount -->

```yaml
all_expended
```
