---
splat_schema: 1
splat_id: 78a1d483-6af6-51d4-907e-9e994fc248ce
splat_kind: rule
splat_key: dnd2024:rules/warlock_visions_of_distant_realms
splat_name: Visions of Distant Realms
splat_origin:
  key: dnd2024:reference/pages/074
splat_sources:
- title: SRD 5.2.1
  locator: pp. 74
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_visions_of_distant_realms
  parameters:
    prerequisites:
      warlock_level_min: 9
    grant_cast:
      spell:
        key: dnd2024:spells/arcane_eye
      spell_slot_cost: 0
      uses: unlimited
  implementation: dnd2024:rules_warlock_visions_of_distant_realms
splat_contracts:
- id: dnd2024:rules_warlock_visions_of_distant_realms
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - spell.cast_pending
  capabilities:
  - spellcasting
  - character_features
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 74
  engine_gaps: &id001
  - dnd2024:rules_warlock_visions_of_distant_realms
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: f49b4263e611a231649ba695ffeb52fc9632ce895a931b62e37a60788759a987
  parent_name: Warlock
  line_span:
    first: 6360
    count: 4
splat_clauses:
- key: source_rules
  text: '# Visions of Distant Realms Prerequisite: Level 9+ Warlock You can cast Arcane Eye without expending
    a spell slot.'
  sha256: 03e755468f614d922d1ebe8806b36d9a18f73782ca1fbf347234752a1488e99c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Visions of Distant Realms

<!-- SRD page 74 -->

Prerequisite: Level 9+ Warlock\
You can cast Arcane Eye without expending a spell\
slot.\
