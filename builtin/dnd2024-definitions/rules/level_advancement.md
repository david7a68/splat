---
splat_schema: 1
splat_id: 6b13a834-95d3-54ba-8233-b94b19856c22
splat_kind: rule
splat_key: dnd2024:rules/level_advancement
splat_name: Level Advancement
splat_origin:
  key: dnd2024:reference/pages/023
splat_sources:
- title: SRD 5.2.1
  locator: pp. 23
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/level_advancement
  parameters:
    advancement:
    - level: 1
      minimum_xp: 0
      proficiency_bonus: 2
    - level: 2
      minimum_xp: 300
      proficiency_bonus: 2
    - level: 3
      minimum_xp: 900
      proficiency_bonus: 2
    - level: 4
      minimum_xp: 2700
      proficiency_bonus: 2
    - level: 5
      minimum_xp: 6500
      proficiency_bonus: 3
    - level: 6
      minimum_xp: 14000
      proficiency_bonus: 3
    - level: 7
      minimum_xp: 23000
      proficiency_bonus: 3
    - level: 8
      minimum_xp: 34000
      proficiency_bonus: 3
    - level: 9
      minimum_xp: 48000
      proficiency_bonus: 4
    - level: 10
      minimum_xp: 64000
      proficiency_bonus: 4
    - level: 11
      minimum_xp: 85000
      proficiency_bonus: 4
    - level: 12
      minimum_xp: 100000
      proficiency_bonus: 4
    - level: 13
      minimum_xp: 120000
      proficiency_bonus: 5
    - level: 14
      minimum_xp: 140000
      proficiency_bonus: 5
    - level: 15
      minimum_xp: 165000
      proficiency_bonus: 5
    - level: 16
      minimum_xp: 195000
      proficiency_bonus: 5
    - level: 17
      minimum_xp: 225000
      proficiency_bonus: 6
    - level: 18
      minimum_xp: 265000
      proficiency_bonus: 6
    - level: 19
      minimum_xp: 305000
      proficiency_bonus: 6
    - level: 20
      minimum_xp: 355000
      proficiency_bonus: 6
    level_selection: highest_threshold_met
  implementation: dnd2024:rules_level_advancement
splat_contracts:
- id: dnd2024:rules_level_advancement
  inputs:
    actor:
      type: participant
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - character_updates
  events: []
  capabilities:
  - character_creation
  - progression
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 23
  engine_gaps: &id001
  - dnd2024:rules_level_advancement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 0780ebaf4237d5c7a2c3186ddb60e33ff84ce710bfb5865fc8a1936e4c172ff6
  parent_name: Level Advancement
  line_span:
    first: 1733
    count: 33
splat_clauses:
- key: source_rules
  text: '# Level Advancement While going on adventures, your character gains experience, represented by
    Experience Points (XP). A character who reaches a specified Experience Point total advances in capability.
    This advancement is called gaining a level. The Character Advancement table lists the XP you need
    to advance to a level and the Proficiency Bonus for a character of that level. When your XP total
    equals or exceeds a number in the Experience Points column, you reach the corresponding level. Character
    Advancement Level Experience Points Proficiency Bonus 1 0 +2 2 300 +2 3 900 +2 4 2,700 +2 5 6,500
    +3 6 14,000 +3 7 23,000 +3 8 34,000 +3 9 48,000 +4 10 64,000 +4 11 85,000 +4 12 100,000 +4 13 120,000
    +5 14 140,000 +5 15 165,000 +5 16 195,000 +5 17 225,000 +6 18 265,000 +6 19 305,000 +6 20 355,000
    +6'
  sha256: 942d7ca516e0bd66d813b85d46b0edee5af74cc05414cb30f22c022e186dc90f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level Advancement

<!-- SRD page 23 -->

While going on adventures, your character gains ex-\
perience, represented by Experience Points (XP). A\
character who reaches a specified Experience Point\
total advances in capability. This advancement is\
called gaining a level.\
The Character Advancement table lists the XP\
you need to advance to a level and the Proficiency\
Bonus for a character of that level. When your XP\
total equals or exceeds a number in the Experience\
Points column, you reach the corresponding level.\
Character Advancement\
Level Experience Points Proficiency Bonus\
1 0 +2\
2 300 +2\
3 900 +2\
4 2,700 +2\
5 6,500 +3\
6 14,000 +3\
7 23,000 +3\
8 34,000 +3\
9 48,000 +4\
10 64,000 +4\
11 85,000 +4\
12 100,000 +4\
13 120,000 +5\
14 140,000 +5\
15 165,000 +5\
16 195,000 +5\
17 225,000 +6\
18 265,000 +6\
19 305,000 +6\
20 355,000 +6\
