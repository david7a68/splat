---
splat_schema: 1
splat_id: 07d470ab-d68e-5abe-b224-d36b2ba35380
splat_kind: item
splat_key: dnd2024:equipment/healer_s_kit_5_gp
splat_name: Healer’s Kit (5 GP)
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
    value: 5
    unit: gp
  mass:
    value: 3.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/healer_s_kit_5_gp
  implementation: dnd2024:equipment_healer_s_kit_5_gp
  parameters:
    uses: 10
    use:
      action: utilize
      expend_uses: 1
      target:
        condition: unconscious
        hit_points: 0
      effect: stabilize
      medicine_check_required: false
    mass:
      value: 3.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_healer_s_kit_5_gp
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
  - 97
  engine_gaps: &id001
  - dnd2024:equipment_healer_s_kit_5_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 1d07806d2de27d204554d47a509786c429420c8a01aff2053be413bc026d859b
  line_span:
    first: 8415
    count: 5
splat_clauses:
- key: source_rules
  text: '# Healer’s Kit (5 GP) A Healer’s Kit has ten uses. As a Utilize action, you can expend one of
    its uses to stabilize an Unconscious creature that has 0 Hit Points without needing to make a Wisdom
    (Medicine) check.'
  sha256: 877a986dd51e2d0259fe8248d922e9e0f615d375808d39347f105093c9a3dc0c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Healer’s Kit (5 GP)

<!-- SRD page 97 -->

A Healer’s Kit has ten uses. As a Utilize action, you\
can expend one of its uses to stabilize an Uncon-\
scious creature that has 0 Hit Points without need-\
ing to make a Wisdom (Medicine) check.\
