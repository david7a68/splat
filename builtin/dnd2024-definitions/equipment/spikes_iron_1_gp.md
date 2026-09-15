---
splat_schema: 1
splat_id: 75adbfde-5148-5cb1-a47a-f8df1383980d
splat_kind: item
splat_key: dnd2024:equipment/spikes_iron_1_gp
splat_name: Spikes, Iron (1 GP)
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
    unit: gp
  mass:
    value: 5.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/spikes_iron_1_gp
  implementation: dnd2024:equipment_spikes_iron_1_gp
  parameters:
    purchase_quantity: 10
    hammer:
      action: utilize
      requires: blunt_object
      materials:
      - wood
      - earth
      - similar_material
      similar_material_authority: dm
      operations:
      - jam_door_shut
      - provide_rope_anchor
      - provide_chain_anchor
    mass:
      value: 5.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_spikes_iron_1_gp
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
  - dnd2024:equipment_spikes_iron_1_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 2e9d87a67cc1cb87e7cbf8311c8e08fea5786ab54115d347b21b80067b2e692e
  line_span:
    first: 8650
    count: 6
splat_clauses:
- key: source_rules
  text: '# Spikes, Iron (1 GP) Iron Spikes come in bundles of ten. As a Utilize action, you can use a
    blunt object, such as a Light Hammer, to hammer a spike into wood, earth, or a similar material. You
    can do so to jam a door shut or to then tie a Rope or Chain to the Spike.'
  sha256: dba89581d46d46270d5df9c1482cddd4591bb1384aa381082b111eae5bb3c8c8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Spikes, Iron (1 GP)

<!-- SRD page 100 -->

Iron Spikes come in bundles of ten. As a Utilize\
action, you can use a blunt object, such as a Light\
Hammer, to hammer a spike into wood, earth, or a\
similar material. You can do so to jam a door shut or\
to then tie a Rope or Chain to the Spike.\
