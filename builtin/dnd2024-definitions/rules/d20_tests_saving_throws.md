---
splat_schema: 1
splat_id: f9acbe43-d563-53b5-a159-5e8a7268b7fc
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_saving_throws
splat_name: Saving Throws
splat_origin:
  key: dnd2024:reference/pages/007
splat_sources:
- title: SRD 5.2.1
  locator: pp. 7
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/d20_tests_saving_throws
  parameters:
    trigger: threat_requires_save
    result_source: triggering_effect
    voluntary_failure:
      optional: true
      roll: false
  implementation: dnd2024:rule_d20_tests_saving_throws
splat_contracts:
- id: dnd2024:rule_d20_tests_saving_throws
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - save_resolution
  events:
  - saving_throw.pending
  capabilities:
  - saving_throws
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 7
  engine_gaps: &id001
  - dnd2024:rule_d20_tests_saving_throws
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: cf0076fe1b72cf96218f66a506ba9f8ab98a430ef9e077655e719109e9cf2a5b
  parent_name: D20 Tests
  line_span:
    first: 189
    count: 10
splat_clauses:
- key: source_rules
  text: '# Saving Throws A saving throw—also called a save—represents an attempt to evade or resist a
    threat, such as a fiery explosion, a blast of poisonous gas, or a spell trying to invade your mind.
    You don’t normally choose to make a save; you must make one because your character or a monster (if
    you’re the GM) is at risk. A save’s result is detailed in the effect that caused it. If you don’t
    want to resist the effect, you can choose to fail the save without rolling.'
  sha256: 94879d4313d27fa9efbd456f3e3252aea0d47385a167e1aadfdb93561c0393d3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Saving Throws

<!-- SRD page 7 -->

A saving throw—also called a save—represents an\
attempt to evade or resist a threat, such as a fiery\
explosion, a blast of poisonous gas, or a spell trying\
to invade your mind. You don’t normally choose to\
make a save; you must make one because your char-\
acter or a monster (if you’re the GM) is at risk. A\
save’s result is detailed in the effect that caused it.\
If you don’t want to resist the effect, you can\
choose to fail the save without rolling.\
