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
splat_rule:
  mechanic: dnd2024:rules/combat_encounters_step_3_spend_your_budget
  parameters:
    budget_remaining:
      op: subtract
      args:
      - variable: xp_budget
      - op: sum
        args:
        - variable: selected_monster_xp
    constraint: selected_total_lte_budget
    unspent_xp_allowed: true
    examples:
    - party_size: 4
      level: 1
      difficulty: low
      xp_per_character: 50
      budget: 200
      choices:
      - - monster: bugbear_warrior
          count: 1
          xp_each: 200
      - - monster: giant_wasp
          count: 2
          xp_each: 100
      - - monster: giant_rat
          count: 6
          xp_each: 25
    - party_size: 5
      level: 3
      difficulty: moderate
      xp_per_character: 225
      budget: 1125
      choices:
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
    - party_size: 6
      level: 15
      difficulty: high
      xp_per_character: 7800
      budget: 46800
      choices:
      - - monster: adult_red_dragon
          count: 2
          xp_each: 18000
        - monster: fire_giant
          count: 2
          xp_each: 5000
  implementation: dnd2024:rule_combat_encounters_step_3_spend_your_budget
splat_contracts:
- id: dnd2024:rule_combat_encounters_step_3_spend_your_budget
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    selected_monster_xp:
      type: resolution
      required: true
    xp_budget:
      type: number
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - resolution
  events: []
  capabilities:
  - encounter_building
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 202
  - 203
  engine_gaps: &id001
  - dnd2024:rule_combat_encounters_step_3_spend_your_budget
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 897495d20832a9b96d72c4bf4fb9718768e4cc55f298e87b6595d51c529b2f00
  parent_name: Combat Encounters
  line_span:
    first: 10316
    count: 29
splat_clauses:
- key: source_rules
  text: '# Step 3: Spend Your Budget Every creature has an XP value in its stat block. When you add a
    creature to your combat encounter, deduct its XP from your XP budget to determine how many XP you
    have left to spend. Spend as much of your XP budget as you can without going over. It’s OK if you
    have a few unspent XP left over. Examples are given below: Example 1. A low-difficulty encounter for
    four level 1 characters has an XP budget of 50 × 4, for a total of 200 XP. With that, you could build
    any of the following encounters: • 1 Bugbear Warrior (200 XP) • 2 Giant Wasps (100 XP each), for 200
    XP total • 6 Giant Rats (25 XP each), for 150 XP total Example 2. A moderate-difficulty encounter
    for five level 3 characters has an XP budget of 225 × 5, for a total of 1,125 XP. With that, you could
    build either of these encounters: • 2 Druids (450 XP each) and 9 Stirges (25 XP each), for 1,125 XP
    total • 1 Wight (700 XP), 1 Warhorse Skeleton (100 XP), and 6 Skeletons (50 XP each), for 1,100 XP
    total Example 3. A high-difficulty encounter for six level 15 characters has an XP budget of 7,800
    × 6, for a total of 46,800 XP. With that, you could build this encounter: • 2 Adult Red Dragons (18,000
    XP each) and 2 Fire Giants (5,000 XP each), for 46,000 XP total'
  sha256: bd4e9082d7716b374bb730385c843a6142ff74456157ff3e033bf6180130dbb4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Step 3: Spend Your Budget

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
