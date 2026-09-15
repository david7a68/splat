---
splat_schema: 1
splat_id: b53a8df2-866d-53f8-a999-eceaeac7369c
splat_kind: rule
splat_key: dnd2024:rules/druid_druid_spell_list
splat_name: Druid Spell List
splat_origin:
  key: dnd2024:reference/pages/044
splat_sources:
- title: SRD 5.2.1
  locator: pp. 44, 45
  url: https://www.dndbeyond.com/srd
---

# Druid Spell List

## Source text

<!-- SRD page 44 -->

This section presents the Druid spell list. The spells\
are organized by spell level and then alphabetized,\
and each spell’s school of magic is listed. In the Spe-\
cial column, C means the spell requires Concentra-\
tion, R means it’s a Ritual, and M means it requires a\
specific Material component.\
Cantrips (Level 0 Druid Spells)\
Spell School Special\
Druidcraft Transmutation —\
Elementalism Transmutation —\
Guidance Divination C\
Mending Transmutation —\
Message Transmutation —\
Poison Spray Necromancy —\
Produce Flame Conjuration —\
Resistance Abjuration C\
Shillelagh Transmutation —\
Spare the Dying Necromancy —\
Starry Wisp Evocation —\
Level 1 Druid Spells\
Spell School Special\
Animal Friendship Enchantment —\
Charm Person Enchantment —\
Create or Destroy Water Transmutation —\
Cure Wounds Abjuration —\
Detect Magic Divination C, R\
Detect Poison and Disease Divination C, R\
Entangle Conjuration C\
Faerie Fire Evocation C\
Fog Cloud Conjuration C\
Goodberry Conjuration —\
Healing Word Abjuration —\
Ice Knife Conjuration —\
Jump Transmutation —\
Longstrider Transmutation —\
Protection from Evil and Good Abjuration C, M\
Purify Food and Drink Transmutation R\
Speak with Animals Divination R\
Thunderwave Evocation —\
Level 2 Druid Spells\
Spell School Special\
Aid Abjuration —\
Animal Messenger Enchantment R\
Augury Divination R, M\
Barkskin Transmutation —\
Continual Flame Evocation M\
Darkvision Transmutation —\
Enhance Ability Transmutation C\
Enlarge/Reduce Transmutation C\
Find Traps Divination —\
Flame Blade Evocation C\
Flaming Sphere Evocation C\
Gust of Wind Evocation C\
Heat Metal Transmutation C\
Hold Person Enchantment C\
Lesser Restoration Abjuration —\
Locate Animals or Plants Divination R\
Locate Object Divination C\
Moonbeam Evocation C\
Pass without Trace Abjuration C\
Protection from Poison Abjuration —\
Spike Growth Transmutation C\
Level 3 Druid Spells\
Spell School Special\
Call Lightning Conjuration C\
Conjure Animals Conjuration C\
Daylight Evocation —\
Dispel Magic Abjuration —\
Meld into Stone Transmutation R\
Plant Growth Transmutation —\
Protection from Energy Abjuration C\
Revivify Necromancy M\
Sleet Storm Conjuration C\
Speak with Plants Transmutation —\
Water Breathing Transmutation R\
Water Walk Transmutation R\
Wind Wall Evocation C\

<!-- SRD page 45 -->

