---
splat_schema: 1
splat_id: 6966f659-f145-58f3-8002-4341356e7f21
splat_kind: rule
splat_key: dnd2024:rules/stat_block_overview
splat_name: Stat Block Overview
splat_origin:
  key: dnd2024:reference/pages/254
splat_sources:
- title: SRD 5.2.1
  locator: pp. 254
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/stat_block_overview
  parameters:
    stat_block_sections:
    - name_size_type_tags_alignment
    - ac_hp_speed_initiative
    - ability_scores_modifiers_saves
    - senses_languages_cr_optional_details
    - traits
    - actions
    - bonus_actions
    - reactions
    - legendary_actions
    traits_active: always_or_stated_situations
    action_options: stat_block_plus_universal_actions
    optional_detail_omission_means_absent: true
  implementation: dnd2024:rules_stat_block_overview
splat_contracts:
- id: dnd2024:rules_stat_block_overview
  inputs:
    definition:
      type: definition
      required: true
  outputs:
  - typed_attributes
  events: []
  capabilities:
  - definition_catalogs
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 254
  engine_gaps: &id001
  - dnd2024:rules_stat_block_overview
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 1a04a0bd21a4d069d1274ce23c40e2f5df28406073969bc1aa15b6644e862b21
  parent_name: Stat Block Overview
  line_span:
    first: 10862
    count: 24
splat_clauses:
- key: source_rules
  text: '# Stat Block Overview A monster has a stat block that contains the rules necessary to use it
    in the game. Stat blocks are divided into the following parts: Name and General Details. The name
    of the monster is followed by its size, creature type (along with any descriptive tags), and alignment.
    Combat Highlights. Armor Class, Hit Points, Speed, and Initiative are provided here. Ability Scores.
    A monster’s ability scores, ability modifiers, and saving throws are listed here. Other Details. The
    monster’s Senses, Languages, and CR entries appear here. Additional details appear in some monsters,
    such as skill proficiencies, Resistances, Immunities, and Gear. If a monster lacks those details,
    entries for them don’t appear. Traits. Monster characteristics that are active at all times or during
    certain situations appear here. Actions. The monster can take the actions here in addition to those
    in “Playing the Game.” Bonus Actions. This section provides a monster’s Bonus Actions, if any. Reactions
    and Legendary Actions. These sections provide Reactions and Legendary Actions, if any.'
  sha256: 8a530f67c8465b7aea46d1f0b4383c54d5fdcba9f611bf552bacaa44afeb8dc6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Stat Block Overview

<!-- SRD page 254 -->

A monster has a stat block that contains the rules\
necessary to use it in the game. Stat blocks are di-\
vided into the following parts:\
Name and General Details. The name of the mon-\
ster is followed by its size, creature type (along\
with any descriptive tags), and alignment.\
Combat Highlights. Armor Class, Hit Points, Speed,\
and Initiative are provided here.\
Ability Scores. A monster’s ability scores, ability\
modifiers, and saving throws are listed here.\
Other Details. The monster’s Senses, Languages,\
and CR entries appear here. Additional details ap-\
pear in some monsters, such as skill proficiencies,\
Resistances, Immunities, and Gear. If a monster\
lacks those details, entries for them don’t appear.\
Traits. Monster characteristics that are active at all\
times or during certain situations appear here.\
Actions. The monster can take the actions here in\
addition to those in “Playing the Game.”\
Bonus Actions. This section provides a monster’s\
Bonus Actions, if any.\
Reactions and Legendary Actions. These sections\
provide Reactions and Legendary Actions, if any.\
