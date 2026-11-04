---
splat_schema: 2
splat_id: 92cad99f-65ae-5a25-a60a-03b36b768489
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_fill_in_numbers
splat_name: Fill In Numbers
splat_origin: dnd2024:reference/pages/022
splat_sources:
- SRD 5.2.1, pp. 22, 23, https://www.dndbeyond.com/srd
---

# Fill In Numbers

## Source text

<!-- SRD page 22 -->

Note these numbers on your character sheet.

Saving Throws. For the saving throws you have proficiency in, add your Proficiency Bonus to the appropriate ability modifier and note the total. Some players also like to note the modifier for saving throws they’re not proficient in, which is just the relevant ability modifier.

Skills. For skills you have proficiency in, add your Proficiency Bonus to the ability modifier associated with that skill, and note the total. You might also wish to note the modifier for skills you’re not proficient in, which is just the relevant ability modifier.

Passive Perception. Sometimes your GM will determine whether your character notices something without asking you to make a Wisdom (Perception) check; the GM uses your Passive Perception instead. Passive Perception is a score that reflects a general awareness of your surroundings when you’re not actively looking for something. Use this formula to determine your Passive Perception score:

Passive Perception = 10 + Wisdom (Perception) check modifier

Include all modifiers that apply to your Wisdom (Perception) checks. For example, if your character has a Wisdom of 15 and proficiency in the Perception skill, you have a Passive Perception of 14 (10 + 2 for your Wisdom modifier + 2 for proficiency).

Hit Points. Your class and Constitution modifier determine your Hit Point maximum at level 1, as shown on the Level 1 Hit Points by Class table.

**Level 1 Hit Points by Class**

| Class | Hit Point Maximum |
| --- | --- |
| Barbarian | 12 + Con. modifier |
| Fighter, Paladin, or Ranger | 10 + Con. modifier |
| Bard, Cleric, Druid, Monk, Rogue, or Warlock | 8 + Con. modifier |
| Sorcerer or Wizard | 6 + Con. modifier |

The character sheet includes room to note your current Hit Points when you take damage, as well as any Temporary Hit Points you might gain. There’s also space to track Death Saving Throws.

Hit Point Dice. Your class’s description tells you the die type of your character’s Hit Point Dice (or Hit Dice for short); write this on your character sheet. At level 1, your character has 1 Hit Die. You can spend Hit Dice during a Short Rest to recover Hit Points. Your character sheet also includes space to note how many Hit Dice you’ve spent.

Initiative. Write your Dexterity modifier in the space for Initiative on your character sheet.

Armor Class. Without armor or a shield, your base Armor Class is 10 plus your Dexterity modifier. If your starting equipment includes armor or a Shield (or both), calculate your AC using the rules in “Equipment.” A class feature might give you a different way to calculate your AC.

Attacks. In the Weapons & Damage Cantrips section of the character sheet, write your starting weapons. The attack roll bonus for a weapon with which you have proficiency is one of the following unless a weapon’s property says otherwise:

Melee attack bonus = Strength modifier + Proficiency Bonus

Ranged attack bonus = Dexterity modifier + Proficiency Bonus

Look up the damage and properties of your weapons in “Equipment.” You add the same ability modifier you use for attacks with a weapon to your damage rolls with that weapon.

Spellcasting. Note both the saving throw DC for your spells and the attack bonus for attacks you make with them, using these formulas:

<!-- SRD page 23 -->

Spell save DC = 8 + spellcasting ability modifier + Proficiency Bonus

Spell attack bonus = spellcasting ability modifier + Proficiency Bonus

Your spellcasting ability modifier for a spell is determined by whatever feature gives you the ability to cast the spell.

