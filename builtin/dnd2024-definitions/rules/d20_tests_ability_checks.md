---
splat_schema: 1
splat_id: 5da0359a-5e88-508c-a5a6-e35a5944b18b
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_ability_checks
splat_name: Ability Checks
splat_origin:
  key: dnd2024:reference/pages/006
splat_sources:
- title: SRD 5.2.1
  locator: pp. 6
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/d20_tests_ability_checks
  parameters:
    check: ability_check
    eligibility:
      attack: false
      meaningful_failure_possible: true
      outcome_uncertain: true
      narrative_interest: true
    call_authority:
    - gm
    - applicable_rule
    ability_and_dc: gm_or_applicable_rule
  implementation: dnd2024:rules_d20_tests_ability_checks
splat_contracts:
- id: dnd2024:rules_d20_tests_ability_checks
  inputs:
    subject:
      type: participant
      required: true
    task:
      type: resolution
      required: true
  outputs:
  - check_request
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
  - dnd2024:rules_d20_tests_ability_checks
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 359daaf4e1a68843169c0e61aa09bbc48795f87baff5216c69f3f659fba0127b
  parent_name: D20 Tests
  line_span:
    first: 140
    count: 10
splat_clauses:
- key: source_rules
  text: '# Ability Checks An ability check represents a creature using talent and training to try to overcome
    a challenge, such as forcing open a stuck door, picking a lock, entertaining a crowd, or deciphering
    a cipher. The GM and the rules often call for an ability check when a creature attempts something
    other than an attack that has a chance of meaningful failure. When the outcome is uncertain and narratively
    interesting, the dice determine the result.'
  sha256: 932ec0c1b563a70f52194679732b8ff9ee070b9f4ed3305ad22836b5a055d6cc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ability Checks

<!-- SRD page 6 -->

An ability check represents a creature using talent\
and training to try to overcome a challenge, such as\
forcing open a stuck door, picking a lock, entertain-\
ing a crowd, or deciphering a cipher. The GM and the\
rules often call for an ability check when a creature\
attempts something other than an attack that has a\
chance of meaningful failure. When the outcome is\
uncertain and narratively interesting, the dice de-\
termine the result.\
