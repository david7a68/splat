---
splat_schema: 1
splat_id: dd9abbac-6dce-5d98-a8a6-66111abe138a
splat_kind: rule
splat_key: dnd2024:rules/armor_armor_training
splat_name: Armor Training
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/armor_armor_training
  parameters:
    don_without_training: true
    training_source:
    - class
    - features
    monster_training: all_armor_in_stat_block
  implementation: dnd2024:rules_armor_armor_training
splat_contracts:
- id: dnd2024:rules_armor_armor_training
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - armor_training
  - equipment_eligibility
  events: []
  capabilities:
  - equipment
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 92
  engine_gaps: &id001
  - dnd2024:rules_armor_armor_training
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 2658d0be04189dd0da0a35f952551bee8b1a10fb783b0abaaf0ac87eade81106
  parent_name: Armor
  line_span:
    first: 7945
    count: 7
splat_clauses:
- key: source_rules
  text: '# Armor Training Anyone can don armor or hold a Shield, but only those with training can use
    them effectively, as explained below. A character’s class and other features determine the character’s
    armor training. A monster has training with any armor in its stat block.'
  sha256: 9bb74a513161dc6319861bbe008330c641558052fc5527776ea74ddb37b4481b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Armor Training

<!-- SRD page 92 -->

Anyone can don armor or hold a Shield, but only\
those with training can use them effectively, as\
explained below. A character’s class and other fea-\
tures determine the character’s armor training.\
A monster has training with any armor in its stat\
block.\
