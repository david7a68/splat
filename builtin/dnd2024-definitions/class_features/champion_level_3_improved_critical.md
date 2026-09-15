---
splat_schema: 1
splat_id: 2067f60b-da8d-51ff-a1cb-2a7483cb44bd
splat_kind: rule
splat_key: dnd2024:class_features/champion_level_3_improved_critical
splat_name: 'Level 3: Improved Critical'
splat_origin:
  key: dnd2024:reference/pages/049
splat_sources:
- title: SRD 5.2.1
  locator: pp. 49
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/champion_level_3_improved_critical
  parameters:
    attack_sources:
    - weapon
    - unarmed_strike
    critical_d20_faces:
    - 19
    - 20
    class_scope:
      class: fighter
      minimum_level: 3
  implementation: dnd2024:class_features_champion_level_3_improved_critical
splat_contracts:
- id: dnd2024:class_features_champion_level_3_improved_critical
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
  - attack.roll
  capabilities:
  - character_features
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 49
  engine_gaps: &id001
  - dnd2024:class_features_champion_level_3_improved_critical
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 9142d2a22705950c1aab38ccac74fa2bd13dc43b372cf3b52833a816b8488446
  parent_name: Champion
  line_span:
    first: 4057
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 3: Improved Critical Your attack rolls with weapons and Unarmed Strikes can score a Critical
    Hit on a roll of 19 or 20 on the d20.'
  sha256: f1438f9a8caf7f6cdaf80b4e97a77ed5b3e259be28afb82a7aca324fa0fb6f8a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Improved Critical

<!-- SRD page 49 -->

Your attack rolls with weapons and Unarmed\
Strikes can score a Critical Hit on a roll of 19 or 20\
on the d20.\
