---
splat_schema: 1
splat_id: d1bc292c-ffb9-56fa-8f3a-db0d56b6233d
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_experience_points
splat_name: Experience Points
splat_origin:
  key: dnd2024:reference/pages/256
splat_sources:
- title: SRD 5.2.1
  locator: pp. 256
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_experience_points
  parameters:
    xp_table:
    - cr: 0
      xp: null
    - cr: 0.125
      xp: 25
    - cr: 0.25
      xp: 50
    - cr: 0.5
      xp: 100
    - cr: 1
      xp: 200
    - cr: 2
      xp: 450
    - cr: 3
      xp: 700
    - cr: 4
      xp: 1100
    - cr: 5
      xp: 1800
    - cr: 6
      xp: 2300
    - cr: 7
      xp: 2900
    - cr: 8
      xp: 3900
    - cr: 9
      xp: 5000
    - cr: 10
      xp: 5900
    - cr: 11
      xp: 7200
    - cr: 12
      xp: 8400
    - cr: 13
      xp: 10000
    - cr: 14
      xp: 11500
    - cr: 15
      xp: 13000
    - cr: 16
      xp: 15000
    - cr: 17
      xp: 18000
    - cr: 18
      xp: 20000
    - cr: 19
      xp: 22000
    - cr: 20
      xp: 25000
    - cr: 21
      xp: 33000
    - cr: 22
      xp: 41000
    - cr: 23
      xp: 50000
    - cr: 24
      xp: 62000
    - cr: 25
      xp: 75000
    - cr: 26
      xp: 90000
    - cr: 27
      xp: 105000
    - cr: 28
      xp: 120000
    - cr: 29
      xp: 135000
    - cr: 30
      xp: 155000
    cr_zero_xp_options:
    - 0
    - 10
    award_events:
    - defeated_in_combat
    - otherwise_neutralized
    magically_summoned_xp: stat_block_unless_overridden
  implementation: dnd2024:rules_parts_of_a_stat_block_experience_points
splat_contracts:
- id: dnd2024:rules_parts_of_a_stat_block_experience_points
  inputs:
    monster:
      type: participant
      required: true
    event:
      type: event
      required: true
  outputs:
  - xp_award
  events:
  - creature.defeated
  capabilities:
  - experience
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 256
  engine_gaps: &id001
  - dnd2024:rules_parts_of_a_stat_block_experience_points
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 84cbff3cf63f0139621bc49b5d1da30a091d38a9e13c059213b54e61113067b1
  parent_name: Parts of a Stat Block
  line_span:
    first: 11084
    count: 29
splat_clauses:
- key: source_rules
  text: '# Experience Points The number of Experience Points (XP) a monster is worth is based on its CR,
    as detailed in the Experience Points by Challenge Rating table. XP is awarded for defeating the monster
    in combat or otherwise neutralizing it. Unless a rule says otherwise, a monster summoned by a spell
    or another magical ability is worth the XP noted in its stat block. Experience Points by Challenge
    Rating CR XP CR XP 0 0 or 10 14 11,500 1/8 25 15 13,000 1/4 50 16 15,000 1/2 100 17 18,000 1 200 18
    20,000 2 450 19 22,000 3 700 20 25,000 4 1,100 21 33,000 5 1,800 22 41,000 6 2,300 23 50,000 7 2,900
    24 62,000 8 3,900 25 75,000 9 5,000 26 90,000 10 5,900 27 105,000 CR XP CR XP 11 7,200 28 120,000
    12 8,400 29 135,000 13 10,000 30 155,000'
  sha256: b171a57db9cb2dec54bdd3fcce69907b144eea15d6fdc9cbc320a952d10a968f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Experience Points

<!-- SRD page 256 -->

The number of Experience Points (XP) a monster\
is worth is based on its CR, as detailed in the Ex-\
perience Points by Challenge Rating table. XP is\
awarded for defeating the monster in combat or\
otherwise neutralizing it.\
Unless a rule says otherwise, a monster sum-\
moned by a spell or another magical ability is worth\
the XP noted in its stat block.\
Experience Points by Challenge Rating\
CR XP CR XP\
0 0 or 10 14 11,500\
1/8 25 15 13,000\
1/4 50 16 15,000\
1/2 100 17 18,000\
1 200 18 20,000\
2 450 19 22,000\
3 700 20 25,000\
4 1,100 21 33,000\
5 1,800 22 41,000\
6 2,300 23 50,000\
7 2,900 24 62,000\
8 3,900 25 75,000\
9 5,000 26 90,000\
10 5,900 27 105,000\
CR XP CR XP\
11 7,200 28 120,000\
12 8,400 29 135,000\
13 10,000 30 155,000\
