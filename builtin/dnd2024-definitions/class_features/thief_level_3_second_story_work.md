---
splat_schema: 1
splat_id: 42c2ba87-b5c4-5a89-bbc9-441fa04b65e9
splat_kind: rule
splat_key: dnd2024:class_features/thief_level_3_second_story_work
splat_name: 'Level 3: Second-Story Work'
splat_origin:
  key: dnd2024:reference/pages/064
splat_sources:
- title: SRD 5.2.1
  locator: pp. 64
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/thief_level_3_second_story_work
  parameters:
    climb_speed:
      stat: speed
    jump_distance_ability_substitution:
      optional: true
      from: strength
      to: dexterity
  implementation: dnd2024:class_features_thief_level_3_second_story_work
splat_contracts:
- id: dnd2024:class_features_thief_level_3_second_story_work
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
  - 64
  engine_gaps: &id001
  - dnd2024:class_features_thief_level_3_second_story_work
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 5484b44884fd13102069636514ef612e6dc3581818a1c8cf78409aa68a2800d8
  parent_name: Thief
  line_span:
    first: 5366
    count: 7
splat_clauses:
- key: source_rules
  text: '# Level 3: Second-Story Work You’ve trained to get into especially hard-to-reach places, granting
    you these benefits. Climber. You gain a Climb Speed equal to your Speed. Jumper. You can determine
    your jump distance using your Dexterity rather than your Strength.'
  sha256: c6247bb86e84abf907b25c4665280aea6001ac5fb7bb7f28b5d6d66167ca5cd9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Second-Story Work

<!-- SRD page 64 -->

You’ve trained to get into especially hard-to-reach\
places, granting you these benefits.\
Climber. You gain a Climb Speed equal to your\
Speed.\
Jumper. You can determine your jump distance\
using your Dexterity rather than your Strength.\
