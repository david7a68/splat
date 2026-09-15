---
splat_schema: 1
splat_id: e5202fd8-e03e-50fe-b1db-92c78964c015
splat_kind: rule
splat_key: dnd2024:class_features/warlock_level_2_magical_cunning
splat_name: 'Level 2: Magical Cunning'
splat_origin:
  key: dnd2024:reference/pages/072
splat_sources:
- title: SRD 5.2.1
  locator: pp. 72
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/warlock_level_2_magical_cunning
  parameters:
    activity_duration:
      minutes: 1
    on_completion:
      recover_resource: pact_magic_spell_slots
      maximum_recovered:
        op: ceil
        args:
        - op: divide
          args:
          - stat: pact_magic.maximum_slots
          - 2
    uses:
      maximum: 1
      recover:
        event: rest.long_rest_completed
        amount: all
  implementation: dnd2024:class_features_warlock_level_2_magical_cunning
splat_contracts:
- id: dnd2024:class_features_warlock_level_2_magical_cunning
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  - resource_changes
  events:
  - activity.completed
  capabilities:
  - resources
  - spell_slots
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 72
  engine_gaps: &id001
  - dnd2024:class_features_warlock_level_2_magical_cunning
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: fd2d47cd67fe77588a09482d90059efe57148b1e2c06134984d6ae5154b1fa12
  parent_name: Warlock
  line_span:
    first: 6089
    count: 6
splat_clauses:
- key: source_rules
  text: '# Level 2: Magical Cunning You can perform an esoteric rite for 1 minute. At the end of it, you
    regain expended Pact Magic spell slots but no more than a number equal to half your maximum (round
    up). Once you use this feature, you can’t do so again until you finish a Long Rest.'
  sha256: 0fcd0363ab87c3ddc557e6ea1ae1ef83ed81da1aa8a5eadafed3d26a8ccbc2e7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 2: Magical Cunning

<!-- SRD page 72 -->

You can perform an esoteric rite for 1 minute. At\
the end of it, you regain expended Pact Magic spell\
slots but no more than a number equal to half your\
maximum (round up). Once you use this feature, you\
can’t do so again until you finish a Long Rest.\
