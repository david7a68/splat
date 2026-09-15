---
splat_schema: 1
splat_id: 3ee1085a-5278-5c47-840b-2ad360958a83
splat_kind: rule
splat_key: dnd2024:rules/warlock_misty_visions
splat_name: Misty Visions
splat_origin:
  key: dnd2024:reference/pages/074
splat_sources:
- title: SRD 5.2.1
  locator: pp. 74
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_misty_visions
  parameters:
    warlock_level_minimum: 2
    cast:
      spell:
        key: dnd2024:spells/silent_image
      spell_slot_cost: 0
      target: spell_definition
      uses: unlimited
  implementation: dnd2024:rule_warlock_misty_visions
splat_contracts:
- id: dnd2024:rule_warlock_misty_visions
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - feature_effects
  events: []
  capabilities:
  - invocations
  - spellcasting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 74
  engine_gaps: &id001
  - dnd2024:rule_warlock_misty_visions
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 0b218b324f4ff09583ee0ef229224a5ca65db97cfff9beb06acb3423a3f6f56c
  parent_name: Warlock
  line_span:
    first: 6281
    count: 4
splat_clauses:
- key: source_rules
  text: '# Misty Visions Prerequisite: Level 2+ Warlock You can cast Silent Image without expending a
    spell slot.'
  sha256: 949876492f73f6e7f0bc1816b1702d2b6e3d4321efddeb308f187e5b74a93854
  status: contract_specified
splat_requires_mechanics: *id001
---

# Misty Visions

<!-- SRD page 74 -->

Prerequisite: Level 2+ Warlock\
You can cast Silent Image without expending a spell\
slot.\
