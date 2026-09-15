---
splat_schema: 1
splat_id: e9a0df6f-d859-5bab-be28-157e061d05b1
splat_kind: item
splat_key: dnd2024:equipment/rope_1_gp
splat_name: Rope (1 GP)
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
    value: 1
    unit: gp
  mass:
    value: 5.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/rope_1_gp
  implementation: dnd2024:equipment_rope_1_gp
  parameters:
    tie_knot:
      action: utilize
      check:
        ability: dexterity
        skill: sleight_of_hand
        dc: 10
    bind:
      unwilling_target_requires_any_condition:
      - grappled
      - incapacitated
      - restrained
      bound_legs_condition: restrained
      until: escape
    burst:
      check:
        ability: strength
        skill: athletics
        dc: 20
    escape:
      economy: action
      check:
        ability: dexterity
        skill: acrobatics
        dc: 15
    mass:
      value: 5.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_rope_1_gp
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
  - dnd2024:equipment_rope_1_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 1c9daf857cf3bb8bbe63959f5dd109a4dbe688612838863faa39f501cf06a788
  line_span:
    first: 8612
    count: 12
splat_clauses:
- key: source_rules
  text: '# Rope (1 GP) As a Utilize action, you can tie a knot with Rope if you succeed on a DC 10 Dexterity
    (Sleight of Hand) check. The Rope can be burst with a successful DC 20 Strength (Athletics) check.
    You can bind an unwilling creature with the Rope only if the creature has the Grappled, Incapacitated,
    or Restrained condition. If the creature’s legs are bound, the creature has the Restrained condition
    until it escapes. Escaping the Rope requires the creature to make a successful DC 15 Dexterity (Acrobatics)
    check as an action.'
  sha256: 584527b4ba006719a5351f55ae74acfdc10a1e5cc7526e0c302c428f1371a9e5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rope (1 GP)

<!-- SRD page 99 -->

As a Utilize action, you can tie a knot with Rope if\
you succeed on a DC 10 Dexterity (Sleight of Hand)\
check. The Rope can be burst with a successful DC\
20 Strength (Athletics) check.\
You can bind an unwilling creature with the Rope\
only if the creature has the Grappled, Incapacitated,\
or Restrained condition. If the creature’s legs are\
bound, the creature has the Restrained condition\
until it escapes. Escaping the Rope requires the\
creature to make a successful DC 15 Dexterity (Ac-\
robatics) check as an action.\
