---
splat_schema: 1
splat_id: 66b97ef1-73ab-50f1-abf7-4b19575fa370
splat_kind: rule
splat_key: dnd2024:rules/warlock_mask_of_many_faces
splat_name: Mask of Many Faces
splat_origin:
  key: dnd2024:reference/pages/073
splat_sources:
- title: SRD 5.2.1
  locator: pp. 73
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_mask_of_many_faces
  parameters:
    prerequisites:
      warlock_level_minimum: 2
    spell:
      key: dnd2024:spells/disguise_self
    spell_slot_cost: 0
  implementation: dnd2024:rules_warlock_mask_of_many_faces
splat_contracts:
- id: dnd2024:rules_warlock_mask_of_many_faces
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    choice:
      type: choice
      required: true
    event:
      required: false
      type: event
  outputs:
  - effects
  - resource_updates
  events: []
  capabilities:
  - class_features
  - effects
  - resources
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 73
  engine_gaps: &id001
  - dnd2024:rules_warlock_mask_of_many_faces
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: cc6af46533a2c0ef4cf5497674f82d497f532a3980bc1af0d65eaed3b19217c9
  parent_name: Warlock
  line_span:
    first: 6273
    count: 4
splat_clauses:
- key: source_rules
  text: '# Mask of Many Faces Prerequisite: Level 2+ Warlock You can cast Disguise Self without expending
    a spell slot.'
  sha256: 11b93ccbcc59ab81562faa295a90ff27291b42bd7201aa542eb0c298773fad39
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mask of Many Faces

<!-- SRD page 73 -->

Prerequisite: Level 2+ Warlock\
You can cast Disguise Self without expending a spell\
slot.\