Spell Slots, Cantrips, and Prepared Spells. If your class gives you the Spellcasting or Pact Magic feature, your class features table shows the number of spell slots you have available, how many cantrips you know, and how many spells you can prepare. Choose your cantrips and prepared spells, and note them—along with your number of spell slots—on your character sheet.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/create_your_character_fill_in_numbers ` |
| ` implementation ` | ` dnd2024:rule_create_your_character_fill_in_numbers ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` level_one_hit_dice ` | ` 1 ` |
| ` hit_die_type_source ` | ` class ` |
| ` initiative ` | ` dexterity_modifier ` |
| ` armor_and_feature_ac_override ` | ` true ` |
| ` weapon_damage_add ` | ` attack_ability_modifier ` |
| ` spellcasting_ability_source ` | ` granting_feature ` |
| ` spell_slots_cantrips_prepared_limits ` | ` class_feature_table ` |

#### Saving throw and skill modifier

<!-- splat:values path=splat_rule/parameters/saving_throw_and_skill_modifier -->

| Field | Value |
| --- | --- |
| ` op ` | ` add ` |

##### Args

<!-- splat:table
path: splat_rule/parameters/saving_throw_and_skill_modifier/args
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Variable:
    path: variable
    type: symbol
    optional: true
  If:
    path: if
    type: symbol
    optional: true
  Then variable:
    path: then/variable
    type: symbol
    optional: true
  Else:
    path: else
    type: integer
    optional: true
-->

| Key | Variable | If | Then variable | Else |
| --- | --- | --- | --- | --- |
| ` row_1 ` | ` Ability modifier ` |  |  |  |
| ` row_2 ` |  | ` Proficient ` | ` Proficiency bonus ` | ` 0 ` |

#### Passive perception

<!-- splat:yaml path=splat_rule/parameters/passive_perception -->

```yaml
op: add
args:
- 10
- variable: total_wisdom_perception_modifier
```

#### Level one hp

<!-- splat:yaml path=splat_rule/parameters/level_one_hp -->

```yaml
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
```

#### Unarmored unshielded ac

<!-- splat:yaml path=splat_rule/parameters/unarmored_unshielded_ac -->

```yaml
op: add
args:
- 10
- variable: dexterity_modifier
```

#### Proficient weapon attack

<!-- splat:values path=splat_rule/parameters/proficient_weapon_attack -->

| Field | Value |
| --- | --- |
| ` property_override ` | ` true ` |

##### Melee

<!-- splat:values path=splat_rule/parameters/proficient_weapon_attack/melee -->

| Field | Value |
| --- | --- |
| ` op ` | ` add ` |

###### Args

<!-- splat:table
path: splat_rule/parameters/proficient_weapon_attack/melee/args
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Variable:
    path: variable
    type: symbol
-->

| Key | Variable |
| --- | --- |
| ` row_1 ` | ` Strength modifier ` |
| ` row_2 ` | ` Proficiency bonus ` |

##### Ranged

<!-- splat:values path=splat_rule/parameters/proficient_weapon_attack/ranged -->

| Field | Value |
| --- | --- |
| ` op ` | ` add ` |

###### Args

<!-- splat:table
path: splat_rule/parameters/proficient_weapon_attack/ranged/args
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Variable:
    path: variable
    type: symbol
-->

| Key | Variable |
| --- | --- |
| ` row_1 ` | ` Dexterity modifier ` |
| ` row_2 ` | ` Proficiency bonus ` |

#### Spell save dc

<!-- splat:yaml path=splat_rule/parameters/spell_save_dc -->

```yaml
op: add
args:
- 8
- variable: spellcasting_ability_modifier
- variable: proficiency_bonus
```

#### Spell attack

<!-- splat:values path=splat_rule/parameters/spell_attack -->

| Field | Value |
| --- | --- |
| ` op ` | ` add ` |

##### Args

<!-- splat:table
path: splat_rule/parameters/spell_attack/args
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Variable:
    path: variable
    type: symbol
-->

| Key | Variable |
| --- | --- |
| ` row_1 ` | ` Spellcasting ability modifier ` |
| ` row_2 ` | ` Proficiency bonus ` |
