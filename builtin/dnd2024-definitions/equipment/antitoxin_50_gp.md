---
splat_schema: 1
splat_id: 647722ff-0ca2-5b87-958c-c34ce116c765
splat_kind: item
splat_key: dnd2024:equipment/antitoxin_50_gp
splat_name: Antitoxin (50 GP)
splat_origin:
  key: dnd2024:reference/pages/096
splat_sources:
- title: SRD 5.2.1
  locator: pp. 96
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 50
    unit: gp
  mass:
    value: 0.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/antitoxin_50_gp
  implementation: dnd2024:equipment_antitoxin_50_gp
  parameters:
    use:
      economy: bonus_action
      consume: 1
      target: self
      operation: drink
      effect:
        roll: saving_throw
        mode: advantage
        scope: avoid_or_end_poisoned
        duration:
          value: 1
          unit: hour
    mass:
      value: 0.0
      unit: lb
      negligible: true
splat_contracts:
- id: dnd2024:equipment_antitoxin_50_gp
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    operation:
      required: false
      type: choice
    target:
      required: false
      type: participant
    target_object:
      required: false
      type: object
    location:
      required: false
      type: location
    event:
      required: false
      type: event
    choices:
      required: false
      type: choice
  outputs:
  - action_options
  - check_requests
  - item_effects
  - item_state
  events:
  - item.used
  capabilities:
  - item_actions
  - effects
  - ability_checks
  - consumables
splat_conversion:
  status: specified
  category: equipment
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 96
  engine_gaps: &id001
  - dnd2024:equipment_antitoxin_50_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: af91853e86bc2017c1544bc1d8cf6f4a14790db5f26be4e245ee856b1d5c7cbd
  line_span:
    first: 8261
    count: 4
splat_clauses:
- key: source_rules
  text: '# Antitoxin (50 GP) As a Bonus Action, you can drink a vial of Antitoxin to gain Advantage on
    saving throws to avoid or end the Poisoned condition for 1 hour.'
  sha256: 8c918aa7aeccf9d6f4265f5c16ab222f1d806a29cd767234b3d486fa4a3575e1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Antitoxin (50 GP)

<!-- SRD page 96 -->

As a Bonus Action, you can drink a vial of Antitoxin\
to gain Advantage on saving throws to avoid or end\
the Poisoned condition for 1 hour.\
