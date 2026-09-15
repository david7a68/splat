---
splat_schema: 1
splat_id: 3e9d4faf-6804-5e6d-95cb-833bd7ede923
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_9_tactical_master
splat_name: 'Level 9: Tactical Master'
splat_origin:
  key: dnd2024:reference/pages/048
splat_sources:
- title: SRD 5.2.1
  locator: pp. 48
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/fighter_level_9_tactical_master
  parameters:
    trigger: weapon.attack
    requires:
      weapon_mastery_available_to_actor: true
    replace_mastery_for_this_attack:
      optional: true
      choices:
      - push
      - sap
      - slow
    class_scope:
      class: fighter
      minimum_level: 9
  implementation: dnd2024:class_features_fighter_level_9_tactical_master
splat_contracts:
- id: dnd2024:class_features_fighter_level_9_tactical_master
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
  - attack.roll_pending
  capabilities:
  - character_features
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 48
  engine_gaps: &id001
  - dnd2024:class_features_fighter_level_9_tactical_master
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 86a26209dae96ea27b641295a12eef948a99f7d31450cdd35e7580ed0c54cc88
  parent_name: Fighter
  line_span:
    first: 4028
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 9: Tactical Master When you attack with a weapon whose mastery property you can use,
    you can replace that property with the Push, Sap, or Slow property for that attack.'
  sha256: abbd603d6b11c739a6585d0464dceee028dc8700a863235c18e6a577ca45e510
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 9: Tactical Master

<!-- SRD page 48 -->

When you attack with a weapon whose mastery\
property you can use, you can replace that property\
with the Push, Sap, or Slow property for that attack.\
