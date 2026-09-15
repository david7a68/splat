---
splat_schema: 1
splat_id: 80ef0c51-5868-51d4-a1a1-33676b083151
splat_kind: rule
splat_key: dnd2024:rules/warlock_armor_of_shadows
splat_name: Armor of Shadows
splat_origin:
  key: dnd2024:reference/pages/072
splat_sources:
- title: SRD 5.2.1
  locator: pp. 72
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_armor_of_shadows
  parameters:
    prerequisites:
      warlock_level_min: 0
    grant_cast:
      spell:
        key: dnd2024:spells/mage_armor
      spell_slot_cost: 0
      uses: unlimited
      target: self
  implementation: dnd2024:rules_warlock_armor_of_shadows
splat_contracts:
- id: dnd2024:rules_warlock_armor_of_shadows
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
  - 72
  engine_gaps: &id001
  - dnd2024:rules_warlock_armor_of_shadows
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 44d9e93ab9912eff13579e0d9f377f53450366f689ec4c22281a0c6c3a38a711
  parent_name: Warlock
  line_span:
    first: 6153
    count: 3
splat_clauses:
- key: source_rules
  text: '# Armor of Shadows You can cast Mage Armor on yourself without expending a spell slot.'
  sha256: af313bdb5d41e4242419e98af8f64e6d8b192df93f192eacaab6df36ff11c00d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Armor of Shadows

<!-- SRD page 72 -->

You can cast Mage Armor on yourself without ex-\
pending a spell slot.\
