---
splat_schema: 1
splat_id: 84c13a9c-c1f9-59f6-94a0-ce4101d1d3f7
splat_kind: rule
splat_key: dnd2024:rules/combat_encounters_step_2_determine_your_xp_budget
splat_name: 'Step 2: Determine Your XP Budget'
splat_origin:
  key: dnd2024:reference/pages/202
splat_sources:
- title: SRD 5.2.1
  locator: pp. 202
  url: https://www.dndbeyond.com/srd
---

# Step 2: Determine Your XP Budget

## Source text

<!-- SRD page 202 -->

Using the XP Budget per Character table, cross-ref-\
erence the party’s level with the desired encounter\
difficulty. Multiply the number in the table by the\
number of characters in the party to get your XP\
budget for the encounter.\
XP Budget per Character\
Party’s ——— Encounter Difficulty ———\
Level Low Moderate High\
1 50 75 100\
2 100 150 200\
3 150 225 400\
4 250 375 500\
5 500 750 1,100\
6 600 1,000 1,400\
7 750 1,300 1,700\
8 1,000 1,700 2,100\
9 1,300 2,000 2,600\
10 1,600 2,300 3,100\
11 1,900 2,900 4,100\
12 2,200 3,700 4,700\
13 2,600 4,200 5,400\
14 2,900 4,900 6,200\
15 3,300 5,400 7,800\
16 3,800 6,100 9,800\
17 4,500 7,200 11,700\
18 5,000 8,700 14,200\
19 5,500 10,700 17,200\
20 6,400 13,200 22,000\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_encounters_step_2_determine_your_xp_budget ` |
| ` implementation ` | ` dnd2024:rules_combat_encounters_step_2_determine_your_xp_budget ` |

### Parameters

#### Per character xp

<!-- splat:table
path: splat_rule/parameters/per_character_xp
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
- row_11
- row_12
- row_13
- row_14
- row_15
- row_16
- row_17
- row_18
- row_19
- row_20
columns:
  Key:
    type: key
  Level:
    path: level
    type: integer
  Low:
    path: low
    type: integer
  Moderate:
    path: moderate
    type: integer
  High:
    path: high
    type: integer
-->

| Key | Level | Low | Moderate | High |
| --- | --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 50 ` | ` 75 ` | ` 100 ` |
| ` row_2 ` | ` 2 ` | ` 100 ` | ` 150 ` | ` 200 ` |
| ` row_3 ` | ` 3 ` | ` 150 ` | ` 225 ` | ` 400 ` |
| ` row_4 ` | ` 4 ` | ` 250 ` | ` 375 ` | ` 500 ` |
| ` row_5 ` | ` 5 ` | ` 500 ` | ` 750 ` | ` 1100 ` |
| ` row_6 ` | ` 6 ` | ` 600 ` | ` 1000 ` | ` 1400 ` |
| ` row_7 ` | ` 7 ` | ` 750 ` | ` 1300 ` | ` 1700 ` |
| ` row_8 ` | ` 8 ` | ` 1000 ` | ` 1700 ` | ` 2100 ` |
| ` row_9 ` | ` 9 ` | ` 1300 ` | ` 2000 ` | ` 2600 ` |
| ` row_10 ` | ` 10 ` | ` 1600 ` | ` 2300 ` | ` 3100 ` |
| ` row_11 ` | ` 11 ` | ` 1900 ` | ` 2900 ` | ` 4100 ` |
| ` row_12 ` | ` 12 ` | ` 2200 ` | ` 3700 ` | ` 4700 ` |
| ` row_13 ` | ` 13 ` | ` 2600 ` | ` 4200 ` | ` 5400 ` |
| ` row_14 ` | ` 14 ` | ` 2900 ` | ` 4900 ` | ` 6200 ` |
| ` row_15 ` | ` 15 ` | ` 3300 ` | ` 5400 ` | ` 7800 ` |
| ` row_16 ` | ` 16 ` | ` 3800 ` | ` 6100 ` | ` 9800 ` |
| ` row_17 ` | ` 17 ` | ` 4500 ` | ` 7200 ` | ` 11700 ` |
| ` row_18 ` | ` 18 ` | ` 5000 ` | ` 8700 ` | ` 14200 ` |
| ` row_19 ` | ` 19 ` | ` 5500 ` | ` 10700 ` | ` 17200 ` |
| ` row_20 ` | ` 20 ` | ` 6400 ` | ` 13200 ` | ` 22000 ` |

#### Encounter budget

<!-- splat:values path=splat_rule/parameters/encounter_budget -->

| Field | Value |
| --- | --- |
| ` op ` | ` multiply ` |

##### Args

<!-- splat:table
path: splat_rule/parameters/encounter_budget/args
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Var:
    path: var
    type: symbol
-->

| Key | Var |
| --- | --- |
| ` row_1 ` | ` Per character budget ` |
| ` row_2 ` | ` Party size ` |
