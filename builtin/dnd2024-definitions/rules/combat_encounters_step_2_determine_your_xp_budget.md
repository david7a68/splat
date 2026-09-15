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

<!-- splat:rows path=splat_rule/parameters/per_character_xp -->

| Row | level | low | moderate | high |
| --- | --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` 50 ` | ` 75 ` | ` 100 ` |
| ` 1 ` | ` 2 ` | ` 100 ` | ` 150 ` | ` 200 ` |
| ` 2 ` | ` 3 ` | ` 150 ` | ` 225 ` | ` 400 ` |
| ` 3 ` | ` 4 ` | ` 250 ` | ` 375 ` | ` 500 ` |
| ` 4 ` | ` 5 ` | ` 500 ` | ` 750 ` | ` 1100 ` |
| ` 5 ` | ` 6 ` | ` 600 ` | ` 1000 ` | ` 1400 ` |
| ` 6 ` | ` 7 ` | ` 750 ` | ` 1300 ` | ` 1700 ` |
| ` 7 ` | ` 8 ` | ` 1000 ` | ` 1700 ` | ` 2100 ` |
| ` 8 ` | ` 9 ` | ` 1300 ` | ` 2000 ` | ` 2600 ` |
| ` 9 ` | ` 10 ` | ` 1600 ` | ` 2300 ` | ` 3100 ` |
| ` 10 ` | ` 11 ` | ` 1900 ` | ` 2900 ` | ` 4100 ` |
| ` 11 ` | ` 12 ` | ` 2200 ` | ` 3700 ` | ` 4700 ` |
| ` 12 ` | ` 13 ` | ` 2600 ` | ` 4200 ` | ` 5400 ` |
| ` 13 ` | ` 14 ` | ` 2900 ` | ` 4900 ` | ` 6200 ` |
| ` 14 ` | ` 15 ` | ` 3300 ` | ` 5400 ` | ` 7800 ` |
| ` 15 ` | ` 16 ` | ` 3800 ` | ` 6100 ` | ` 9800 ` |
| ` 16 ` | ` 17 ` | ` 4500 ` | ` 7200 ` | ` 11700 ` |
| ` 17 ` | ` 18 ` | ` 5000 ` | ` 8700 ` | ` 14200 ` |
| ` 18 ` | ` 19 ` | ` 5500 ` | ` 10700 ` | ` 17200 ` |
| ` 19 ` | ` 20 ` | ` 6400 ` | ` 13200 ` | ` 22000 ` |

#### Encounter budget

<!-- splat:yaml path=splat_rule/parameters/encounter_budget -->

```yaml
op: multiply
args:
- var: per_character_budget
- var: party_size
```
