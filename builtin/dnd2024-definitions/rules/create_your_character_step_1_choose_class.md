---
splat_schema: 1
splat_id: 480440e0-e1f4-55de-8648-7cc15ece079c
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_step_1_choose_class
splat_name: 'Step 1: Choose Class'
splat_origin:
  key: dnd2024:reference/pages/019
splat_sources:
- title: SRD 5.2.1
  locator: pp. 19
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/create_your_character_step_1_choose_class
  parameters:
    choose:
      count: 1
      options:
      - key: dnd2024:classes/barbarian
      - key: dnd2024:classes/bard
      - key: dnd2024:classes/cleric
      - key: dnd2024:classes/druid
      - key: dnd2024:classes/fighter
      - key: dnd2024:classes/monk
      - key: dnd2024:classes/paladin
      - key: dnd2024:classes/ranger
      - key: dnd2024:classes/rogue
      - key: dnd2024:classes/sorcerer
      - key: dnd2024:classes/warlock
      - key: dnd2024:classes/wizard
    class_primary_abilities:
      barbarian:
      - strength
      bard:
      - charisma
      cleric:
      - wisdom
      druid:
      - wisdom
      fighter:
        any:
        - strength
        - dexterity
      monk:
      - dexterity
      - wisdom
      paladin:
      - strength
      - charisma
      ranger:
      - dexterity
      - wisdom
      rogue:
      - dexterity
      sorcerer:
      - charisma
      warlock:
      - charisma
      wizard:
      - intelligence
  implementation: dnd2024:rule_create_your_character_step_1_choose_class
splat_contracts:
- id: dnd2024:rule_create_your_character_step_1_choose_class
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - character_changes
  events: []
  capabilities:
  - character_creation
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 19
  engine_gaps: &id001
  - dnd2024:rule_create_your_character_step_1_choose_class
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 6432ac2e8cc305fc193b7074e77bef82d275f1d9b7b49f60de1c2b08b4ab3b47
  parent_name: Create Your Character
  line_span:
    first: 1346
    count: 25
splat_clauses:
- key: source_rules
  text: '# Step 1: Choose Class Choose a class, and write it on your character sheet. The Class Overview
    table summarizes the classes. See “Classes” for the classes’ details. Class Overview Class Likes ...
    Primary Complexity Ability Barbarian Battle Strength Average Bard Performing Charisma High Cleric
    Gods Wisdom Average Druid Nature Wisdom High Fighter Weapons Strength Low or Dexterity Monk Unarmed
    Dexterity High combat and Wisdom Paladin Defense Strength Average and Charisma Ranger Survival Dexterity
    Average and Wisdom Class Likes ... Primary Complexity Ability Rogue Stealth Dexterity Low Sorcerer
    Power Charisma High Warlock Occult lore Charisma High Wizard Spellbooks Intelligence Average'
  sha256: e8f25beacca7800470c8e6c9fa5d62ca7394411758192f075e9525fe300d50b5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Step 1: Choose Class

<!-- SRD page 19 -->

Choose a class, and write it on your character sheet.\
The Class Overview table summarizes the classes.\
See “Classes” for the classes’ details.\
Class Overview\
Class Likes ... Primary Complexity\
Ability\
Barbarian Battle Strength Average\
Bard Performing Charisma High\
Cleric Gods Wisdom Average\
Druid Nature Wisdom High\
Fighter Weapons Strength Low\
or Dexterity\
Monk Unarmed Dexterity High\
combat and Wisdom\
Paladin Defense Strength Average\
and Charisma\
Ranger Survival Dexterity Average\
and Wisdom\
Class Likes ... Primary Complexity\
Ability\
Rogue Stealth Dexterity Low\
Sorcerer Power Charisma High\
Warlock Occult lore Charisma High\
Wizard Spellbooks Intelligence Average\
