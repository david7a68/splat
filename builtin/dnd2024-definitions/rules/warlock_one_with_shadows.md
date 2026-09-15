---
splat_schema: 1
splat_id: 89220c1c-6611-5e59-b721-27175e19516a
splat_kind: rule
splat_key: dnd2024:rules/warlock_one_with_shadows
splat_name: One with Shadows
splat_origin:
  key: dnd2024:reference/pages/074
splat_sources:
- title: SRD 5.2.1
  locator: pp. 74
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_one_with_shadows
  parameters:
    prerequisites:
      warlock_level_minimum: 5
    activation_illumination:
    - dim_light
    - darkness
    spell:
      key: dnd2024:spells/invisibility
    target: self
    spell_slot_cost: 0
  implementation: dnd2024:rules_warlock_one_with_shadows
splat_contracts:
- id: dnd2024:rules_warlock_one_with_shadows
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
  - 74
  engine_gaps: &id001
  - dnd2024:rules_warlock_one_with_shadows
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: e4a2869d27a3db58f45117053177f8dcdd0e5fc449bd0c9d87fe6f6adf76f533
  parent_name: Warlock
  line_span:
    first: 6285
    count: 5
splat_clauses:
- key: source_rules
  text: '# One with Shadows Prerequisite: Level 5+ Warlock While you’re in an area of Dim Light or Darkness,
    you can cast Invisibility on yourself without expending a spell slot.'
  sha256: c0319804c70f38039c7ce8e19c462e524c2a5b75e3745685967a3613bd5c21f1
  status: contract_specified
splat_requires_mechanics: *id001
---

# One with Shadows

<!-- SRD page 74 -->

Prerequisite: Level 5+ Warlock\
While you’re in an area of Dim Light or Darkness,\
you can cast Invisibility on yourself without expend-\
ing a spell slot.\
