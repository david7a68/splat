---
splat_schema: 1
splat_id: c726b4ab-3d36-5b41-89f1-a7a19c8ccf2a
splat_kind: rule
splat_key: dnd2024:rules/multiclassing_experience_points
splat_name: Experience Points
splat_origin:
  key: dnd2024:reference/pages/025
splat_sources:
- title: SRD 5.2.1
  locator: pp. 25
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/multiclassing_experience_points
  parameters:
    xp_threshold_level_basis:
      op: sum
      terms: class_levels
    advancement:
      key: dnd2024:rules/level_advancement
  implementation: dnd2024:rules_multiclassing_experience_points
splat_contracts:
- id: dnd2024:rules_multiclassing_experience_points
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
  - 25
  engine_gaps: &id001
  - dnd2024:rules_multiclassing_experience_points
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: a3aee59f8d53f24285f1550ef1deadeea5d07b88b358554c7dccf01ea54563a8
  parent_name: Multiclassing
  line_span:
    first: 1915
    count: 9
splat_clauses:
- key: source_rules
  text: '# Experience Points The Experience Point cost to gain a level is based on your total character
    level, not your level in a particular class, as shown in the Character Advancement table in “Character
    Creation.” For example, if you are a level 6 Cleric / level 1 Fighter, you must gain enough XP to
    reach level 8 before you can take your second level as a Fighter or your seventh level as a Cleric.'
  sha256: e43470781aa3896e87d805aadd3090d18bfb531b9a8d5acf11d1df3abb81e6ef
  status: contract_specified
splat_requires_mechanics: *id001
---

# Experience Points

<!-- SRD page 25 -->

The Experience Point cost to gain a level is based on\
your total character level, not your level in a partic-\
ular class, as shown in the Character Advancement\
table in “Character Creation.” For example, if you\
are a level 6 Cleric / level 1 Fighter, you must gain\
enough XP to reach level 8 before you can take your\
second level as a Fighter or your seventh level as a\
Cleric.\
