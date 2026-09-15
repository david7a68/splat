---
splat_schema: 1
splat_id: a32df50c-196a-5842-8053-38461e70458a
splat_kind: item
splat_key: dnd2024:equipment/grappling_hook_2_gp
splat_name: Grappling Hook (2 GP)
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
    value: 4.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/grappling_hook_2_gp
  implementation: dnd2024:equipment_grappling_hook_2_gp
  parameters:
    throw:
      action: utilize
      range_ft: 50
      target_kinds:
      - railing
      - ledge
      - other_catch
      check:
        ability: dexterity
        skill: acrobatics
        dc: 13
      success: hook_catches
    climb:
      requires: rope_tied_to_caught_hook
      operation: climb_rope
    mass:
      value: 4.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_grappling_hook_2_gp
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
  - attack.replacement
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
  - 97
  engine_gaps: &id001
  - dnd2024:equipment_grappling_hook_2_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: ebcb54ec0a4cf0937f121593c4f7089f4dc257acf40be616cdac9795c12a2ccc
  line_span:
    first: 8409
    count: 6
splat_clauses:
- key: source_rules
  text: '# Grappling Hook (2 GP) As a Utilize action, you can throw the Grappling Hook at a railing, a
    ledge, or another catch within 50 feet of yourself, and the hook catches on if you succeed on a DC
    13 Dexterity (Acrobatics) check. If you tied a Rope to the hook, you can then climb it.'
  sha256: b28d53f854a3038cc87581a9936e90b11f97098edf7414c942195cf629a2a25a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Grappling Hook (2 GP)

<!-- SRD page 97 -->

As a Utilize action, you can throw the Grappling\
Hook at a railing, a ledge, or another catch within 50\
feet of yourself, and the hook catches on if you suc-\
ceed on a DC 13 Dexterity (Acrobatics) check. If you\
tied a Rope to the hook, you can then climb it.\
