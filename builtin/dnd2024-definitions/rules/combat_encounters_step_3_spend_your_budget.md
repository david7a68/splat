---
splat_schema: 1
splat_id: d0f524aa-d09e-5e5a-8d56-29bc88f54821
splat_kind: rule
splat_key: dnd2024:rules/combat_encounters_step_3_spend_your_budget
splat_name: 'Step 3: Spend Your Budget'
splat_origin:
  key: dnd2024:reference/pages/202
splat_sources:
- title: SRD 5.2.1
  locator: pp. 202, 203
  url: https://www.dndbeyond.com/srd
---

# Step 3: Spend Your Budget

## Source text

<!-- SRD page 202 -->

Every creature has an XP value in its stat block.\
When you add a creature to your combat encounter,\
deduct its XP from your XP budget to determine\
how many XP you have left to spend. Spend as much\
of your XP budget as you can without going over. It’s\
OK if you have a few unspent XP left over. Examples\
are given below:\
Example 1. A low-difficulty encounter for four level\
1 characters has an XP budget of 50 × 4, for a total\
of 200 XP. With that, you could build any of the fol-\
lowing encounters:\
•   1  Bugbear Warrior (200 XP)\
•   2  Giant Wasps (100 XP each), for 200 XP total\
•   6  Giant Rats (25 XP each), for 150 XP total\
Example 2. A moderate-difficulty encounter for\
five level 3 characters has an XP budget of 225 × 5,\
for a total of 1,125 XP. With that, you could build\
either of these encounters:\
•   2  Druids (450 XP each) and 9 Stirges (25 XP\
each), for 1,125 XP total\
•   1  Wight (700 XP), 1 Warhorse Skeleton (100 XP),\
and 6 Skeletons (50 XP each), for 1,100 XP total\

<!-- SRD page 203 -->

Example 3. A high-difficulty encounter for six level\
15 characters has an XP budget of 7,800 × 6, for a\
total of 46,800 XP. With that, you could build this\
encounter:\
•   2  Adult Red Dragons (18,000 XP each) and 2 Fire\
Giants (5,000 XP each), for 46,000 XP total\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_encounters_step_3_spend_your_budget ` |
| ` implementation ` | ` dnd2024:rule_combat_encounters_step_3_spend_your_budget ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` constraint ` | ` selected_total_lte_budget ` |
| ` unspent_xp_allowed ` | ` true ` |

#### Budget remaining

<!-- splat:yaml path=splat_rule/parameters/budget_remaining -->

```yaml
op: subtract
args:
- variable: xp_budget
- op: sum
  args:
  - variable: selected_monster_xp
```

#### Examples

<!-- splat:rows path=splat_rule/parameters/examples -->

| Row | party_size | level | difficulty | xp_per_character | budget |
| --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 4 ` | ` 1 ` | ` low ` | ` 50 ` | ` 200 ` |
| ` 1 ` | ` 5 ` | ` 3 ` | ` moderate ` | ` 225 ` | ` 1125 ` |
| ` 2 ` | ` 6 ` | ` 15 ` | ` high ` | ` 7800 ` | ` 46800 ` |

##### Choices

<!-- splat:yaml path=splat_rule/parameters/examples/0/choices -->

```yaml
- - monster: bugbear_warrior
    count: 1
    xp_each: 200
- - monster: giant_wasp
    count: 2
    xp_each: 100
- - monster: giant_rat
    count: 6
    xp_each: 25
```

##### Choices

<!-- splat:yaml path=splat_rule/parameters/examples/1/choices -->

```yaml
- - monster: druid
    count: 2
    xp_each: 450
  - monster: stirge
    count: 9
    xp_each: 25
- - monster: wight
    count: 1
    xp_each: 700
  - monster: warhorse_skeleton
    count: 1
    xp_each: 100
  - monster: skeleton
    count: 6
    xp_each: 50
```

##### Choices

<!-- splat:yaml path=splat_rule/parameters/examples/2/choices -->

```yaml
- - monster: adult_red_dragon
    count: 2
    xp_each: 18000
  - monster: fire_giant
    count: 2
    xp_each: 5000
```
