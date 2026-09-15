---
splat_schema: 1
splat_id: 4ce1c2c7-f289-5518-80fc-0fabc2dac658
splat_kind: rule
splat_key: dnd2024:glossary/ability_check
splat_name: Ability Check
splat_origin:
  key: dnd2024:reference/glossary/ability_check
splat_sources:
- title: SRD 5.2.1
  locator: pp. 176
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/ability_check
  parameters:
    test_kind: ability_check
    base_rule:
      key: dnd2024:glossary/d20_test
    abilities:
    - strength
    - dexterity
    - constitution
    - intelligence
    - wisdom
    - charisma
    skill_optional: true
  implementation: dnd2024:glossary_ability_check
splat_contracts:
- id: dnd2024:glossary_ability_check
  inputs:
    subject:
      type: participant
      required: true
    ability:
      type: choice
      required: true
    skill:
      required: false
      type: definition
  outputs:
  - check_definition
  events: []
  capabilities:
  - checks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 45e24942dd5c4205c64172bee10920748d1973872b1c0529761c27499a7cf8f3
  source_path: builtin/srd-5.2.1/glossary/ability_check.md
  pages:
  - 176
  engine_gaps: &id001
  - dnd2024:glossary_ability_check
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: f6e70cb404593474b0ec0ac188b0c272f868af0e3359b0521dd2ddb953d05d3b
splat_clauses:
- key: source_rules
  text: '# Ability Check An ability check is a D20 Test that represents using one of the six abilities—or
    a specific skill associated with an ability—to overcome a challenge. See also “Playing the Game” (“D20
    Tests” and “Proficiency”).'
  sha256: ccb904adc973751cf5b058d6beb5f482b7e1b41c773e03f3b31a1361d2cd8a8b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ability Check

<!-- SRD page 176 -->

An ability check is a D20 Test that represents using\
one of the six abilities—or a specific skill associated\
with an ability—to overcome a challenge. See also\
“Playing the Game” (“D20 Tests” and “Proficiency”).\

Original layout: [[../../srd-5.2.1/pages/176|page 176]].
