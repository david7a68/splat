---
splat_schema: 1
splat_id: 7d376d29-a16c-583d-9111-e4a57333c52b
splat_kind: rule
splat_key: dnd2024:class_features/college_of_lore_level_3_bonus_proficiencies
splat_name: 'Level 3: Bonus Proficiencies'
splat_origin:
  key: dnd2024:reference/pages/035
splat_sources:
- title: SRD 5.2.1
  locator: pp. 35
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/college_of_lore_level_3_bonus_proficiencies
  parameters:
    skill_proficiency_choice:
      count: 3
      category: any_skill
  implementation: dnd2024:class_features_college_of_lore_level_3_bonus_proficiencies
splat_contracts:
- id: dnd2024:class_features_college_of_lore_level_3_bonus_proficiencies
  inputs:
    actor:
      type: participant
      required: true
    selections:
      required: false
      type: choice
  outputs:
  - feature_state
  - choices
  - proficiencies
  events:
  - character.feature_gained
  capabilities:
  - character_choices
  - proficiencies
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 35
  engine_gaps: &id001
  - dnd2024:class_features_college_of_lore_level_3_bonus_proficiencies
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 7142e56bdc64301153d5b52af6345d45daf4be1caaaee2a41c9dc6da2cb7755a
  parent_name: College of Lore
  line_span:
    first: 2882
    count: 2
splat_clauses:
- key: source_rules
  text: '# Level 3: Bonus Proficiencies You gain proficiency with three skills of your choice.'
  sha256: 7ce47eb580936721288309ae501765ba6269dabd437cb2848decc279326d70aa
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Bonus Proficiencies

<!-- SRD page 35 -->

You gain proficiency with three skills of your choice.\
