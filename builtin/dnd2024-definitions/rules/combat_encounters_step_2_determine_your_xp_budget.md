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
splat_rule:
  mechanic: dnd2024:rules/combat_encounters_step_2_determine_your_xp_budget
  parameters:
    per_character_xp:
    - level: 1
      low: 50
      moderate: 75
      high: 100
    - level: 2
      low: 100
      moderate: 150
      high: 200
    - level: 3
      low: 150
      moderate: 225
      high: 400
    - level: 4
      low: 250
      moderate: 375
      high: 500
    - level: 5
      low: 500
      moderate: 750
      high: 1100
    - level: 6
      low: 600
      moderate: 1000
      high: 1400
    - level: 7
      low: 750
      moderate: 1300
      high: 1700
    - level: 8
      low: 1000
      moderate: 1700
      high: 2100
    - level: 9
      low: 1300
      moderate: 2000
      high: 2600
    - level: 10
      low: 1600
      moderate: 2300
      high: 3100
    - level: 11
      low: 1900
      moderate: 2900
      high: 4100
    - level: 12
      low: 2200
      moderate: 3700
      high: 4700
    - level: 13
      low: 2600
      moderate: 4200
      high: 5400
    - level: 14
      low: 2900
      moderate: 4900
      high: 6200
    - level: 15
      low: 3300
      moderate: 5400
      high: 7800
    - level: 16
      low: 3800
      moderate: 6100
      high: 9800
    - level: 17
      low: 4500
      moderate: 7200
      high: 11700
    - level: 18
      low: 5000
      moderate: 8700
      high: 14200
    - level: 19
      low: 5500
      moderate: 10700
      high: 17200
    - level: 20
      low: 6400
      moderate: 13200
      high: 22000
    encounter_budget:
      op: multiply
      args:
      - var: per_character_budget
      - var: party_size
  implementation: dnd2024:rules_combat_encounters_step_2_determine_your_xp_budget
splat_contracts:
- id: dnd2024:rules_combat_encounters_step_2_determine_your_xp_budget
  inputs:
    party_level:
      type: integer
      required: true
    party_size:
      type: integer
      required: true
    difficulty:
      type: choice
      required: true
  outputs:
  - xp_budget
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
  engine_gaps: &id001
  - dnd2024:rules_combat_encounters_step_2_determine_your_xp_budget
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 89da161e515707d49837ba664fbea98edb19016108486d33c2ea0ff4b8ca0254
  parent_name: Combat Encounters
  line_span:
    first: 10287
    count: 29
splat_clauses:
- key: source_rules
  text: '# Step 2: Determine Your XP Budget Using the XP Budget per Character table, cross-reference the
    party’s level with the desired encounter difficulty. Multiply the number in the table by the number
    of characters in the party to get your XP budget for the encounter. XP Budget per Character Party’s
    ——— Encounter Difficulty ——— Level Low Moderate High 1 50 75 100 2 100 150 200 3 150 225 400 4 250
    375 500 5 500 750 1,100 6 600 1,000 1,400 7 750 1,300 1,700 8 1,000 1,700 2,100 9 1,300 2,000 2,600
    10 1,600 2,300 3,100 11 1,900 2,900 4,100 12 2,200 3,700 4,700 13 2,600 4,200 5,400 14 2,900 4,900
    6,200 15 3,300 5,400 7,800 16 3,800 6,100 9,800 17 4,500 7,200 11,700 18 5,000 8,700 14,200 19 5,500
    10,700 17,200 20 6,400 13,200 22,000'
  sha256: 7df7cef6202a42f018ebc04febbd3c8a7a55c7a203a023dd7424721bdeb65c6f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Step 2: Determine Your XP Budget

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
