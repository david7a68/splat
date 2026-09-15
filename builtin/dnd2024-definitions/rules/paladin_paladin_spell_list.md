---
splat_schema: 1
splat_id: a3fca2a6-e22b-5e2d-8dd5-84182cd47df2
splat_kind: rule
splat_key: dnd2024:rules/paladin_paladin_spell_list
splat_name: Paladin Spell List
splat_origin:
  key: dnd2024:reference/pages/055
splat_sources:
- title: SRD 5.2.1
  locator: pp. 55, 56
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/paladin_paladin_spell_list
  parameters:
    class:
      key: dnd2024:classes/paladin
    spell_list:
      '1':
      - key: dnd2024:spells/bless
      - key: dnd2024:spells/command
      - key: dnd2024:spells/cure_wounds
      - key: dnd2024:spells/detect_evil_and_good
      - key: dnd2024:spells/detect_magic
      - key: dnd2024:spells/detect_poison_and_disease
      - key: dnd2024:spells/divine_favor
      - key: dnd2024:spells/divine_smite
      - key: dnd2024:spells/heroism
      - key: dnd2024:spells/protection_from_evil_and_good
      - key: dnd2024:spells/purify_food_and_drink
      - key: dnd2024:spells/searing_smite
      - key: dnd2024:spells/shield_of_faith
      '2':
      - key: dnd2024:spells/aid
      - key: dnd2024:spells/find_steed
      - key: dnd2024:spells/gentle_repose
      - key: dnd2024:spells/lesser_restoration
      - key: dnd2024:spells/locate_object
      - key: dnd2024:spells/magic_weapon
      - key: dnd2024:spells/prayer_of_healing
      - key: dnd2024:spells/protection_from_poison
      - key: dnd2024:spells/shining_smite
      - key: dnd2024:spells/warding_bond
      - key: dnd2024:spells/zone_of_truth
      '3':
      - key: dnd2024:spells/create_food_and_water
      - key: dnd2024:spells/daylight
      - key: dnd2024:spells/dispel_magic
      - key: dnd2024:spells/magic_circle
      - key: dnd2024:spells/remove_curse
      - key: dnd2024:spells/revivify
      '4':
      - key: dnd2024:spells/aura_of_life
      - key: dnd2024:spells/banishment
      - key: dnd2024:spells/death_ward
      - key: dnd2024:spells/locate_creature
      '5':
      - key: dnd2024:spells/dispel_evil_and_good
      - key: dnd2024:spells/geas
      - key: dnd2024:spells/greater_restoration
      - key: dnd2024:spells/raise_dead
    school_concentration_ritual_and_material_metadata: referenced_spell_definition
  implementation: dnd2024:rule_paladin_paladin_spell_list
splat_contracts:
- id: dnd2024:rule_paladin_paladin_spell_list
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - spell_resolution
  events: []
  capabilities:
  - spellcasting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 55
  - 56
  engine_gaps: &id001
  - dnd2024:rule_paladin_paladin_spell_list
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: ba9177e5a840240907c662ac2e3bfde6eb64963c4e4d7929b5030b9b50205f14
  parent_name: Paladin
  line_span:
    first: 4662
    count: 56
splat_clauses:
- key: source_rules
  text: '# Paladin Spell List This section presents the Paladin spell list. The spells are organized by
    spell level and then alphabetized, and each spell’s school of magic is listed. In the Special column,
    C means the spell requires Concentration, R means it’s a Ritual, and M means it requires a specific
    Material component. Level 1 Paladin Spells Spell School Special Bless Enchantment C, M Command Enchantment
    — Cure Wounds Abjuration — Detect Evil and Good Divination C Detect Magic Divination C, R Detect Poison
    and Disease Divination C, R Spell School Special Divine Favor Transmutation — Divine Smite Evocation
    — Heroism Enchantment C Protection from Evil and Good Abjuration C, M Purify Food and Drink Transmutation
    R Searing Smite Evocation — Shield of Faith Abjuration C Level 2 Paladin Spells Spell School Special
    Aid Abjuration — Find Steed Conjuration — Gentle Repose Necromancy R, M Lesser Restoration Abjuration
    — Locate Object Divination C Magic Weapon Transmutation — Prayer of Healing Abjuration — Protection
    from Poison Abjuration — Shining Smite Transmutation C Warding Bond Abjuration M Zone of Truth Enchantment
    — Level 3 Paladin Spells Spell School Special Create Food and Water Conjuration — Daylight Evocation
    — Dispel Magic Abjuration — Magic Circle Abjuration M Remove Curse Abjuration — Revivify Necromancy
    M Level 4 Paladin Spells Spell School Special Aura of Life Abjuration C Banishment Abjuration C Death
    Ward Abjuration — Locate Creature Divination C Level 5 Paladin Spells Spell School Special Dispel
    Evil and Good Abjuration C Geas Enchantment — Greater Restoration Abjuration M Raise Dead Necromancy
    M'
  sha256: b4c284a59a57304c015a4204910c467c247c2125fbff2c161103f49fb358633a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Paladin Spell List

<!-- SRD page 55 -->

This section presents the Paladin spell list. The\
spells are organized by spell level and then alpha-\
betized, and each spell’s school of magic is listed.\
In the Special column, C means the spell requires\
Concentration, R means it’s a Ritual, and M means it\
requires a specific Material component.\
Level 1 Paladin Spells\
Spell School Special\
Bless Enchantment C, M\
Command Enchantment —\
Cure Wounds Abjuration —\
Detect Evil and Good Divination C\
Detect Magic Divination C, R\
Detect Poison and Disease Divination C, R\

<!-- SRD page 56 -->

Spell School Special\
Divine Favor Transmutation —\
Divine Smite Evocation —\
Heroism Enchantment C\
Protection from Evil and Good Abjuration C, M\
Purify Food and Drink Transmutation R\
Searing Smite Evocation —\
Shield of Faith Abjuration C\
Level 2 Paladin Spells\
Spell School Special\
Aid Abjuration —\
Find Steed Conjuration —\
Gentle Repose Necromancy R, M\
Lesser Restoration Abjuration —\
Locate Object Divination C\
Magic Weapon Transmutation —\
Prayer of Healing Abjuration —\
Protection from Poison Abjuration —\
Shining Smite Transmutation C\
Warding Bond Abjuration M\
Zone of Truth Enchantment —\
Level 3 Paladin Spells\
Spell School Special\
Create Food and Water Conjuration —\
Daylight Evocation —\
Dispel Magic Abjuration —\
Magic Circle Abjuration M\
Remove Curse Abjuration —\
Revivify Necromancy M\
Level 4 Paladin Spells\
Spell School Special\
Aura of Life Abjuration C\
Banishment Abjuration C\
Death Ward Abjuration —\
Locate Creature Divination C\
Level 5 Paladin Spells\
Spell School Special\
Dispel Evil and Good Abjuration C\
Geas Enchantment —\
Greater Restoration Abjuration M\
Raise Dead Necromancy M\
