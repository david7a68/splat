---
splat_schema: 1
splat_id: 92cad99f-65ae-5a25-a60a-03b36b768489
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_fill_in_numbers
splat_name: Fill In Numbers
splat_origin:
  key: dnd2024:reference/pages/022
splat_sources:
- title: SRD 5.2.1
  locator: pp. 22, 23
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/create_your_character_fill_in_numbers
  parameters:
    saving_throw_and_skill_modifier:
      op: add
      args:
      - variable: ability_modifier
      - if: proficient
        then:
          variable: proficiency_bonus
        else: 0
    passive_perception:
      op: add
      args:
      - 10
      - variable: total_wisdom_perception_modifier
    level_one_hp:
      base_by_class:
        barbarian: 12
        fighter: 10
        paladin: 10
        ranger: 10
        bard: 8
        cleric: 8
        druid: 8
        monk: 8
        rogue: 8
        warlock: 8
        sorcerer: 6
        wizard: 6
      add: constitution_modifier
    level_one_hit_dice: 1
    hit_die_type_source: class
    initiative: dexterity_modifier
    unarmored_unshielded_ac:
      op: add
      args:
      - 10
      - variable: dexterity_modifier
    armor_and_feature_ac_override: true
    proficient_weapon_attack:
      melee:
        op: add
        args:
        - variable: strength_modifier
        - variable: proficiency_bonus
      ranged:
        op: add
        args:
        - variable: dexterity_modifier
        - variable: proficiency_bonus
      property_override: true
    weapon_damage_add: attack_ability_modifier
    spell_save_dc:
      op: add
      args:
      - 8
      - variable: spellcasting_ability_modifier
      - variable: proficiency_bonus
    spell_attack:
      op: add
      args:
      - variable: spellcasting_ability_modifier
      - variable: proficiency_bonus
    spellcasting_ability_source: granting_feature
    spell_slots_cantrips_prepared_limits: class_feature_table
  implementation: dnd2024:rule_create_your_character_fill_in_numbers
splat_contracts:
- id: dnd2024:rule_create_your_character_fill_in_numbers
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    ability_modifier:
      type: number
      required: true
    dexterity_modifier:
      type: number
      required: true
    proficiency_bonus:
      type: number
      required: true
    spellcasting_ability_modifier:
      type: number
      required: true
    strength_modifier:
      type: number
      required: true
    total_wisdom_perception_modifier:
      type: number
      required: true
  outputs:
  - character_changes
  events: []
  capabilities:
  - character_creation
  - statistics
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 22
  - 23
  engine_gaps: &id001
  - dnd2024:rule_create_your_character_fill_in_numbers
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 2cf13b25560f7626c083f52313b86bbf357c59405baa9b434ffa405068375e43
  parent_name: Create Your Character
  line_span:
    first: 1645
    count: 88
splat_clauses:
- key: source_rules
  text: '# Fill In Numbers Note these numbers on your character sheet. Saving Throws. For the saving throws
    you have proficiency in, add your Proficiency Bonus to the appropriate ability modifier and note the
    total. Some players also like to note the modifier for saving throws they’re not proficient in, which
    is just the relevant ability modifier. Skills. For skills you have proficiency in, add your Proficiency
    Bonus to the ability modifier associated with that skill, and note the total. You might also wish
    to note the modifier for skills you’re not proficient in, which is just the relevant ability modifier.
    Passive Perception. Sometimes your GM will determine whether your character notices something without
    asking you to make a Wisdom (Perception) check; the GM uses your Passive Perception instead. Passive
    Perception is a score that reflects a general awareness of your surroundings when you’re not actively
    looking for something. Use this formula to determine your Passive Perception score: Passive Perception
    = 10 + Wisdom (Perception) check modifier Include all modifiers that apply to your Wisdom (Perception)
    checks. For example, if your character has a Wisdom of 15 and proficiency in the Perception skill,
    you have a Passive Perception of 14 (10 + 2 for your Wisdom modifier + 2 for proficiency). Hit Points.
    Your class and Constitution modifier determine your Hit Point maximum at level 1, as shown on the
    Level 1 Hit Points by Class table. Level 1 Hit Points by Class Class Hit Point Maximum Barbarian 12
    + Con. modifier Fighter, Paladin, or Ranger 10 + Con. modifier Bard, Cleric, Druid, Monk, 8 + Con.
    modifier Rogue, or Warlock Sorcerer or Wizard 6 + Con. modifier The character sheet includes room
    to note your current Hit Points when you take damage, as well as any Temporary Hit Points you might
    gain. There’s also space to track Death Saving Throws. Hit Point Dice. Your class’s description tells
    you the die type of your character’s Hit Point Dice (or Hit Dice for short); write this on your character
    sheet. At level 1, your character has 1 Hit Die. You can spend Hit Dice during a Short Rest to recover
    Hit Points. Your character sheet also includes space to note how many Hit Dice you’ve spent. Initiative.
    Write your Dexterity modifier in the space for Initiative on your character sheet. Armor Class. Without
    armor or a shield, your base Armor Class is 10 plus your Dexterity modifier. If your starting equipment
    includes armor or a Shield (or both), calculate your AC using the rules in “Equipment.” A class feature
    might give you a different way to calculate your AC. Attacks. In the Weapons & Damage Cantrips section
    of the character sheet, write your starting weapons. The attack roll bonus for a weapon with which
    you have proficiency is one of the following unless a weapon’s property says otherwise: Melee attack
    bonus = Strength modifier + Proficiency Bonus Ranged attack bonus = Dexterity modifier + Proficiency
    Bonus Look up the damage and properties of your weapons in “Equipment.” You add the same ability modifier
    you use for attacks with a weapon to your damage rolls with that weapon. Spellcasting. Note both the
    saving throw DC for your spells and the attack bonus for attacks you make with them, using these formulas:
    Spell save DC = 8 + spellcasting ability modifier + Proficiency Bonus Spell attack bonus = spellcasting
    ability modifier + Proficiency Bonus Your spellcasting ability modifier for a spell is determined
    by whatever feature gives you the ability to cast the spell. Spell Slots, Cantrips, and Prepared Spells.
    If your class gives you the Spellcasting or Pact Magic feature, your class features table shows the
    number of spell slots you have available, how many cantrips you know, and how many spells you can
    prepare. Choose your cantrips and prepared spells, and note them—along with your number of spell slots—on
    your character sheet.'
  sha256: 2fa6443994167424ee2c5ffc20a4d0fbd081b9a96624fb3e174c86a355ff01cb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Fill In Numbers

