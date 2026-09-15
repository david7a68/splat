---
splat_schema: 1
splat_id: d1ea6b55-eb25-5dcb-915e-b5b8c0ffb000
splat_kind: rule
splat_key: dnd2024:rules/rogue_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin:
  key: dnd2024:reference/pages/061
splat_sources:
- title: SRD 5.2.1
  locator: pp. 61
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/rogue_as_a_multiclass_character
  parameters:
    class:
      key: dnd2024:classes/rogue
    entry: multiclass
    hit_die_sides: 8
    armor_training:
    - light
    features:
    - level: 1
      feature:
        key: dnd2024:class_features/rogue_level_1_expertise
    - level: 1
      feature:
        key: dnd2024:class_features/rogue_level_1_sneak_attack
    - level: 1
      feature:
        key: dnd2024:class_features/rogue_level_1_thieves_cant
    - level: 1
      feature:
        key: dnd2024:class_features/rogue_level_1_weapon_mastery
    tool_proficiencies:
    - thieves_tools
    skill_choice:
      count: 1
      options:
      - acrobatics
      - athletics
      - deception
      - insight
      - intimidation
      - investigation
      - perception
      - persuasion
      - sleight_of_hand
      - stealth
  implementation: dnd2024:rules_rogue_as_a_multiclass_character
splat_contracts:
- id: dnd2024:rules_rogue_as_a_multiclass_character
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - multiclass_grants
  events: []
  capabilities:
  - multiclassing
  - character_features
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 61
  engine_gaps: &id001
  - dnd2024:rules_rogue_as_a_multiclass_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: b3c3b126e3a843027d1cf724572f79f237796028dfa1a8760732b81c9be79e39
  parent_name: Rogue
  line_span:
    first: 5153
    count: 8
splat_clauses:
- key: source_rules
  text: '# As a Multiclass Character • Gain the following traits from the Core Rogue Traits table: Hit
    Point Die, proficiency in one skill of your choice from the Rogue’s skill list, proficiency with Thieves’
    Tools, and training with Light armor. • Gain the Rogue’s level 1 features, which are listed in the
    Rogue Features table.'
  sha256: fa7db62ae15abab099d7d1509902597426643295743fa5c571189af1bb2a8d02
  status: contract_specified
splat_requires_mechanics: *id001
---

# As a Multiclass Character

<!-- SRD page 61 -->

•  Gain the following traits from the Core Rogue\
Traits table: Hit Point Die, proficiency in one skill\
of your choice from the Rogue’s skill list, pro-\
ficiency with Thieves’ Tools, and training with\
Light armor.\
•  Gain the Rogue’s level 1 features, which are listed\
in the Rogue Features table.\
