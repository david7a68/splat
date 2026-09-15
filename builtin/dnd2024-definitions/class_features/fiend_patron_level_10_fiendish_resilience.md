---
splat_schema: 1
splat_id: d4d4815c-1dc1-5546-81a7-0429135dfedf
splat_kind: rule
splat_key: dnd2024:class_features/fiend_patron_level_10_fiendish_resilience
splat_name: 'Level 10: Fiendish Resilience'
splat_origin:
  key: dnd2024:reference/pages/076
splat_sources:
- title: SRD 5.2.1
  locator: pp. 76
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/fiend_patron_level_10_fiendish_resilience
  parameters:
    choose_damage_type:
      count: 1
      exclude:
      - force
      events:
      - rest.short_completed
      - rest.long_completed
    grant: resistance
    duration: until_different_choice
    class_scope:
      class: warlock
      minimum_level: 10
  implementation: dnd2024:class_features_fiend_patron_level_10_fiendish_resilience
splat_contracts:
- id: dnd2024:class_features_fiend_patron_level_10_fiendish_resilience
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - rest.short_completed
  - rest.long_completed
  - damage.pending
  capabilities:
  - character_features
  - damage
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 76
  engine_gaps: &id001
  - dnd2024:class_features_fiend_patron_level_10_fiendish_resilience
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: a852086ec40aa36503dee58f5db421cc77261d3f294e216749b3b1729b30bda6
  parent_name: Fiend Patron
  line_span:
    first: 6509
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 10: Fiendish Resilience Choose one damage type, other than Force, whenever you finish
    a Short or Long Rest. You have Resistance to that damage type until you choose a different one with
    this feature.'
  sha256: c52eea2283fa61dfdfc75681805edece1fb36e2058f2b49960497917c6f8c7d9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 10: Fiendish Resilience

<!-- SRD page 76 -->

Choose one damage type, other than Force, when-\
ever you finish a Short or Long Rest. You have Resis-\
tance to that damage type until you choose a differ-\
ent one with this feature.\