<!-- SRD page 22 -->

Note these numbers on your character sheet.\
Saving Throws. For the saving throws you have\
proficiency in, add your Proficiency Bonus to the ap-\
propriate ability modifier and note the total. Some\
players also like to note the modifier for saving\
throws they’re not proficient in, which is just the\
relevant ability modifier.\
Skills. For skills you have proficiency in, add your\
Proficiency Bonus to the ability modifier associated\
with that skill, and note the total. You might also\
wish to note the modifier for skills you’re not profi-\
cient in, which is just the relevant ability modifier.\
Passive Perception. Sometimes your GM will de-\
termine whether your character notices something\
without asking you to make a Wisdom (Perception)\
check; the GM uses your Passive Perception instead.\
Passive Perception is a score that reflects a general\
awareness of your surroundings when you’re not\
actively looking for something. Use this formula to\
determine your Passive Perception score:\
Passive Perception = 10 + Wisdom (Perception)\
check modifier\
Include all modifiers that apply to your Wisdom\
(Perception) checks. For example, if your character\
has a Wisdom of 15 and proficiency in the Percep-\
tion skill, you have a Passive Perception of 14 (10 +\
2 for your Wisdom modifier + 2 for proficiency).\
Hit Points. Your class and Constitution modifier\
determine your Hit Point maximum at level 1, as\
shown on the Level 1 Hit Points by Class table.\
Level 1 Hit Points by Class\
Class Hit Point Maximum\
Barbarian 12 + Con. modifier\
Fighter, Paladin, or Ranger 10 + Con. modifier\
Bard, Cleric, Druid, Monk, 8 + Con. modifier\
Rogue, or Warlock\
Sorcerer or Wizard 6 + Con. modifier\
The character sheet includes room to note your\
current Hit Points when you take damage, as well as\
any Temporary Hit Points you might gain. There’s\
also space to track Death Saving Throws.\
Hit Point Dice. Your class’s description tells you\
the die type of your character’s Hit Point Dice (or\
Hit Dice for short); write this on your character\
sheet. At level 1, your character has 1 Hit Die. You\
can spend Hit Dice during a Short Rest to recover\
Hit Points. Your character sheet also includes space\
to note how many Hit Dice you’ve spent.\
Initiative. Write your Dexterity modifier in the\
space for Initiative on your character sheet.\
Armor Class. Without armor or a shield, your\
base Armor Class is 10 plus your Dexterity modi-\
fier. If your starting equipment includes armor or a\
Shield (or both), calculate your AC using the rules in\
“Equipment.” A class feature might give you a differ-\
ent way to calculate your AC.\
Attacks. In the Weapons & Damage Cantrips\
section of the character sheet, write your starting\
weapons. The attack roll bonus for a weapon with\
which you have proficiency is one of the following\
unless a weapon’s property says otherwise:\
Melee attack bonus = Strength modifier\
+ Proficiency Bonus\
Ranged attack bonus = Dexterity modifier\
+ Proficiency Bonus\
Look up the damage and properties of your\
weapons in “Equipment.” You add the same ability\
modifier you use for attacks with a weapon to your\
damage rolls with that weapon.\
Spellcasting. Note both the saving throw DC for\
your spells and the attack bonus for attacks you\
make with them, using these formulas:\

<!-- SRD page 23 -->

Spell save DC = 8 + spellcasting ability modifier\
+ Proficiency Bonus\
Spell attack bonus = spellcasting ability modifier\
+ Proficiency Bonus\
Your spellcasting ability modifier for a spell is de-\
termined by whatever feature gives you the ability\
to cast the spell.\
Spell Slots, Cantrips, and Prepared Spells. If\
your class gives you the Spellcasting or Pact Magic\
feature, your class features table shows the number\
of spell slots you have available, how many cantrips\
you know, and how many spells you can prepare.\
Choose your cantrips and prepared spells, and note\
them—along with your number of spell slots—on\
your character sheet.\
