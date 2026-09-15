---
splat_schema: 1
splat_id: 9076f231-1622-5842-9ad8-05f714983f08
splat_kind: rule
splat_key: dnd2024:rules/weapons_two_handed
splat_name: Two-Handed
splat_origin:
  key: dnd2024:reference/pages/090
splat_sources:
- title: SRD 5.2.1
  locator: pp. 90
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/weapons_two_handed
  parameters:
    requires_property: two_handed
    hands_required_when_attacking: 2
  implementation: dnd2024:rules_weapons_two_handed
splat_contracts:
- id: dnd2024:rules_weapons_two_handed
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - attack.requested
  capabilities:
  - weapon_properties
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 90
  engine_gaps: &id001
  - dnd2024:rules_weapons_two_handed
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 97272c3f26c24ea4a233f39ad277d46da46ee3134a2323955c88baaf6ad2cae0
  parent_name: Weapons
  line_span:
    first: 7753
    count: 3
splat_clauses:
- key: source_rules
  text: '# Two-Handed A Two-Handed weapon requires two hands when you attack with it.'
  sha256: 18353df5707ad988bd67d8a84214a055d366d83370768e9e9f5e3d1d89ee9a42
  status: contract_specified
splat_requires_mechanics: *id001
---

# Two-Handed

<!-- SRD page 90 -->

A Two-Handed weapon requires two hands when\
you attack with it.\
