---
splat_schema: 1
splat_id: 5e635839-74d4-5538-a595-b3fde065bab6
splat_kind: rule
splat_key: dnd2024:rules/starting_at_higher_levels_starting_equipment
splat_name: Starting Equipment
splat_origin:
  key: dnd2024:reference/pages/024
splat_sources:
- title: SRD 5.2.1
  locator: pp. 24
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/starting_at_higher_levels_starting_equipment
  parameters:
    authority: gm
    table_is_guidance: true
    normal_starting_equipment_included: true
    availability_authority: gm
    starting_level_equipment:
    - levels:
      - 2
      - 4
      additional_gp: 0
      magic_items:
        common: 1
    - levels:
      - 5
      - 10
      additional_gp:
        base: 500
        roll:
          count: 1
          sides: 10
          add: 0
        multiplier: 25
      magic_items:
        common: 1
        uncommon: 1
    - levels:
      - 11
      - 16
      additional_gp:
        base: 5000
        roll:
          count: 1
          sides: 10
          add: 0
        multiplier: 250
      magic_items:
        common: 2
        uncommon: 3
        rare: 1
    - levels:
      - 17
      - 20
      additional_gp:
        base: 20000
        roll:
          count: 1
          sides: 10
          add: 0
        multiplier: 250
      magic_items:
        common: 2
        uncommon: 4
        rare: 3
        very_rare: 1
  implementation: dnd2024:rules_starting_at_higher_levels_starting_equipment
splat_contracts:
- id: dnd2024:rules_starting_at_higher_levels_starting_equipment
  inputs:
    actor:
      type: participant
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - character_updates
  events: []
  capabilities:
  - character_creation
  - progression
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 24
  engine_gaps: &id001
  - dnd2024:rules_starting_at_higher_levels_starting_equipment
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: dd3f244a410180132a3f2e0ae47c08538e24bf0d44b9728f9d9472568e7d2342
  parent_name: Starting at Higher Levels
  line_span:
    first: 1882
    count: 12
splat_clauses:
- key: source_rules
  text: '# Starting Equipment The GM decides whether your character starts with more than the standard
    equipment for a level 1 character, possibly even one or more magic items. The Starting Equipment at
    Higher Levels table is a guide for the GM. Also, check with your GM about what equipment is available
    for you to buy with your starting money. For example, the firearms described in “Equipment” are too
    expensive for level 1 characters, but they might be available for purchase if your GM allows them.'
  sha256: e29b60b5371cbd011636116678e6b00fb2d61ce28699e2a3f83b5395254eb8e1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Starting Equipment

<!-- SRD page 24 -->

The GM decides whether your character starts with\
more than the standard equipment for a level 1\
character, possibly even one or more magic items.\
The Starting Equipment at Higher Levels table is a\
guide for the GM.\
Also, check with your GM about what equipment\
is available for you to buy with your starting money.\
For example, the firearms described in “Equipment”\
are too expensive for level 1 characters, but they\
might be available for purchase if your GM allows\
them.\
