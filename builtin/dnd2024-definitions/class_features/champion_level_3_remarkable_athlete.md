---
splat_schema: 1
splat_id: 54abaa45-4ef6-5af9-a91a-0de7960bc28a
splat_kind: rule
splat_key: dnd2024:class_features/champion_level_3_remarkable_athlete
splat_name: 'Level 3: Remarkable Athlete'
splat_origin:
  key: dnd2024:reference/pages/049
splat_sources:
- title: SRD 5.2.1
  locator: pp. 49
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/champion_level_3_remarkable_athlete
  parameters:
    advantage_on:
    - roll: initiative
    - roll: ability_check
      ability: strength
      skill: athletics
    after_critical_hit:
      optional_move_maximum_feet:
        op: divide
        args:
        - stat: speed
        - 2
      provokes_opportunity_attacks: false
  implementation: dnd2024:class_features_champion_level_3_remarkable_athlete
splat_contracts:
- id: dnd2024:class_features_champion_level_3_remarkable_athlete
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
  - initiative.pending
  - ability_check.pending
  - attack.critical_hit
  capabilities:
  - roll_modifiers
  - movement
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 49
  engine_gaps: &id001
  - dnd2024:class_features_champion_level_3_remarkable_athlete
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: eac3bdc43b3697709be1acd5f984efa00a679aa6419dd7338140cb3db58e9470
  parent_name: Champion
  line_span:
    first: 4061
    count: 6
splat_clauses:
- key: source_rules
  text: '# Level 3: Remarkable Athlete Thanks to your athleticism, you have Advantage on Initiative rolls
    and Strength (Athletics) checks. In addition, immediately after you score a Critical Hit, you can
    move up to half your Speed without provoking Opportunity Attacks.'
  sha256: 1d26ee09d5aba253aedc95b8702dc27cd4f64fbba68041e3abc66f0e7a3032df
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Remarkable Athlete

<!-- SRD page 49 -->

Thanks to your athleticism, you have Advantage on\
Initiative rolls and Strength (Athletics) checks.\
In addition, immediately after you score a Critical\
Hit, you can move up to half your Speed without\
provoking Opportunity Attacks.\
