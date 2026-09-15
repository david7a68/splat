---
splat_schema: 1
splat_id: ede384ce-63d2-5fb8-8cfd-310a50df8095
splat_kind: rule
splat_key: dnd2024:rules/cleric_cleric_spell_list
splat_name: Cleric Spell List
splat_origin:
  key: dnd2024:reference/pages/038
splat_sources:
- title: SRD 5.2.1
  locator: pp. 38, 39, 40
  url: https://www.dndbeyond.com/srd
---

# Cleric Spell List

## Source text

<!-- SRD page 38 -->

This section presents the Cleric spell list. The spells\
are organized by spell level and then alphabetized,\
and each spell’s school of magic is listed. In the Spe-\
cial column, C means the spell requires Concentra-\
tion, R means it’s a Ritual, and M means it requires a\
specific Material component.\
Cantrips (Level 0 Cleric Spells)\
Spell School Special\
Guidance Divination C\
Light Evocation —\
Mending Transmutation —\
Resistance Abjuration C\
Sacred Flame Evocation —\
Spare the Dying Necromancy —\
Thaumaturgy Transmutation —\
Level 1 Cleric Spells\
Spell School Special\
Bane Enchantment C\
Bless Enchantment C, M\
Command Enchantment —\
Create or Destroy Water Transmutation —\
Cure Wounds Abjuration —\
Detect Evil and Good Divination C\
Detect Magic Divination C, R\
Detect Poison and Disease Divination C, R\
Guiding Bolt Evocation —\
Healing Word Abjuration —\
Inflict Wounds Necromancy —\
Protection from Evil and Good Abjuration C, M\
Purify Food and Drink Transmutation R\
Sanctuary Abjuration —\
Shield of Faith Abjuration C\
Level 2 Cleric Spells\
Spell School Special\
Aid Abjuration —\
Augury Divination R, M\
Blindness/Deafness Transmutation —\
Calm Emotions Enchantment C\
Continual Flame Evocation M\
Enhance Ability Transmutation C\
Find Traps Divination —\
Gentle Repose Necromancy R, M\
Hold Person Enchantment C\
Lesser Restoration Abjuration —\
Locate Object Divination C\
Prayer of Healing Abjuration —\

<!-- SRD page 39 -->

Spell School Special\
Protection from Poison Abjuration —\
Silence Illusion C, R\
Spiritual Weapon Evocation C\
Warding Bond Abjuration M\
Zone of Truth Enchantment —\
Level 3 Cleric Spells\
Spell School Special\
Animate Dead Necromancy —\
Beacon of Hope Abjuration C\
Bestow Curse Necromancy C\
Clairvoyance Divination C, M\
Create Food and Water Conjuration —\
Daylight Evocation —\
Dispel Magic Abjuration —\
Glyph of Warding Abjuration M\
Magic Circle Abjuration M\
Mass Healing Word Abjuration —\
Meld into Stone Transmutation R\
Protection from Energy Abjuration C\
Remove Curse Abjuration —\
Revivify Necromancy M\
Sending Divination —\
Speak with Dead Necromancy —\
Spirit Guardians Conjuration C\
Tongues Divination —\
Water Walk Transmutation R\
Level 4 Cleric Spells\
Spell School Special\
Aura of Life Abjuration C\
Banishment Abjuration C\
Control Water Transmutation C\
Death Ward Abjuration —\
Divination Divination R, M\
Freedom of Movement Abjuration —\
Guardian of Faith Conjuration —\
Locate Creature Divination C\
Stone Shape Transmutation —\
Level 5 Cleric Spells\
Spell School Special\
Commune Divination R\
Contagion Necromancy —\
Dispel Evil and Good Abjuration C\
Flame Strike Evocation —\
Geas Enchantment —\
Greater Restoration Abjuration M\
Hallow Abjuration M\
Insect Plague Conjuration C\
Legend Lore Divination M\
Mass Cure Wounds Abjuration —\
Planar Binding Abjuration M\
Raise Dead Necromancy M\
Scrying Divination C, M\
Level 6 Cleric Spells\
Spell School Special\
Blade Barrier Evocation C\
Create Undead Necromancy M\
Find the Path Divination C, M\
Forbiddance Abjuration R, M\
Harm Necromancy —\
Heal Abjuration —\
Heroes’ Feast Conjuration M\
Planar Ally Conjuration —\
Sunbeam Evocation C\
True Seeing Divination M\
Word of Recall Conjuration —\
Level 7 Cleric Spells\
Spell School Special\
Conjure Celestial Conjuration C\
Divine Word Evocation —\
Etherealness Conjuration —\
Fire Storm Evocation —\
Plane Shift Conjuration M\
Regenerate Transmutation —\
Resurrection Necromancy M\
Symbol Abjuration M\

