---
splat_schema: 1
splat_id: b218697c-baf0-53fe-bea1-725364371d82
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_choose_a_background
splat_name: Choose a Background
splat_origin:
  key: dnd2024:reference/pages/019
splat_sources:
- title: SRD 5.2.1
  locator: pp. 19, 20
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/create_your_character_choose_a_background
  parameters:
    background_choice:
      count: 1
      options:
      - key: dnd2024:backgrounds/acolyte
      - key: dnd2024:backgrounds/soldier
      additional_options_authority: gm
    grants:
      feat: background_feat
      skill_proficiencies: 2
      tool_proficiencies: 1
    class_proficiencies_also_granted: true
    level_1_proficiency_bonus: 2
    ability_background_examples:
      strength: soldier
      dexterity: soldier
      constitution: soldier
      intelligence: acolyte
      wisdom: acolyte
      charisma: acolyte
  implementation: dnd2024:rules_create_your_character_choose_a_background
splat_contracts:
- id: dnd2024:rules_create_your_character_choose_a_background
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
  - 19
  - 20
  engine_gaps: &id001
  - dnd2024:rules_create_your_character_choose_a_background
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 61285099f5125f2eb3c9f1cf79af1584f68d87265cde7fdf5789b5d4003341ed
  parent_name: Create Your Character
  line_span:
    first: 1398
    count: 35
splat_clauses:
- key: source_rules
  text: '# Choose a Background Choose your character’s background, and write it on your character sheet.
    You can choose any of the backgrounds detailed in “Character Origins,” and your GM might offer additional
    backgrounds as options. The background you choose influences step 3, when you determine your character’s
    ability scores. If you’re having trouble choosing, the Ability Scores and Backgrounds table shows
    which backgrounds benefit which ability scores. Look for your class’s primary ability there. Ability
    Scores and Backgrounds Ability Background Strength Soldier Dexterity Soldier Constitution Soldier
    Intelligence Acolyte Ability Background Wisdom Acolyte Charisma Acolyte Record Your Feat. A background
    gives you a feat, which grants your character particular capabilities. Feats are detailed in “Feats.”
    Write the feat on your character sheet. Note Proficiencies. Your background gives proficiency in two
    skills and with one tool. Record this information on your character sheet. Your class also gives proficiencies.
    Check your class description in “Classes” and note the proficiencies on your character sheet. The
    features table in your class description shows your Proficiency Bonus (described in “Playing the Game”),
    which is +2 for a level 1 character. Note this number on your character sheet. You’ll fill in other
    numbers connected to these proficiencies in step 5.'
  sha256: e1c4cd3e4a2a1fa038db6ace163c93fe27e9251e1e9314d87fea05a0567ab746
  status: contract_specified
splat_requires_mechanics: *id001
---

# Choose a Background

<!-- SRD page 19 -->

Choose your character’s background, and write it on\
your character sheet. You can choose any of the back-\
grounds detailed in “Character Origins,” and your GM\
might offer additional backgrounds as options.\
The background you choose influences step 3,\
when you determine your character’s ability scores.\
If you’re having trouble choosing, the Ability Scores\
and Backgrounds table shows which backgrounds\
benefit which ability scores. Look for your class’s\
primary ability there.\
Ability Scores and Backgrounds\
Ability Background\
Strength Soldier\
Dexterity Soldier\
Constitution Soldier\
Intelligence Acolyte\

<!-- SRD page 20 -->

Ability Background\
Wisdom Acolyte\
Charisma Acolyte\
Record Your Feat. A background gives you a feat,\
which grants your character particular capabilities.\
Feats are detailed in “Feats.” Write the feat on your\
character sheet.\
Note Proficiencies. Your background gives profi-\
ciency in two skills and with one tool. Record this\
information on your character sheet.\
Your class also gives proficiencies. Check your\
class description in “Classes” and note the proficien-\
cies on your character sheet.\
The features table in your class description shows\
your Proficiency Bonus (described in “Playing the\
Game”), which is +2 for a level 1 character. Note this\
number on your character sheet. You’ll fill in other\
numbers connected to these proficiencies in step 5.\
