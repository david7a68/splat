---
splat_schema: 1
splat_id: b9c3b1bf-e82e-5824-a369-f3aaa80cda67
splat_kind: item
splat_key: dnd2024:equipment/alchemist_s_fire_50_gp
splat_name: Alchemist’s Fire (50 GP)
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
    value: 50
    unit: gp
  mass:
    value: 1.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/alchemist_s_fire_50_gp
  implementation: dnd2024:equipment_alchemist_s_fire_50_gp
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
                count: 1
                sides: 4
              bonus: 0
            type: fire
          apply_rule:
            key: dnd2024:glossary/burning_hazard
    mass:
      value: 1.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_alchemist_s_fire_50_gp
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
  - dnd2024:equipment_alchemist_s_fire_50_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: cd46c21981d1bd2d7aa99ebdbeb5d774cedc605ebd9dd3bb19adfa874ea415c5
  line_span:
    first: 8152
    count: 9
splat_clauses:
- key: source_rules
  text: '# Alchemist’s Fire (50 GP) When you take the Attack action, you can replace one of your attacks
    with throwing a flask of Alchemist’s Fire. Target one creature or object you can see within 20 feet
    of yourself. The target must succeed on a Dexterity saving throw (DC 8 plus your Dexterity modifier
    and Proficiency Bonus) or take 1d4 Fire damage and start burning (see “Rules Glossary”).'
  sha256: c53c4d807c0afd25c1709c6fdc56a0d83ec28f6ddc5b878f5741b7792d5a2570
  status: contract_specified
splat_requires_mechanics: *id001
---

# Alchemist’s Fire (50 GP)

<!-- SRD page 94 -->

When you take the Attack action, you can replace\
one of your attacks with throwing a flask of Al-\
chemist’s Fire. Target one creature or object you\
can see within 20 feet of yourself. The target must\
succeed on a Dexterity saving throw (DC 8 plus\
your Dexterity modifier and Proficiency Bonus) or\
take 1d4 Fire damage and start burning (see “Rules\
Glossary”).\
