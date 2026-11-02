---
splat_schema: 1
splat_id: 5ed175ce-1faa-5182-8c02-eab4dcf4638d
splat_kind: rule
splat_key: dnd2024:rules/mounts_and_vehicles_saddles
splat_name: Saddles
splat_origin:
  key: dnd2024:reference/pages/100
splat_sources:
- title: SRD 5.2.1
  locator: pp. 100
  url: https://www.dndbeyond.com/srd
---

# Saddles

## Source text

<!-- SRD page 100 -->

A saddle comes with a bit, a bridle, reins, and any other equipment needed to use the saddle. A Military Saddle gives Advantage on any ability check you make to remain mounted. An Exotic Saddle is required for riding an aquatic or a flying mount.

**Mounts and Other Animals**

| Item | Carrying Capacity | Cost |
| --- | --- | --- |
| Camel | 450 lb. | 50 GP |
| Elephant | 1,320 lb. | 200 GP |
| Horse, Draft | 540 lb. | 50 GP |
| Horse, Riding | 480 lb. | 75 GP |
| Mastiff | 195 lb. | 25 GP |
| Mule | 420 lb. | 8 GP |
| Pony | 225 lb. | 30 GP |
| Warhorse | 540 lb. | 400 GP |

**Tack, Harness, and Drawn Vehicles**

| Item | Weight | Cost |
| --- | --- | --- |
| Carriage | 600 lb. | 100 GP |
| Cart | 200 lb. | 15 GP |
| Chariot | 100 lb. | 250 GP |
| Feed per day | 10 lb. | 5 CP |
| Saddle |  |  |
| Exotic | 40 lb. | 60 GP |
| Military | 30 lb. | 20 GP |
| Riding | 25 lb. | 10 GP |
| Sled | 300 lb. | 20 GP |
| Stabling per day | — | 5 SP |
| Wagon | 400 lb. | 35 GP |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/mounts_and_vehicles_saddles ` |
| ` implementation ` | ` dnd2024:rules_mounts_and_vehicles_saddles ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` saddle_includes ` | ` ["bit", "bridle", "reins", "other_required_equipment"] ` |
| ` exotic_saddle_required_for ` | ` ["aquatic_mount", "flying_mount"] ` |

#### Military saddle

<!-- splat:values path=splat_rule/parameters/military_saddle -->

| Field | Value |
| --- | --- |
| ` remain_mounted_ability_checks ` | ` advantage ` |

#### Mounts

<!-- splat:table
path: splat_rule/parameters/mounts
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
- row_7
- row_8
columns:
  Key:
    type: key
  Kind:
    path: kind
    type: symbol
  Capacity lb:
    path: capacity_lb
    type: integer
  Cost gp:
    path: cost_gp
    type: integer
-->

| Key | Kind | Capacity lb | Cost gp |
| --- | --- | --- | --- |
| ` row_1 ` | ` Camel ` | ` 450 ` | ` 50 ` |
| ` row_2 ` | ` Elephant ` | ` 1320 ` | ` 200 ` |
| ` row_3 ` | ` Draft horse ` | ` 540 ` | ` 50 ` |
| ` row_4 ` | ` Riding horse ` | ` 480 ` | ` 75 ` |
| ` row_5 ` | ` Mastiff ` | ` 195 ` | ` 25 ` |
| ` row_6 ` | ` Mule ` | ` 420 ` | ` 8 ` |
| ` row_7 ` | ` Pony ` | ` 225 ` | ` 30 ` |
| ` row_8 ` | ` Warhorse ` | ` 540 ` | ` 400 ` |

#### Tack and vehicles

<!-- splat:table
path: splat_rule/parameters/tack_and_vehicles
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
- row_7
- row_8
- row_9
- row_10
columns:
  Key:
    type: key
  Kind:
    path: kind
    type: symbol
  Weight lb:
    path: weight_lb
    type: integer
    nullable: true
  Details:
    type: section
    optional: true
-->

| Key | Kind | Weight lb | Details |
| --- | --- | --- | --- |
| ` row_1 ` | ` Carriage ` | ` 600 ` | [Rules](#^splat-b477bd65-row_1 "section:row_1") |
| ` row_2 ` | ` Cart ` | ` 200 ` | [Rules](#^splat-b477bd65-row_2 "section:row_2") |
| ` row_3 ` | ` Chariot ` | ` 100 ` | [Rules](#^splat-b477bd65-row_3 "section:row_3") |
| ` row_4 ` | ` Feed per day ` | ` 10 ` | [Rules](#^splat-b477bd65-row_4 "section:row_4") |
| ` row_5 ` | ` Exotic saddle ` | ` 40 ` | [Rules](#^splat-b477bd65-row_5 "section:row_5") |
| ` row_6 ` | ` Military saddle ` | ` 30 ` | [Rules](#^splat-b477bd65-row_6 "section:row_6") |
| ` row_7 ` | ` Riding saddle ` | ` 25 ` | [Rules](#^splat-b477bd65-row_7 "section:row_7") |
| ` row_8 ` | ` Sled ` | ` 300 ` | [Rules](#^splat-b477bd65-row_8 "section:row_8") |
| ` row_9 ` | ` Stabling per day ` | null | [Rules](#^splat-b477bd65-row_9 "section:row_9") |
| ` row_10 ` | ` Wagon ` | ` 400 ` | [Rules](#^splat-b477bd65-row_10 "section:row_10") |

##### Row 1

^splat-b477bd65-row_1

###### Cost gp

<!-- splat:yaml path=splat_rule/parameters/tack_and_vehicles/@row_1/cost_gp -->

```yaml
100
```

##### Row 2

^splat-b477bd65-row_2

###### Cost gp

<!-- splat:yaml path=splat_rule/parameters/tack_and_vehicles/@row_2/cost_gp -->

```yaml
15
```

##### Row 3

^splat-b477bd65-row_3

###### Cost gp

<!-- splat:yaml path=splat_rule/parameters/tack_and_vehicles/@row_3/cost_gp -->

```yaml
250
```

##### Row 4

^splat-b477bd65-row_4

###### Cost gp

<!-- splat:yaml path=splat_rule/parameters/tack_and_vehicles/@row_4/cost_gp -->

```yaml
0.05
```

##### Row 5

^splat-b477bd65-row_5

###### Cost gp

<!-- splat:yaml path=splat_rule/parameters/tack_and_vehicles/@row_5/cost_gp -->

```yaml
60
```

##### Row 6

^splat-b477bd65-row_6

###### Cost gp

<!-- splat:yaml path=splat_rule/parameters/tack_and_vehicles/@row_6/cost_gp -->

```yaml
20
```

##### Row 7

^splat-b477bd65-row_7

###### Cost gp

<!-- splat:yaml path=splat_rule/parameters/tack_and_vehicles/@row_7/cost_gp -->

```yaml
10
```

##### Row 8

^splat-b477bd65-row_8

###### Cost gp

<!-- splat:yaml path=splat_rule/parameters/tack_and_vehicles/@row_8/cost_gp -->

```yaml
20
```

##### Row 9

^splat-b477bd65-row_9

###### Cost gp

<!-- splat:yaml path=splat_rule/parameters/tack_and_vehicles/@row_9/cost_gp -->

```yaml
0.5
```

##### Row 10

^splat-b477bd65-row_10

###### Cost gp

<!-- splat:yaml path=splat_rule/parameters/tack_and_vehicles/@row_10/cost_gp -->

```yaml
35
```