Level 4 Druid Spells\
Spell School Special\
Blight Necromancy —\
Charm Monster Enchantment —\
Confusion Enchantment C\
Conjure Minor Elementals Conjuration C\
Conjure Woodland Beings Conjuration C\
Control Water Transmutation C\
Divination Divination R, M\
Dominate Beast Enchantment C\
Fire Shield Evocation —\
Freedom of Movement Abjuration —\
Giant Insect Conjuration C\
Hallucinatory Terrain Illusion —\
Ice Storm Evocation —\
Locate Creature Divination C\
Polymorph Transmutation C\
Stone Shape Transmutation —\
Stoneskin Transmutation C, M\
Wall of Fire Evocation C\
Level 5 Druid Spells\
Spell School Special\
Antilife Shell Abjuration C\
Awaken Transmutation M\
Commune with Nature Divination R\
Cone of Cold Evocation —\
Conjure Elemental Conjuration C\
Contagion Necromancy —\
Geas Enchantment —\
Greater Restoration Abjuration M\
Insect Plague Conjuration C\
Mass Cure Wounds Abjuration —\
Planar Binding Abjuration M\
Reincarnate Necromancy M\
Scrying Divination C, M\
Tree Stride Conjuration C\
Wall of Stone Evocation C\
Level 6 Druid Spells\
Spell School Special\
Conjure Fey Conjuration C\
Find the Path Divination C, M\
Flesh to Stone Transmutation C\
Heal Abjuration —\
Heroes’ Feast Conjuration M\
Move Earth Transmutation C\
Sunbeam Evocation C\
Transport via Plants Conjuration —\
Wall of Thorns Conjuration C\
Wind Walk Transmutation —\
Level 7 Druid Spells\
Spell School Special\
Fire Storm Evocation —\
Mirage Arcane Illusion —\
Plane Shift Conjuration M\
Regenerate Transmutation —\
Reverse Gravity Transmutation C\
Symbol Abjuration M\
Level 8 Druid Spells\
Spell School Special\
Animal Shapes Transmutation —\
Antipathy/Sympathy Enchantment —\
Befuddlement Enchantment —\
Control Weather Transmutation C\
Earthquake Transmutation C\
Incendiary Cloud Conjuration C\
Sunburst Evocation —\
Tsunami Conjuration C\
Level 9 Druid Spells\
Spell School Special\
Foresight Divination —\
Shapechange Transmutation C, M\
Storm of Vengeance Conjuration C\
True Resurrection Necromancy M\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/druid_druid_spell_list ` |
| ` implementation ` | ` dnd2024:rules_druid_druid_spell_list ` |

### Parameters

#### Class definition

<!-- splat:values path=splat_rule/parameters/class_definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/druid ` |

#### Spell list

<!-- splat:rows path=splat_rule/parameters/spell_list -->

