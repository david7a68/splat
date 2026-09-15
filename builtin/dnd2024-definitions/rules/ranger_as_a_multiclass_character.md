---
splat_schema: 1
splat_id: 6592c6b4-936e-5dd1-9a7d-69a9e85939bb
splat_kind: rule
splat_key: dnd2024:rules/ranger_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin:
  key: dnd2024:reference/pages/057
splat_sources:
- title: SRD 5.2.1
  locator: pp. 57
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/ranger_as_a_multiclass_character
  parameters:
    class:
      key: dnd2024:classes/ranger
    entry_mode: multiclass
    hit_die_sides: 10
    armor_training:
    - light
    - medium
    - shield
    skill_choice:
      count: 1
      options:
      - animal_handling
      - athletics
      - insight
      - investigation
      - nature
      - perception
      - stealth
      - survival
    musical_instrument_choice_count: 0
    weapon_proficiencies:
    - martial
    grant_features_at_class_level: 1
    spell_slots:
      key: dnd2024:rules/multiclassing_spellcasting
    level_one_features:
    - key: dnd2024:class_features/ranger_level_1_favored_enemy
    - key: dnd2024:class_features/ranger_level_1_spellcasting
    - key: dnd2024:class_features/ranger_level_1_weapon_mastery
  implementation: dnd2024:rule_ranger_as_a_multiclass_character
splat_contracts:
- id: dnd2024:rule_ranger_as_a_multiclass_character
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - character_changes
  events: []
  capabilities:
  - multiclassing
  - classes
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 57
  engine_gaps: &id001
  - dnd2024:rule_ranger_as_a_multiclass_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 7bb6befe558f7a14d538482c8854616702ad31d4a1d7713e838114eccbb00f66
  parent_name: Ranger
  line_span:
    first: 4815
    count: 10
splat_clauses:
- key: source_rules
  text: '# As a Multiclass Character • Gain the following traits from the Core Ranger Traits table: Hit
    Point Die, proficiency with Martial weapons, proficiency in one skill of your choice from the Ranger’s
    skill list, and training with Light and Medium armor and Shields. • Gain the Ranger’s level 1 features,
    which are listed in the Ranger Features table. See the multiclassing rules in “Character Creation”
    to determine your available spell slots.'
  sha256: 151256573243feaa7d81ff829e9138a1e4cfc5cf9bbd992bdb94bb0485e57f72
  status: contract_specified
splat_requires_mechanics: *id001
---

# As a Multiclass Character

<!-- SRD page 57 -->

•  Gain the following traits from the Core Ranger\
Traits table: Hit Point Die, proficiency with Mar-\
tial weapons, proficiency in one skill of your\
choice from the Ranger’s skill list, and training\
with Light and Medium armor and Shields.\
•  Gain the Ranger’s level 1 features, which are\
listed in the Ranger Features table. See the mul-\
ticlassing rules in “Character Creation” to deter-\
mine your available spell slots.\
