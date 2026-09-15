---
splat_schema: 1
splat_id: dba9cc10-b4d9-5a2e-b9fb-2b1549c3f37f
splat_kind: rule
splat_key: dnd2024:class_features/champion_level_15_superior_critical
splat_name: 'Level 15: Superior Critical'
splat_origin:
  key: dnd2024:reference/pages/049
splat_sources:
- title: SRD 5.2.1
  locator: pp. 49
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/champion_level_15_superior_critical
  parameters:
    attack_sources:
    - weapon
    - unarmed_strike
    critical_d20_faces:
    - 18
    - 19
    - 20
    class_scope:
      class: fighter
      minimum_level: 15
  implementation: dnd2024:class_features_champion_level_15_superior_critical
splat_contracts:
- id: dnd2024:class_features_champion_level_15_superior_critical
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
  - dnd2024:class_features_champion_level_15_superior_critical
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: db109ccd2529d8f4028a8706eeffdd53502f4565b894cf9c5467e64752ba376a
  parent_name: Champion
  line_span:
    first: 4073
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 15: Superior Critical Your attack rolls with weapons and Unarmed Strikes can now score
    a Critical Hit on a roll of 18–20 on the d20.'
  sha256: d6884137622c15b2df96a5f1a46f3f3366e82a289b732ce3dbb9f955e7ccef73
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 15: Superior Critical

<!-- SRD page 49 -->

Your attack rolls with weapons and Unarmed\
Strikes can now score a Critical Hit on a roll of\
18–20 on the d20.\
