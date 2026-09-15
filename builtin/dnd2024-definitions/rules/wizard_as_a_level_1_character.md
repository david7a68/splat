---
splat_schema: 1
splat_id: 8c1a7c72-b2d4-5cae-90b9-f383b5ecd878
splat_kind: rule
splat_key: dnd2024:rules/wizard_as_a_level_1_character
splat_name: As a Level 1 Character
splat_origin:
  key: dnd2024:reference/pages/077
splat_sources:
- title: SRD 5.2.1
  locator: pp. 77
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/wizard_as_a_level_1_character
  parameters:
    class_definition:
      key: dnd2024:classes/wizard
    grant_core_traits: all
    grant_features:
      class_level: 1
      from: class_definition
    feature_unlocks:
    - class_level: 18
      feature:
        key: dnd2024:class_features/wizard_level_18_spell_mastery
    - class_level: 19
      feature:
        key: dnd2024:class_features/wizard_level_19_epic_boon
    - class_level: 1
      feature:
        key: dnd2024:class_features/wizard_level_1_arcane_recovery
    - class_level: 1
      feature:
        key: dnd2024:class_features/wizard_level_1_ritual_adept
    - class_level: 1
      feature:
        key: dnd2024:class_features/wizard_level_1_spellcasting
    - class_level: 20
      feature:
        key: dnd2024:class_features/wizard_level_20_signature_spells
    - class_level: 2
      feature:
        key: dnd2024:class_features/wizard_level_2_scholar
    - class_level: 3
      feature:
        key: dnd2024:class_features/wizard_level_3_wizard_subclass
    - class_level: 4
      feature:
        key: dnd2024:class_features/wizard_level_4_ability_score_improvement
    - class_level: 5
      feature:
        key: dnd2024:class_features/wizard_level_5_memorize_spell
    recurring_feature_levels:
      ability_score_improvement:
      - 4
      - 8
      - 12
      - 16
      subclass:
      - 3
      - 6
      - 10
      - 14
  implementation: dnd2024:rules_wizard_as_a_level_1_character
splat_contracts:
- id: dnd2024:rules_wizard_as_a_level_1_character
  inputs:
    actor:
      type: participant
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - character_updates
  events: []
  capabilities:
  - character_creation
  - progression
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 77
  engine_gaps: &id001
  - dnd2024:rules_wizard_as_a_level_1_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 6dc066126837d539e5503232f232a99acbb0a45b94d4523c4d69e05223d6b99a
  parent_name: Wizard
  line_span:
    first: 6545
    count: 28
splat_clauses:
- key: source_rules
  text: '# As a Level 1 Character • Gain all the traits in the Core Wizard Traits table. • Gain the Wizard’s
    level 1 features, which are listed in the Wizard Features table. Wizard Features Proficiency Level
    Bonus Class Features Cantrips 1 +2 Spellcasting, Ritual Adept, Arcane Recovery 2 +2 Scholar 3 +2 Wizard
    Subclass 4 +2 Ability Score Improvement 5 +3 Memorize Spell 6 +3 Subclass feature 7 +3 — 8 +3 Ability
    Score Improvement 9 +4 — 10 +4 Subclass feature 11 +4 — 12 +4 Ability Score Improvement 13 +5 — 14
    +5 Subclass feature 15 +5 — 16 +5 Ability Score Improvement 17 +6 — 18 +6 Spell Mastery 19 +6 Epic
    Boon 20 +6 Signature Spells'
  sha256: f7c87664874a44d4d149192e0b088a44c6efe65a58838c9ed6aaa046a10e5c97
  status: contract_specified
splat_requires_mechanics: *id001
---

# As a Level 1 Character

<!-- SRD page 77 -->

•  Gain all the traits in the Core Wizard Traits table.\
•  Gain the Wizard’s level 1 features, which are\
listed in the Wizard Features table.\
Wizard Features\
Proficiency\
Level Bonus Class Features Cantrips\
1 +2 Spellcasting, Ritual Adept,\
Arcane Recovery\
2 +2 Scholar\
3 +2 Wizard Subclass\
4 +2 Ability Score Improvement\
5 +3 Memorize Spell\
6 +3 Subclass feature\
7 +3 —\
8 +3 Ability Score Improvement\
9 +4 —\
10 +4 Subclass feature\
11 +4 —\
12 +4 Ability Score Improvement\
13 +5 —\
14 +5 Subclass feature\
15 +5 —\
16 +5 Ability Score Improvement\
17 +6 —\
18 +6 Spell Mastery\
19 +6 Epic Boon\
20 +6 Signature Spells\
