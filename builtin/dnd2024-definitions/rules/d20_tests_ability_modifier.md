---
splat_schema: 1
splat_id: 2dd56827-5b78-584b-85aa-63c9d4405d47
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_ability_modifier
splat_name: Ability Modifier
splat_origin:
  key: dnd2024:reference/pages/006
splat_sources:
- title: SRD 5.2.1
  locator: pp. 6
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/d20_tests_ability_modifier
  parameters:
    ability_selection:
      actor: gm
      criterion: most_relevant_ability
    ability_examples:
      strength:
      - lift
      - push
      - pull
      - break
      dexterity:
      - nimble_movement
      - quick_movement
      - quiet_movement
      constitution:
      - exceed_endurance
      intelligence:
      - reason
      - remember
      wisdom:
      - notice_environment
      - notice_behavior
      charisma:
      - influence
      - entertain
      - deceive
    check_modifier: selected_ability_modifier
  implementation: dnd2024:rule_d20_tests_ability_modifier
splat_contracts:
- id: dnd2024:rule_d20_tests_ability_modifier
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - check_modifiers
  events: []
  capabilities:
  - checks
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 6
  engine_gaps: &id001
  - dnd2024:rule_d20_tests_ability_modifier
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: fc68c7b016b76c9e23b6f6e1f6078f543d8b4cae1e868775e5c4833ade7eda05
  parent_name: D20 Tests
  line_span:
    first: 150
    count: 16
splat_clauses:
- key: source_rules
  text: '# Ability Modifier An ability check is named for the ability modifier it uses: a Strength check,
    an Intelligence check, and so on. Different ability checks are called for in different situations,
    depending on which ability is most relevant. See the Ability Check Examples table for examples of
    each check’s use. Ability Check Examples Ability Make a Check To … Strength Lift, push, pull, or break
    something Dexterity Move nimbly, quickly, or quietly Constitution Push your body beyond normal limits
    Intelligence Reason or remember Wisdom Notice things in the environment or in creatures’ behavior
    Charisma Influence, entertain, or deceive'
  sha256: f9201a1957817c10a8e2b87afc0b23cc4b40b3c474557335f5dd2d2f1f843499
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ability Modifier

<!-- SRD page 6 -->

An ability check is named for the ability modifier it\
uses: a Strength check, an Intelligence check, and so\
on. Different ability checks are called for in differ-\
ent situations, depending on which ability is most\
relevant. See the Ability Check Examples table for\
examples of each check’s use.\
Ability Check Examples\
Ability Make a Check To …\
Strength Lift, push, pull, or break something\
Dexterity Move nimbly, quickly, or quietly\
Constitution Push your body beyond normal limits\
Intelligence Reason or remember\
Wisdom Notice things in the environment or in\
creatures’ behavior\
Charisma Influence, entertain, or deceive\
