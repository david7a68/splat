---
splat_schema: 1
splat_id: 38f61356-3ddd-523a-8a52-548a6cdfe034
splat_kind: item
splat_key: dnd2024:equipment/poison_basic_100_gp
splat_name: Poison, Basic (100 GP)
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
    value: 100
    unit: gp
  mass:
    value: 0.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/poison_basic_100_gp
  implementation: dnd2024:equipment_poison_basic_100_gp
  parameters:
    apply:
      economy: bonus_action
      consume: 1
      target_choices:
      - kind: weapon
        count: 1
      - kind: ammunition
        maximum_count: 3
      duration:
        value: 1
        unit: minute
      early_expiry: poison_damage_dealt
      trigger:
        target_kind: creature
        weapon_or_ammunition_damage_type_in:
        - piercing
        - slashing
      extra_damage:
        amount:
          dice:
            count: 1
            sides: 4
          bonus: 0
        type: poison
    mass:
      value: 0.0
      unit: lb
      negligible: true
splat_contracts:
- id: dnd2024:equipment_poison_basic_100_gp
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
  - dnd2024:equipment_poison_basic_100_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 0636e750ada3ae013c84bd1a0fa4218e8b8b0583733fa75d3d08a0357821ef33
  line_span:
    first: 8568
    count: 8
splat_clauses:
- key: source_rules
  text: '# Poison, Basic (100 GP) As a Bonus Action, you can use a vial of Basic Poison to coat one weapon
    or up to three pieces of ammunition. A creature that takes Piercing or Slashing damage from the poisoned
    weapon or ammunition takes an extra 1d4 Poison damage. Once applied, the poison retains potency for
    1 minute or until its damage is dealt, whichever comes first.'
  sha256: d56544356eb339d8f388cbb1ff2f3430a48c774f30a1aa2c322d0b3baa0b9d6a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Poison, Basic (100 GP)

<!-- SRD page 99 -->

As a Bonus Action, you can use a vial of Basic Poison\
to coat one weapon or up to three pieces of ammu-\
nition. A creature that takes Piercing or Slashing\
damage from the poisoned weapon or ammunition\
takes an extra 1d4 Poison damage. Once applied,\
the poison retains potency for 1 minute or until its\
damage is dealt, whichever comes first.\
