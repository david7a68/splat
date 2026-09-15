---
splat_schema: 1
splat_id: 30b7c9ab-a9f5-5560-a14b-b94d63fb9429
splat_kind: item
splat_key: dnd2024:equipment/crowbar_2_gp
splat_name: Crowbar (2 GP)
splat_origin:
  key: dnd2024:reference/pages/097
splat_sources:
- title: SRD 5.2.1
  locator: pp. 97
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 2
    unit: gp
  mass:
    value: 5.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/crowbar_2_gp
  implementation: dnd2024:equipment_crowbar_2_gp
  parameters:
    using:
      roll: ability_check
      mode: advantage
      ability: strength
      requires: leverage_applicable
    mass:
      value: 5.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_crowbar_2_gp
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    event:
      required: false
      type: event
    choices:
      required: false
      type: choice
  outputs:
  - item_properties
  - conditional_modifiers
  events: []
  capabilities:
  - equipment
  - conditional_modifiers
splat_conversion:
  status: specified
  category: equipment
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 97
  engine_gaps: &id001
  - dnd2024:equipment_crowbar_2_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 94b626737515650722b86e9f6b7443c0fb9c690c81e647202a655462dab4bc46
  line_span:
    first: 8374
    count: 4
splat_clauses:
- key: source_rules
  text: '# Crowbar (2 GP) Using a Crowbar gives you Advantage on Strength checks where the Crowbar’s leverage
    can be applied.'
  sha256: b57eba28c76b3f08434e83acff464793646d7eea0b286d7402792cb1eddc9319
  status: contract_specified
splat_requires_mechanics: *id001
---

# Crowbar (2 GP)

<!-- SRD page 97 -->

Using a Crowbar gives you Advantage on Strength\
checks where the Crowbar’s leverage can be\
applied.\
