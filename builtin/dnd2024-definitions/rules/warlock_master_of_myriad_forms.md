---
splat_schema: 1
splat_id: 460b9e04-03ad-565c-8b97-c6c6454ed2af
splat_kind: rule
splat_key: dnd2024:rules/warlock_master_of_myriad_forms
splat_name: Master of Myriad Forms
splat_origin:
  key: dnd2024:reference/pages/073
splat_sources:
- title: SRD 5.2.1
  locator: pp. 73
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_master_of_myriad_forms
  parameters:
    prerequisites:
      warlock_level_min: 5
    grant_cast:
      spell:
        key: dnd2024:spells/alter_self
      spell_slot_cost: 0
      uses: unlimited
  implementation: dnd2024:rules_warlock_master_of_myriad_forms
splat_contracts:
- id: dnd2024:rules_warlock_master_of_myriad_forms
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
  - 73
  engine_gaps: &id001
  - dnd2024:rules_warlock_master_of_myriad_forms
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: e7221bd437447d203a40c22dfa7ed8a883e30f9f81044f98fb5920abf1ffb15c
  parent_name: Warlock
  line_span:
    first: 6277
    count: 4
splat_clauses:
- key: source_rules
  text: '# Master of Myriad Forms Prerequisite: Level 5+ Warlock You can cast Alter Self without expending
    a spell slot.'
  sha256: c5daf0199707b3a83302469cae50b40d29919e31a9f0a1eb842b1d1f789194df
  status: contract_specified
splat_requires_mechanics: *id001
---

# Master of Myriad Forms

<!-- SRD page 73 -->

Prerequisite: Level 5+ Warlock\
You can cast Alter Self without expending a spell\
slot.\
