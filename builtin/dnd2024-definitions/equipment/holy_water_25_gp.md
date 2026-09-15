---
splat_schema: 1
splat_id: a0d4116d-36c2-52de-af3b-ce5dfa01e7c1
splat_kind: item
splat_key: dnd2024:equipment/holy_water_25_gp
splat_name: Holy Water (25 GP)
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
    value: 25
    unit: gp
  mass:
    value: 1.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/holy_water_25_gp
  implementation: dnd2024:equipment_holy_water_25_gp
  parameters:
    use:
      economy: replace_one_attack
      action: attack
      consume: 1
      target:
        count: 1
        kinds:
        - creature
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
                sides: 8
              bonus: 0
            type: radiant
          only_if_creature_type_in:
          - fiend
          - undead
    mass:
      value: 1.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_holy_water_25_gp
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
  - dnd2024:equipment_holy_water_25_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 3129459729ab3a3fe77177aeca0532635fcb9ffdf8ecf776e185197d39f9d224
  line_span:
    first: 8433
    count: 8
splat_clauses:
- key: source_rules
  text: '# Holy Water (25 GP) When you take the Attack action, you can replace one of your attacks with
    throwing a flask of Holy Water. Target one creature you can see within 20 feet of yourself. The target
    must succeed on a Dexterity saving throw (DC 8 plus your Dexterity modifier and Proficiency Bonus)
    or take 2d8 Radiant damage if it is a Fiend or an Undead.'
  sha256: 736de61fbd7164bd2e04e56a1006e7bd96f0fd21ccccae9c71b1a9c4bae4a72f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Holy Water (25 GP)

<!-- SRD page 97 -->

When you take the Attack action, you can replace\
one of your attacks with throwing a flask of Holy\
Water. Target one creature you can see within 20\
feet of yourself. The target must succeed on a Dex-\
terity saving throw (DC 8 plus your Dexterity mod-\
ifier and Proficiency Bonus) or take 2d8 Radiant\
damage if it is a Fiend or an Undead.\
