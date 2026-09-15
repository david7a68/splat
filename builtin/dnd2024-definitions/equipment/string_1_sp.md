---
splat_schema: 1
splat_id: fa51b46c-8b43-5174-b9b5-b535c228a2b3
splat_kind: item
splat_key: dnd2024:equipment/string_1_sp
splat_name: String (1 SP)
splat_origin:
  key: dnd2024:reference/pages/100
splat_sources:
- title: SRD 5.2.1
  locator: pp. 100
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 1
    unit: sp
  mass:
    value: 0.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/string_1_sp
  implementation: dnd2024:equipment_string_1_sp
  parameters:
    length_ft: 10
    tie_knot:
      action: utilize
    mass:
      value: 0.0
      unit: lb
      negligible: true
splat_contracts:
- id: dnd2024:equipment_string_1_sp
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
  - 100
  engine_gaps: &id001
  - dnd2024:equipment_string_1_sp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 5312f8412c61a5968efd1f62fccb10f2f451a9c5de5b05b04b4c2a69af741a41
  line_span:
    first: 8659
    count: 3
splat_clauses:
- key: source_rules
  text: '# String (1 SP) String is 10 feet long. You can tie a knot in it as a Utilize action.'
  sha256: 3d575fdfba3a7b7899e2dc867da9964f278fa44941af74785f5c0c028619c22b
  status: contract_specified
splat_requires_mechanics: *id001
---

# String (1 SP)

<!-- SRD page 100 -->

String is 10 feet long. You can tie a knot in it as a Uti-\
lize action.\
