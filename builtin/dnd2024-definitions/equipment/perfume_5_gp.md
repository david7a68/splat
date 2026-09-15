---
splat_schema: 1
splat_id: 28e2a10a-5f6e-510d-a6d4-e7b3287c5c95
splat_kind: item
splat_key: dnd2024:equipment/perfume_5_gp
splat_name: Perfume (5 GP)
splat_origin:
  key: dnd2024:reference/pages/099
splat_sources:
- title: SRD 5.2.1
  locator: pp. 99
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 5
    unit: gp
  mass:
    value: 0.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/perfume_5_gp
  implementation: dnd2024:equipment_perfume_5_gp
  parameters:
    container: vial
    volume_ounces: 4
    apply:
      target: self
      duration:
        value: 1
        unit: hour
      effect:
        roll: ability_check
        mode: advantage
        ability: charisma
        skill: persuasion
        target_type: humanoid
        target_attitude: indifferent
        target_max_distance_ft: 5
    mass:
      value: 0.0
      unit: lb
      negligible: true
splat_contracts:
- id: dnd2024:equipment_perfume_5_gp
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
  - 99
  engine_gaps: &id001
  - dnd2024:equipment_perfume_5_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: aec1787cc91d9ab3a5e06ea071311b9c9912881e80e8c9323af7ab6b09745e35
  line_span:
    first: 8563
    count: 5
splat_clauses:
- key: source_rules
  text: '# Perfume (5 GP) Perfume comes in a 4-ounce vial. For 1 hour after applying Perfume to yourself,
    you have Advantage on Charisma (Persuasion) checks made to influence an Indifferent Humanoid within
    5 feet of yourself.'
  sha256: 5eeebea64e12dfcabfa3002c9114b32398501915defe9aa20e90f3d0c65e5a07
  status: contract_specified
splat_requires_mechanics: *id001
---

# Perfume (5 GP)

<!-- SRD page 99 -->

Perfume comes in a 4-ounce vial. For 1 hour after\
applying Perfume to yourself, you have Advantage\
on Charisma (Persuasion) checks made to influence\
an Indifferent Humanoid within 5 feet of yourself.\
