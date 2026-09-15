---
splat_schema: 1
splat_id: bf2814bb-c15d-51ad-aa57-af32a7a81ba7
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_speed
splat_name: Speed
splat_origin:
  key: dnd2024:reference/pages/255
splat_sources:
- title: SRD 5.2.1
  locator: pp. 255
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_speed
  parameters:
    speed_fields:
    - walk
    - burrow
    - climb
    - fly
    - swim
    rules:
    - key: dnd2024:glossary/speed
    - key: dnd2024:glossary/burrow_speed
    - key: dnd2024:glossary/climb_speed
    - key: dnd2024:glossary/fly_speed
    - key: dnd2024:glossary/swim_speed
  implementation: dnd2024:rules_parts_of_a_stat_block_speed
splat_contracts:
- id: dnd2024:rules_parts_of_a_stat_block_speed
  inputs:
    definition:
      type: definition
      required: true
  outputs:
  - typed_attributes
  events: []
  capabilities:
  - definition_catalogs
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 255
  engine_gaps: &id001
  - dnd2024:rules_parts_of_a_stat_block_speed
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: caee92a047e0e93b9085de4a070db9fc6dd27b90e0905676b4e1b1fa19ff3795
  parent_name: Parts of a Stat Block
  line_span:
    first: 10988
    count: 5
splat_clauses:
- key: source_rules
  text: '# Speed The Speed entry specifies a monster’s Speed. Some monsters have one or more of the following
    speeds: Burrow, Climb, Fly, Swim. Rules for Speed and these specials speeds appear in “Rules Glossary.”'
  sha256: 24e8ca0c7b9c04fcf3f0d9933d55ea67feee1826da9191dbb98eb9710b6671c4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Speed

<!-- SRD page 255 -->

The Speed entry specifies a monster’s Speed. Some\
monsters have one or more of the following speeds:\
Burrow, Climb, Fly, Swim. Rules for Speed and these\
specials speeds appear in “Rules Glossary.”\