| Row | level | school | concentration | ritual | specific_material |
| --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 0 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 1 ` | ` 0 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 2 ` | ` 0 ` | ` divination ` | ` true ` | ` false ` | ` false ` |
| ` 3 ` | ` 0 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 4 ` | ` 0 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 5 ` | ` 0 ` | ` necromancy ` | ` false ` | ` false ` | ` false ` |
| ` 6 ` | ` 0 ` | ` conjuration ` | ` false ` | ` false ` | ` false ` |
| ` 7 ` | ` 0 ` | ` abjuration ` | ` true ` | ` false ` | ` false ` |
| ` 8 ` | ` 0 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 9 ` | ` 0 ` | ` necromancy ` | ` false ` | ` false ` | ` false ` |
| ` 10 ` | ` 0 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 11 ` | ` 1 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 12 ` | ` 1 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 13 ` | ` 1 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 14 ` | ` 1 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 15 ` | ` 1 ` | ` divination ` | ` true ` | ` true ` | ` false ` |
| ` 16 ` | ` 1 ` | ` divination ` | ` true ` | ` true ` | ` false ` |
| ` 17 ` | ` 1 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 18 ` | ` 1 ` | ` evocation ` | ` true ` | ` false ` | ` false ` |
| ` 19 ` | ` 1 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 20 ` | ` 1 ` | ` conjuration ` | ` false ` | ` false ` | ` false ` |
| ` 21 ` | ` 1 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 22 ` | ` 1 ` | ` conjuration ` | ` false ` | ` false ` | ` false ` |
| ` 23 ` | ` 1 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 24 ` | ` 1 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 25 ` | ` 1 ` | ` abjuration ` | ` true ` | ` false ` | ` true ` |
| ` 26 ` | ` 1 ` | ` transmutation ` | ` false ` | ` true ` | ` false ` |
| ` 27 ` | ` 1 ` | ` divination ` | ` false ` | ` true ` | ` false ` |
| ` 28 ` | ` 1 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 29 ` | ` 2 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 30 ` | ` 2 ` | ` enchantment ` | ` false ` | ` true ` | ` false ` |
| ` 31 ` | ` 2 ` | ` divination ` | ` false ` | ` true ` | ` true ` |
| ` 32 ` | ` 2 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 33 ` | ` 2 ` | ` evocation ` | ` false ` | ` false ` | ` true ` |
| ` 34 ` | ` 2 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 35 ` | ` 2 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 36 ` | ` 2 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 37 ` | ` 2 ` | ` divination ` | ` false ` | ` false ` | ` false ` |
| ` 38 ` | ` 2 ` | ` evocation ` | ` true ` | ` false ` | ` false ` |
| ` 39 ` | ` 2 ` | ` evocation ` | ` true ` | ` false ` | ` false ` |
| ` 40 ` | ` 2 ` | ` evocation ` | ` true ` | ` false ` | ` false ` |
| ` 41 ` | ` 2 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 42 ` | ` 2 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 43 ` | ` 2 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 44 ` | ` 2 ` | ` divination ` | ` false ` | ` true ` | ` false ` |
| ` 45 ` | ` 2 ` | ` divination ` | ` true ` | ` false ` | ` false ` |
| ` 46 ` | ` 2 ` | ` evocation ` | ` true ` | ` false ` | ` false ` |
| ` 47 ` | ` 2 ` | ` abjuration ` | ` true ` | ` false ` | ` false ` |
| ` 48 ` | ` 2 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 49 ` | ` 2 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 50 ` | ` 3 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 51 ` | ` 3 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 52 ` | ` 3 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 53 ` | ` 3 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 54 ` | ` 3 ` | ` transmutation ` | ` false ` | ` true ` | ` false ` |
| ` 55 ` | ` 3 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 56 ` | ` 3 ` | ` abjuration ` | ` true ` | ` false ` | ` false ` |
| ` 57 ` | ` 3 ` | ` necromancy ` | ` false ` | ` false ` | ` true ` |
| ` 58 ` | ` 3 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 59 ` | ` 3 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 60 ` | ` 3 ` | ` transmutation ` | ` false ` | ` true ` | ` false ` |
| ` 61 ` | ` 3 ` | ` transmutation ` | ` false ` | ` true ` | ` false ` |
| ` 62 ` | ` 3 ` | ` evocation ` | ` true ` | ` false ` | ` false ` |
| ` 63 ` | ` 4 ` | ` necromancy ` | ` false ` | ` false ` | ` false ` |
| ` 64 ` | ` 4 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 65 ` | ` 4 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 66 ` | ` 4 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 67 ` | ` 4 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 68 ` | ` 4 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 69 ` | ` 4 ` | ` divination ` | ` false ` | ` true ` | ` true ` |
| ` 70 ` | ` 4 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 71 ` | ` 4 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 72 ` | ` 4 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 73 ` | ` 4 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 74 ` | ` 4 ` | ` illusion ` | ` false ` | ` false ` | ` false ` |
| ` 75 ` | ` 4 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 76 ` | ` 4 ` | ` divination ` | ` true ` | ` false ` | ` false ` |
| ` 77 ` | ` 4 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 78 ` | ` 4 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 79 ` | ` 4 ` | ` transmutation ` | ` true ` | ` false ` | ` true ` |
| ` 80 ` | ` 4 ` | ` evocation ` | ` true ` | ` false ` | ` false ` |
| ` 81 ` | ` 5 ` | ` abjuration ` | ` true ` | ` false ` | ` false ` |
| ` 82 ` | ` 5 ` | ` transmutation ` | ` false ` | ` false ` | ` true ` |
| ` 83 ` | ` 5 ` | ` divination ` | ` false ` | ` true ` | ` false ` |
| ` 84 ` | ` 5 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 85 ` | ` 5 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 86 ` | ` 5 ` | ` necromancy ` | ` false ` | ` false ` | ` false ` |
| ` 87 ` | ` 5 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 88 ` | ` 5 ` | ` abjuration ` | ` false ` | ` false ` | ` true ` |
| ` 89 ` | ` 5 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 90 ` | ` 5 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 91 ` | ` 5 ` | ` abjuration ` | ` false ` | ` false ` | ` true ` |
| ` 92 ` | ` 5 ` | ` necromancy ` | ` false ` | ` false ` | ` true ` |
| ` 93 ` | ` 5 ` | ` divination ` | ` true ` | ` false ` | ` true ` |
| ` 94 ` | ` 5 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 95 ` | ` 5 ` | ` evocation ` | ` true ` | ` false ` | ` false ` |
| ` 96 ` | ` 6 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 97 ` | ` 6 ` | ` divination ` | ` true ` | ` false ` | ` true ` |
| ` 98 ` | ` 6 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 99 ` | ` 6 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 100 ` | ` 6 ` | ` conjuration ` | ` false ` | ` false ` | ` true ` |
| ` 101 ` | ` 6 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 102 ` | ` 6 ` | ` evocation ` | ` true ` | ` false ` | ` false ` |
| ` 103 ` | ` 6 ` | ` conjuration ` | ` false ` | ` false ` | ` false ` |
| ` 104 ` | ` 6 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 105 ` | ` 6 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 106 ` | ` 7 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 107 ` | ` 7 ` | ` illusion ` | ` false ` | ` false ` | ` false ` |
| ` 108 ` | ` 7 ` | ` conjuration ` | ` false ` | ` false ` | ` true ` |
| ` 109 ` | ` 7 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 110 ` | ` 7 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 111 ` | ` 7 ` | ` abjuration ` | ` false ` | ` false ` | ` true ` |
| ` 112 ` | ` 8 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 113 ` | ` 8 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 114 ` | ` 8 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 115 ` | ` 8 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 116 ` | ` 8 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 117 ` | ` 8 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 118 ` | ` 8 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 119 ` | ` 8 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 120 ` | ` 9 ` | ` divination ` | ` false ` | ` false ` | ` false ` |
| ` 121 ` | ` 9 ` | ` transmutation ` | ` true ` | ` false ` | ` true ` |
| ` 122 ` | ` 9 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 123 ` | ` 9 ` | ` necromancy ` | ` false ` | ` false ` | ` true ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/druidcraft ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/1/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/elementalism ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/2/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/guidance ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/3/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/mending ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/4/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/message ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/5/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/poison_spray ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/6/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/produce_flame ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/7/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/resistance ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/8/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/shillelagh ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/9/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/spare_the_dying ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/10/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/starry_wisp ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/11/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/animal_friendship ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/12/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/charm_person ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/13/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/create_or_destroy_water ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/14/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/cure_wounds ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/15/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/detect_magic ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/16/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/detect_poison_and_disease ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/17/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/entangle ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/18/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/faerie_fire ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/19/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/fog_cloud ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/20/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/goodberry ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/21/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/healing_word ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/22/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/ice_knife ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/23/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/jump ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/24/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/longstrider ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/25/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/protection_from_evil_and_good ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/26/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/purify_food_and_drink ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/27/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/speak_with_animals ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/28/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/thunderwave ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/29/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/aid ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/30/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/animal_messenger ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/31/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/augury ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/32/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/barkskin ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/33/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/continual_flame ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/34/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/darkvision ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/35/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/enhance_ability ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/36/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/enlarge_reduce ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/37/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/find_traps ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/38/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/flame_blade ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/39/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/flaming_sphere ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/40/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/gust_of_wind ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/41/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/heat_metal ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/42/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/hold_person ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/43/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/lesser_restoration ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/44/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/locate_animals_or_plants ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/45/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/locate_object ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/46/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/moonbeam ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/47/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/pass_without_trace ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/48/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/protection_from_poison ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/49/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/spike_growth ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/50/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/call_lightning ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/51/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/conjure_animals ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/52/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/daylight ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/53/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/dispel_magic ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/54/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/meld_into_stone ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/55/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/plant_growth ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/56/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/protection_from_energy ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/57/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/revivify ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/58/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/sleet_storm ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/59/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/speak_with_plants ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/60/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/water_breathing ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/61/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/water_walk ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/62/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/wind_wall ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/63/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/blight ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/64/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/charm_monster ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/65/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/confusion ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/66/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/conjure_minor_elementals ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/67/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/conjure_woodland_beings ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/68/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/control_water ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/69/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/divination ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/70/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/dominate_beast ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/71/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/fire_shield ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/72/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/freedom_of_movement ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/73/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/giant_insect ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/74/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/hallucinatory_terrain ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/75/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/ice_storm ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/76/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/locate_creature ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/77/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/polymorph ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/78/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/stone_shape ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/79/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/stoneskin ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/80/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/wall_of_fire ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/81/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/antilife_shell ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/82/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/awaken ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/83/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/commune_with_nature ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/84/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/cone_of_cold ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/85/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/conjure_elemental ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/86/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/contagion ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/87/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/geas ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/88/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/greater_restoration ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/89/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/insect_plague ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/90/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/mass_cure_wounds ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/91/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/planar_binding ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/92/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/reincarnate ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/93/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/scrying ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/94/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/tree_stride ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/95/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/wall_of_stone ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/96/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/conjure_fey ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/97/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/find_the_path ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/98/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/flesh_to_stone ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/99/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/heal ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/100/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/heroes_feast ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/101/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/move_earth ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/102/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/sunbeam ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/103/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/transport_via_plants ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/104/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/wall_of_thorns ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/105/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/wind_walk ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/106/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/fire_storm ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/107/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/mirage_arcane ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/108/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/plane_shift ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/109/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/regenerate ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/110/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/reverse_gravity ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/111/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/symbol ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/112/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/animal_shapes ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/113/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/antipathy_sympathy ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/114/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/befuddlement ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/115/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/control_weather ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/116/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/earthquake ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/117/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/incendiary_cloud ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/118/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/sunburst ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/119/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/tsunami ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/120/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/foresight ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/121/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/shapechange ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/122/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/storm_of_vengeance ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_list/123/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/true_resurrection ` |