<!-- SRD page 40 -->

Level 8 Cleric Spells\
Spell School Special\
Antimagic Field Abjuration C\
Control Weather Transmutation C\
Earthquake Transmutation C\
Holy Aura Abjuration C, M\
Sunburst Evocation —\
Level 9 Cleric Spells\
Spell School Special\
Astral Projection Necromancy M\
Gate Conjuration C, M\
Mass Heal Abjuration —\
Power Word Heal Enchantment —\
True Resurrection Necromancy M\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/cleric_cleric_spell_list ` |
| ` implementation ` | ` dnd2024:rules_cleric_cleric_spell_list ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` sorting ` | ` ["spell_level", "name"] ` |

#### Class

<!-- splat:values path=splat_rule/parameters/class -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/cleric ` |

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | level | school | concentration | ritual | specific_material |
| --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 0 ` | ` divination ` | ` true ` | ` false ` | ` false ` |
| ` 1 ` | ` 0 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 2 ` | ` 0 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 3 ` | ` 0 ` | ` abjuration ` | ` true ` | ` false ` | ` false ` |
| ` 4 ` | ` 0 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 5 ` | ` 0 ` | ` necromancy ` | ` false ` | ` false ` | ` false ` |
| ` 6 ` | ` 0 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 7 ` | ` 1 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 8 ` | ` 1 ` | ` enchantment ` | ` true ` | ` false ` | ` true ` |
| ` 9 ` | ` 1 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 10 ` | ` 1 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 11 ` | ` 1 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 12 ` | ` 1 ` | ` divination ` | ` true ` | ` false ` | ` false ` |
| ` 13 ` | ` 1 ` | ` divination ` | ` true ` | ` true ` | ` false ` |
| ` 14 ` | ` 1 ` | ` divination ` | ` true ` | ` true ` | ` false ` |
| ` 15 ` | ` 1 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 16 ` | ` 1 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 17 ` | ` 1 ` | ` necromancy ` | ` false ` | ` false ` | ` false ` |
| ` 18 ` | ` 1 ` | ` abjuration ` | ` true ` | ` false ` | ` true ` |
| ` 19 ` | ` 1 ` | ` transmutation ` | ` false ` | ` true ` | ` false ` |
| ` 20 ` | ` 1 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 21 ` | ` 1 ` | ` abjuration ` | ` true ` | ` false ` | ` false ` |
| ` 22 ` | ` 2 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 23 ` | ` 2 ` | ` divination ` | ` false ` | ` true ` | ` true ` |
| ` 24 ` | ` 2 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 25 ` | ` 2 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 26 ` | ` 2 ` | ` evocation ` | ` false ` | ` false ` | ` true ` |
| ` 27 ` | ` 2 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 28 ` | ` 2 ` | ` divination ` | ` false ` | ` false ` | ` false ` |
| ` 29 ` | ` 2 ` | ` necromancy ` | ` false ` | ` true ` | ` true ` |
| ` 30 ` | ` 2 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 31 ` | ` 2 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 32 ` | ` 2 ` | ` divination ` | ` true ` | ` false ` | ` false ` |
| ` 33 ` | ` 2 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 34 ` | ` 2 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 35 ` | ` 2 ` | ` illusion ` | ` true ` | ` true ` | ` false ` |
| ` 36 ` | ` 2 ` | ` evocation ` | ` true ` | ` false ` | ` false ` |
| ` 37 ` | ` 2 ` | ` abjuration ` | ` false ` | ` false ` | ` true ` |
| ` 38 ` | ` 2 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 39 ` | ` 3 ` | ` necromancy ` | ` false ` | ` false ` | ` false ` |
| ` 40 ` | ` 3 ` | ` abjuration ` | ` true ` | ` false ` | ` false ` |
| ` 41 ` | ` 3 ` | ` necromancy ` | ` true ` | ` false ` | ` false ` |
| ` 42 ` | ` 3 ` | ` divination ` | ` true ` | ` false ` | ` true ` |
| ` 43 ` | ` 3 ` | ` conjuration ` | ` false ` | ` false ` | ` false ` |
| ` 44 ` | ` 3 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 45 ` | ` 3 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 46 ` | ` 3 ` | ` abjuration ` | ` false ` | ` false ` | ` true ` |
| ` 47 ` | ` 3 ` | ` abjuration ` | ` false ` | ` false ` | ` true ` |
| ` 48 ` | ` 3 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 49 ` | ` 3 ` | ` transmutation ` | ` false ` | ` true ` | ` false ` |
| ` 50 ` | ` 3 ` | ` abjuration ` | ` true ` | ` false ` | ` false ` |
| ` 51 ` | ` 3 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 52 ` | ` 3 ` | ` necromancy ` | ` false ` | ` false ` | ` true ` |
| ` 53 ` | ` 3 ` | ` divination ` | ` false ` | ` false ` | ` false ` |
| ` 54 ` | ` 3 ` | ` necromancy ` | ` false ` | ` false ` | ` false ` |
| ` 55 ` | ` 3 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 56 ` | ` 3 ` | ` divination ` | ` false ` | ` false ` | ` false ` |
| ` 57 ` | ` 3 ` | ` transmutation ` | ` false ` | ` true ` | ` false ` |
| ` 58 ` | ` 4 ` | ` abjuration ` | ` true ` | ` false ` | ` false ` |
| ` 59 ` | ` 4 ` | ` abjuration ` | ` true ` | ` false ` | ` false ` |
| ` 60 ` | ` 4 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 61 ` | ` 4 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 62 ` | ` 4 ` | ` divination ` | ` false ` | ` true ` | ` true ` |
| ` 63 ` | ` 4 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 64 ` | ` 4 ` | ` conjuration ` | ` false ` | ` false ` | ` false ` |
| ` 65 ` | ` 4 ` | ` divination ` | ` true ` | ` false ` | ` false ` |
| ` 66 ` | ` 4 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 67 ` | ` 5 ` | ` divination ` | ` false ` | ` true ` | ` false ` |
| ` 68 ` | ` 5 ` | ` necromancy ` | ` false ` | ` false ` | ` false ` |
| ` 69 ` | ` 5 ` | ` abjuration ` | ` true ` | ` false ` | ` false ` |
| ` 70 ` | ` 5 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 71 ` | ` 5 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 72 ` | ` 5 ` | ` abjuration ` | ` false ` | ` false ` | ` true ` |
| ` 73 ` | ` 5 ` | ` abjuration ` | ` false ` | ` false ` | ` true ` |
| ` 74 ` | ` 5 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 75 ` | ` 5 ` | ` divination ` | ` false ` | ` false ` | ` true ` |
| ` 76 ` | ` 5 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 77 ` | ` 5 ` | ` abjuration ` | ` false ` | ` false ` | ` true ` |
| ` 78 ` | ` 5 ` | ` necromancy ` | ` false ` | ` false ` | ` true ` |
| ` 79 ` | ` 5 ` | ` divination ` | ` true ` | ` false ` | ` true ` |
| ` 80 ` | ` 6 ` | ` evocation ` | ` true ` | ` false ` | ` false ` |
| ` 81 ` | ` 6 ` | ` necromancy ` | ` false ` | ` false ` | ` true ` |
| ` 82 ` | ` 6 ` | ` divination ` | ` true ` | ` false ` | ` true ` |
| ` 83 ` | ` 6 ` | ` abjuration ` | ` false ` | ` true ` | ` true ` |
| ` 84 ` | ` 6 ` | ` necromancy ` | ` false ` | ` false ` | ` false ` |
| ` 85 ` | ` 6 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 86 ` | ` 6 ` | ` conjuration ` | ` false ` | ` false ` | ` true ` |
| ` 87 ` | ` 6 ` | ` conjuration ` | ` false ` | ` false ` | ` false ` |
| ` 88 ` | ` 6 ` | ` evocation ` | ` true ` | ` false ` | ` false ` |
| ` 89 ` | ` 6 ` | ` divination ` | ` false ` | ` false ` | ` true ` |
| ` 90 ` | ` 6 ` | ` conjuration ` | ` false ` | ` false ` | ` false ` |
| ` 91 ` | ` 7 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 92 ` | ` 7 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 93 ` | ` 7 ` | ` conjuration ` | ` false ` | ` false ` | ` false ` |
| ` 94 ` | ` 7 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 95 ` | ` 7 ` | ` conjuration ` | ` false ` | ` false ` | ` true ` |
| ` 96 ` | ` 7 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 97 ` | ` 7 ` | ` necromancy ` | ` false ` | ` false ` | ` true ` |
| ` 98 ` | ` 7 ` | ` abjuration ` | ` false ` | ` false ` | ` true ` |
| ` 99 ` | ` 8 ` | ` abjuration ` | ` true ` | ` false ` | ` false ` |
| ` 100 ` | ` 8 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 101 ` | ` 8 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 102 ` | ` 8 ` | ` abjuration ` | ` true ` | ` false ` | ` true ` |
| ` 103 ` | ` 8 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 104 ` | ` 9 ` | ` necromancy ` | ` false ` | ` false ` | ` true ` |
| ` 105 ` | ` 9 ` | ` conjuration ` | ` true ` | ` false ` | ` true ` |
| ` 106 ` | ` 9 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 107 ` | ` 9 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 108 ` | ` 9 ` | ` necromancy ` | ` false ` | ` false ` | ` true ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/guidance ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/1/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/light ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/2/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/mending ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/3/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/resistance ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/4/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/sacred_flame ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/5/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/spare_the_dying ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/6/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/thaumaturgy ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/7/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/bane ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/8/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/bless ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/9/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/command ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/10/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/create_or_destroy_water ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/11/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/cure_wounds ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/12/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/detect_evil_and_good ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/13/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/detect_magic ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/14/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/detect_poison_and_disease ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/15/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/guiding_bolt ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/16/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/healing_word ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/17/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/inflict_wounds ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/18/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/protection_from_evil_and_good ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/19/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/purify_food_and_drink ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/20/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/sanctuary ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/21/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/shield_of_faith ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/22/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/aid ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/23/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/augury ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/24/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/blindness_deafness ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/25/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/calm_emotions ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/26/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/continual_flame ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/27/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/enhance_ability ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/28/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/find_traps ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/29/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/gentle_repose ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/30/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/hold_person ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/31/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/lesser_restoration ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/32/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/locate_object ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/33/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/prayer_of_healing ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/34/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/protection_from_poison ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/35/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/silence ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/36/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/spiritual_weapon ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/37/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/warding_bond ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/38/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/zone_of_truth ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/39/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/animate_dead ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/40/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/beacon_of_hope ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/41/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/bestow_curse ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/42/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/clairvoyance ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/43/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/create_food_and_water ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/44/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/daylight ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/45/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/dispel_magic ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/46/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/glyph_of_warding ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/47/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/magic_circle ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/48/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/mass_healing_word ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/49/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/meld_into_stone ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/50/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/protection_from_energy ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/51/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/remove_curse ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/52/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/revivify ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/53/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/sending ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/54/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/speak_with_dead ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/55/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/spirit_guardians ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/56/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/tongues ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/57/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/water_walk ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/58/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/aura_of_life ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/59/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/banishment ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/60/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/control_water ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/61/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/death_ward ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/62/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/divination ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/63/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/freedom_of_movement ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/64/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/guardian_of_faith ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/65/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/locate_creature ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/66/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/stone_shape ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/67/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/commune ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/68/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/contagion ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/69/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/dispel_evil_and_good ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/70/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/flame_strike ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/71/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/geas ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/72/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/greater_restoration ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/73/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/hallow ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/74/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/insect_plague ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/75/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/legend_lore ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/76/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/mass_cure_wounds ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/77/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/planar_binding ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/78/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/raise_dead ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/79/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/scrying ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/80/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/blade_barrier ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/81/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/create_undead ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/82/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/find_the_path ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/83/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/forbiddance ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/84/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/harm ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/85/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/heal ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/86/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/heroes_feast ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/87/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/planar_ally ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/88/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/sunbeam ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/89/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/true_seeing ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/90/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/word_of_recall ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/91/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/conjure_celestial ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/92/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/divine_word ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/93/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/etherealness ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/94/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/fire_storm ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/95/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/plane_shift ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/96/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/regenerate ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/97/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/resurrection ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/98/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/symbol ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/99/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/antimagic_field ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/100/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/control_weather ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/101/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/earthquake ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/102/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/holy_aura ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/103/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/sunburst ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/104/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/astral_projection ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/105/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/gate ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/106/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/mass_heal ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/107/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/power_word_heal ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/108/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/true_resurrection ` |
