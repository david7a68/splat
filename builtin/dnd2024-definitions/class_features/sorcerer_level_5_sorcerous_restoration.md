---
splat_schema: 1
splat_id: 7cf2b7d8-d092-5ede-adbc-77b4ab2cacff
splat_kind: rule
splat_key: dnd2024:class_features/sorcerer_level_5_sorcerous_restoration
splat_name: 'Level 5: Sorcerous Restoration'
splat_origin:
  key: dnd2024:reference/pages/066
splat_sources:
- title: SRD 5.2.1
  locator: pp. 66
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/sorcerer_level_5_sorcerous_restoration
  parameters:
    trigger: rest.short_completed
    optional: true
    resource:
      id: sorcerer.sorcerous_restoration
      maximum: 1
      recovery:
      - event: rest.long_completed
        amount: all
    cost: 1
    restore:
      resource: sorcerer.sorcery_points
      maximum:
        op: floor
        args:
        - op: divide
          args:
          - var: actor.class_levels.sorcerer
          - 2
      expended_only: true
    class_scope:
      class: sorcerer
      minimum_level: 5
  implementation: dnd2024:class_features_sorcerer_level_5_sorcerous_restoration
splat_contracts:
- id: dnd2024:class_features_sorcerer_level_5_sorcerous_restoration
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - resources
  events:
  - rest.short_completed
  - rest.long_completed
  capabilities:
  - character_features
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 66
  engine_gaps: &id001
  - dnd2024:class_features_sorcerer_level_5_sorcerous_restoration
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 02a6b126f104ff6175794052b0e294bdb2dd5cc26ab5b7e6bc38a9442fd675d6
  parent_name: Sorcerer
  line_span:
    first: 5607
    count: 6
splat_clauses:
- key: source_rules
  text: '# Level 5: Sorcerous Restoration When you finish a Short Rest, you can regain expended Sorcery
    Points, but no more than a number equal to half your Sorcerer level (round down). Once you use this
    feature, you can’t do so again until you finish a Long Rest.'
  sha256: d9f56d99ba85c2a00c11ead99977871dcaa72608b390e50131df9b60786e9b54
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 5: Sorcerous Restoration

<!-- SRD page 66 -->

When you finish a Short Rest, you can regain ex-\
pended Sorcery Points, but no more than a number\
equal to half your Sorcerer level (round down). Once\
you use this feature, you can’t do so again until you\
finish a Long Rest.\
