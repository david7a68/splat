---
splat_schema: 1
splat_id: 69fa7c15-e235-5184-bfee-dcc1f9731f0b
splat_kind: rule
splat_key: dnd2024:class_features/druid_level_1_druidic
splat_name: 'Level 1: Druidic'
splat_origin:
  key: dnd2024:reference/pages/042
splat_sources:
- title: SRD 5.2.1
  locator: pp. 42
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/druid_level_1_druidic
  parameters:
    language_grant: druidic
    always_prepared:
    - key: dnd2024:spells/speak_with_animals
    hidden_messages:
      can_create: true
      druidic_knowers:
        automatically_spot: true
        can_read: true
      others:
        spot_check:
          ability: intelligence
          skill: investigation
          dc: 15
        decipher_requires_magic: true
  implementation: dnd2024:class_features_druid_level_1_druidic
splat_contracts:
- id: dnd2024:class_features_druid_level_1_druidic
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  events:
  - character.feature_gained
  capabilities:
  - languages
  - spell_preparation
  - ability_checks
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 42
  engine_gaps: &id001
  - dnd2024:class_features_druid_level_1_druidic
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: cdb3d11c701e7ec5aee5b73a721761cd7997075c9492fa512e16a8a7d5ee5e50
  parent_name: Druid
  line_span:
    first: 3472
    count: 11
splat_clauses:
- key: source_rules
  text: '# Level 1: Druidic You know Druidic, the secret language of Druids. While learning this ancient
    tongue, you also unlocked the magic of communicating with animals; you always have the Speak with
    Animals spell prepared. You can use Druidic to leave hidden messages. You and others who know Druidic
    automatically spot such a message. Others spot the message’s presence with a successful DC 15 Intelligence
    (Investigation) check but can’t decipher it without magic.'
  sha256: c9e2d4d60e3e746aea68506da52bd028ee841216cf663d4ae8a6efe44b72ab27
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Druidic

<!-- SRD page 42 -->

You know Druidic, the secret language of Druids.\
While learning this ancient tongue, you also un-\
locked the magic of communicating with animals;\
you always have the Speak with Animals spell\
prepared.\
You can use Druidic to leave hidden messages. You\
and others who know Druidic automatically spot\
such a message. Others spot the message’s presence\
with a successful DC 15 Intelligence (Investigation)\
check but can’t decipher it without magic.\
