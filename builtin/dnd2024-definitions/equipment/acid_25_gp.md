---
splat_schema: 1
splat_id: 3c0faf22-8ca2-5423-a190-b444a5dfc8b9
splat_kind: item
splat_key: dnd2024:equipment/acid_25_gp
splat_name: Acid (25 GP)
splat_origin:
  key: dnd2024:reference/pages/094
splat_sources:
- title: SRD 5.2.1
  locator: pp. 94
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 25
    unit: gp
  mass:
    value: 1.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/acid_25_gp
  implementation: dnd2024:equipment_acid_25_gp
  parameters:
    use:
      economy: replace_one_attack
      action: attack
      consume: 1
      target:
        count: 1
        kinds:
        - creature
        - object
        visible: true
        range_ft: 20
      save:
        ability: dexterity
        dc:
          op: add
          operands:
          - 8
          - ability_modifier: dexterity
          - stat: proficiency_bonus
        success: no_effect
        failure:
          damage:
            amount:
              dice:
                count: 2
                sides: 6
              bonus: 0
            type: acid
    mass:
      value: 1.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_acid_25_gp
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
  - 94
  engine_gaps: &id001
  - dnd2024:equipment_acid_25_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 1967f6eb15ee5fa319aea5823e56024f331b251332c226d00ce783dda835d38c
  line_span:
    first: 8144
    count: 8
splat_clauses:
- key: source_rules
  text: '# Acid (25 GP) When you take the Attack action, you can replace one of your attacks with throwing
    a vial of Acid. Target one creature or object you can see within 20 feet of yourself. The target must
    succeed on a Dexterity saving throw (DC 8 plus your Dexterity modifier and Proficiency Bonus) or take
    2d6 Acid damage.'
  sha256: 629e7bb1b7af12ac33ebec0e8777d0eb933c4037748d2ef7aa6520ed56a93b09
  status: contract_specified
splat_requires_mechanics: *id001
---

# Acid (25 GP)

<!-- SRD page 94 -->

When you take the Attack action, you can replace\
one of your attacks with throwing a vial of Acid.\
Target one creature or object you can see within\
20 feet of yourself. The target must succeed on a\
Dexterity saving throw (DC 8 plus your Dexterity\
modifier and Proficiency Bonus) or take 2d6 Acid\
damage.\
