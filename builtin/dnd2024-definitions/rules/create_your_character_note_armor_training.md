---
splat_schema: 1
splat_id: 0d7978c4-4929-5885-a911-a5c68b97d137
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_note_armor_training
splat_name: Note Armor Training
splat_origin:
  key: dnd2024:reference/pages/019
splat_sources:
- title: SRD 5.2.1
  locator: pp. 19
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/create_your_character_note_armor_training
  parameters:
    grant_source: class_definition
    armor_training: record_granted_categories
    training_enables_effective_armor_use: true
  implementation: dnd2024:rules_create_your_character_note_armor_training
splat_contracts:
- id: dnd2024:rules_create_your_character_note_armor_training
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - armor_training
  events: []
  capabilities:
  - character_creation
  - equipment
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 19
  engine_gaps: &id001
  - dnd2024:rules_create_your_character_note_armor_training
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: b87d27b421852cb7dfa23338ea9743e11c86ad64e905ad3dc17c1e05512f7c74
  parent_name: Create Your Character
  line_span:
    first: 1383
    count: 7
splat_clauses:
- key: source_rules
  text: '# Note Armor Training Your class might give you training with certain categories of armor. Note
    your armor training on your character sheet. Armor training with a kind of armor means you can wear
    that armor effectively, gaining defensive bonuses from it. The categories of armor are described in
    “Equipment.”'
  sha256: 400c83b9d17811e6103857996a28b6c2db81e897e9dd84cf6f283a4152d4f982
  status: contract_specified
splat_requires_mechanics: *id001
---

# Note Armor Training

<!-- SRD page 19 -->

Your class might give you training with certain\
categories of armor. Note your armor training on\
your character sheet. Armor training with a kind of\
armor means you can wear that armor effectively,\
gaining defensive bonuses from it. The categories of\
armor are described in “Equipment.”\
