---
splat_schema: 1
splat_id: 46425f3e-833b-536a-baad-1cf4d1d929a9
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_write_your_level
splat_name: Write Your Level
splat_origin:
  key: dnd2024:reference/pages/019
splat_sources:
- title: SRD 5.2.1
  locator: pp. 19
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/create_your_character_write_your_level
  parameters:
    default_starting_level: 1
    level_1_xp: 0
    higher_start_level_authority: gm
    subclass_required_at_start_level_minimum: 3
  implementation: dnd2024:rules_create_your_character_write_your_level
splat_contracts:
- id: dnd2024:rules_create_your_character_write_your_level
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
  - 19
  engine_gaps: &id001
  - dnd2024:rules_create_your_character_write_your_level
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: c2d2d8cb1bec1fbeac5a19125e244fd6516cc743373ae2e3d14e07c2c3c45af3
  parent_name: Create Your Character
  line_span:
    first: 1371
    count: 12
splat_clauses:
- key: source_rules
  text: '# Write Your Level Write your character’s level on your character sheet. Typically, a character
    starts at level 1 and advances in level by adventuring and gaining Experience Points (XP). Write Your
    XP. Also record your Experience Points. A level 1 character has 0 XP. Starting at a Higher Level.
    Your GM might start you at a higher level. If you start at level 3 or higher, write your chosen subclass
    on your character sheet. See the “Starting at Higher Levels” section later in “Character Creation”
    for more information.'
  sha256: f13eb19ec0d8a632955dec61361e404633a0b1c305491cb07fc0d7fe7e22fb6a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Write Your Level

<!-- SRD page 19 -->

Write your character’s level on your character\
sheet. Typically, a character starts at level 1 and\
advances in level by adventuring and gaining Expe-\
rience Points (XP).\
Write Your XP. Also record your Experience\
Points. A level 1 character has 0 XP.\
Starting at a Higher Level. Your GM might start\
you at a higher level. If you start at level 3 or higher,\
write your chosen subclass on your character sheet.\
See the “Starting at Higher Levels” section later in\
“Character Creation” for more information.\
