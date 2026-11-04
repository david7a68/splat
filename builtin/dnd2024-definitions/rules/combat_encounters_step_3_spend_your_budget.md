---
splat_schema: 2
splat_id: d0f524aa-d09e-5e5a-8d56-29bc88f54821
splat_kind: rule
splat_key: dnd2024:rules/combat_encounters_step_3_spend_your_budget
splat_name: 'Step 3: Spend Your Budget'
splat_origin: dnd2024:reference/pages/202
splat_sources:
- SRD 5.2.1, pp. 202, 203, https://www.dndbeyond.com/srd
---

# Step 3: Spend Your Budget

## Source text

<!-- SRD page 202 -->

Every creature has an XP value in its stat block. When you add a creature to your combat encounter, deduct its XP from your XP budget to determine how many XP you have left to spend. Spend as much of your XP budget as you can without going over. It’s OK if you have a few unspent XP left over. Examples are given below:

Example 1. A low-difficulty encounter for four level 1 characters has an XP budget of 50 × 4, for a total of 200 XP. With that, you could build any of the following encounters:

•   1  Bugbear Warrior (200 XP)

•   2  Giant Wasps (100 XP each), for 200 XP total

•   6  Giant Rats (25 XP each), for 150 XP total

Example 2. A moderate-difficulty encounter for five level 3 characters has an XP budget of 225 × 5, for a total of 1,125 XP. With that, you could build either of these encounters:

•   2  Druids (450 XP each) and 9 Stirges (25 XP each), for 1,125 XP total

•   1  Wight (700 XP), 1 Warhorse Skeleton (100 XP), and 6 Skeletons (50 XP each), for 1,100 XP total

<!-- SRD page 203 -->

Example 3. A high-difficulty encounter for six level 15 characters has an XP budget of 7,800 × 6, for a total of 46,800 XP. With that, you could build this encounter:

•   2  Adult Red Dragons (18,000 XP each) and 2 Fire Giants (5,000 XP each), for 46,000 XP total

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

<!-- splat:table
path: splat_rule/parameters/examples
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Party size:
    path: party_size
    type: integer
  Level:
    path: level
    type: integer
  Difficulty:
    path: difficulty
    type: symbol
  Xp per character:
    path: xp_per_character
    type: integer
  Budget:
    path: budget
    type: integer
  Details:
    type: section
    optional: true
-->

| Key | Party size | Level | Difficulty | Xp per character | Budget | Details |
| --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 4 ` | ` 1 ` | ` Low ` | ` 50 ` | ` 200 ` | [Rules](#^splat-476791c0-row_1 "section:row_1") |
| ` row_2 ` | ` 5 ` | ` 3 ` | ` Moderate ` | ` 225 ` | ` 1125 ` | [Rules](#^splat-476791c0-row_2 "section:row_2") |
| ` row_3 ` | ` 6 ` | ` 15 ` | ` High ` | ` 7800 ` | ` 46800 ` | [Rules](#^splat-476791c0-row_3 "section:row_3") |

##### Row 1

^splat-476791c0-row_1

###### Choices

<!-- splat:yaml path=splat_rule/parameters/examples/@row_1/choices -->

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

##### Row 2

^splat-476791c0-row_2

###### Choices

<!-- splat:yaml path=splat_rule/parameters/examples/@row_2/choices -->

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

##### Row 3

^splat-476791c0-row_3

###### Choices

<!-- splat:yaml path=splat_rule/parameters/examples/@row_3/choices -->

```yaml
- - monster: adult_red_dragon
    count: 2
    xp_each: 18000
  - monster: fire_giant
    count: 2
    xp_each: 5000
```
