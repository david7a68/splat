---
splat_schema: 1
splat_id: c615f96a-0485-54ce-9e1e-9686fefdd8cf
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_6_roving
splat_name: 'Level 6: Roving'
splat_origin:
  key: dnd2024:reference/pages/059
splat_sources:
- title: SRD 5.2.1
  locator: pp. 59
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/ranger_level_6_roving
  parameters:
    speed_bonus:
      feet: 10
      unless_armor: heavy
    movement_speeds:
      climb: &id001
        stat: speed
      swim: *id001
  implementation: dnd2024:class_features_ranger_level_6_roving
splat_contracts:
- id: dnd2024:class_features_ranger_level_6_roving
  inputs:
    actor:
      type: participant
      required: true
    destination:
      required: false
      type: location
  outputs:
  - feature_state
  - movement
  events:
  - character.feature_gained
  capabilities:
  - movement
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 59
  engine_gaps: &id002
  - dnd2024:class_features_ranger_level_6_roving
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 836b9c85a0e163bbbd041465c589c1ad9711696b0aafcc876d660789ee2a5e87
  parent_name: Ranger
  line_span:
    first: 4967
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 6: Roving Your Speed increases by 10 feet while you aren’t wearing Heavy armor. You also
    have a Climb Speed and a Swim Speed equal to your Speed.'
  sha256: a676e7e501cc922b9bcbda3fda9f438dbbadf7bcf887d5f958a0bf65f6095ba9
  status: contract_specified
splat_requires_mechanics: *id002
---

# Level 6: Roving

<!-- SRD page 59 -->

Your Speed increases by 10 feet while you aren’t\
wearing Heavy armor. You also have a Climb Speed\
and a Swim Speed equal to your Speed.\
