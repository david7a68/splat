---
splat_schema: 1
splat_id: 2cdbf241-d2da-5938-afc8-0850fddc7ffd
splat_kind: rule
splat_key: dnd2024:rules/warlock_otherworldly_leap
splat_name: Otherworldly Leap
splat_origin:
  key: dnd2024:reference/pages/074
splat_sources:
- title: SRD 5.2.1
  locator: pp. 74
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_otherworldly_leap
  parameters:
    prerequisites:
      warlock_level_min: 2
    grant_cast:
      spell:
        key: dnd2024:spells/jump
      spell_slot_cost: 0
      uses: unlimited
      target: self
  implementation: dnd2024:rules_warlock_otherworldly_leap
splat_contracts:
- id: dnd2024:rules_warlock_otherworldly_leap
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
  - dnd2024:rules_warlock_otherworldly_leap
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 6cd3627687a5349dcee222c0cead1205b598b889d1e4955effc1450366952cda
  parent_name: Warlock
  line_span:
    first: 6290
    count: 4
splat_clauses:
- key: source_rules
  text: '# Otherworldly Leap Prerequisite: Level 2+ Warlock You can cast Jump on yourself without expending
    a spell slot.'
  sha256: 3dc4aa221de63a296179e833f65daa9b1188bb48e21b71a5e1d99e6caf1146ae
  status: contract_specified
splat_requires_mechanics: *id001
---

# Otherworldly Leap

<!-- SRD page 74 -->

Prerequisite: Level 2+ Warlock\
You can cast Jump on yourself without expending a\
spell slot.\
