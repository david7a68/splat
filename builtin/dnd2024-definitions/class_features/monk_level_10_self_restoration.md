---
splat_schema: 1
splat_id: 93e92cf7-fb60-58fc-b95f-3470fbd3f6d6
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_10_self_restoration
splat_name: 'Level 10: Self-Restoration'
splat_origin:
  key: dnd2024:reference/pages/052
splat_sources:
- title: SRD 5.2.1
  locator: pp. 52
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/monk_level_10_self_restoration
  parameters:
    condition_removal:
      event: actor.turn.end
      optional: true
      choose_maximum: 1
      conditions:
      - charmed
      - frightened
      - poisoned
    exhaustion_prevention_sources:
    - lack_of_food
    - lack_of_drink
    class_scope:
      class: monk
      minimum_level: 10
  implementation: dnd2024:class_features_monk_level_10_self_restoration
splat_contracts:
- id: dnd2024:class_features_monk_level_10_self_restoration
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
  - turn.end
  - exhaustion.pending
  capabilities:
  - character_features
  - conditions
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 52
  engine_gaps: &id001
  - dnd2024:class_features_monk_level_10_self_restoration
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 964679743e0f831c4daa0bfc67808b560db7ca2faabec102ca56796daf0cd76d
  parent_name: Monk
  line_span:
    first: 4315
    count: 7
splat_clauses:
- key: source_rules
  text: '# Level 10: Self-Restoration Through sheer force of will, you can remove one of the following
    conditions from yourself at the end of each of your turns: Charmed, Frightened, or Poisoned. In addition,
    forgoing food and drink doesn’t give you levels of Exhaustion.'
  sha256: 50e7ae16b1d26d707b7e22d6d5768e6395b3d239991ebe680c405310587b6eb6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 10: Self-Restoration

<!-- SRD page 52 -->

Through sheer force of will, you can remove one\
of the following conditions from yourself at the\
end of each of your turns: Charmed, Frightened, or\
Poisoned.\
In addition, forgoing food and drink doesn’t give\
you levels of Exhaustion.\
