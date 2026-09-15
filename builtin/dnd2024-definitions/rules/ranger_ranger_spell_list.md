---
splat_schema: 1
splat_id: 9ff830e7-389d-50c9-a6ba-f1f356e041da
splat_kind: rule
splat_key: dnd2024:rules/ranger_ranger_spell_list
splat_name: Ranger Spell List
splat_origin:
  key: dnd2024:reference/pages/060
splat_sources:
- title: SRD 5.2.1
  locator: pp. 60
  url: https://www.dndbeyond.com/srd
---

# Ranger Spell List

## Source text

<!-- SRD page 60 -->

This section presents the Ranger spell list. The\
spells are organized by spell level and then alpha-\
betized, and each spell’s school of magic is listed.\
In the Special column, C means the spell requires\
Concentration, R means it’s a Ritual, and M means it\
requires a specific Material component.\
Level 1 Ranger Spells\
Spell School Special\
Alarm Abjuration R\
Animal Friendship Enchantment —\
Cure Wounds Abjuration —\
Detect Magic Divination C, R\
Detect Poison and Disease Divination C, R\
Ensnaring Strike Conjuration C\
Entangle Conjuration C\
Fog Cloud Conjuration C\
Goodberry Conjuration —\
Hunter’s Mark Divination C\
Jump Transmutation —\
Longstrider Transmutation —\
Speak with Animals Divination R\
Level 2 Ranger Spells\
Spell School Special\
Aid Abjuration —\
Animal Messenger Enchantment R\
Barkskin Transmutation —\
Darkvision Transmutation —\
Enhance Ability Transmutation C\
Find Traps Divination —\
Gust of Wind Evocation C\
Lesser Restoration Abjuration —\
Locate Animals or Plants Divination R\
Locate Object Divination C\
Magic Weapon Transmutation —\
Pass without Trace Abjuration C\
Protection from Poison Abjuration —\
Silence Illusion C, R\
Spike Growth Transmutation C\
Level 3 Ranger Spells\
Spell School Special\
Conjure Animals Conjuration C\
Daylight Evocation —\
Dispel Magic Abjuration —\
Meld into Stone Transmutation R\
Nondetection Abjuration M\
Plant Growth Transmutation —\
Protection from Energy Abjuration C\
Revivify Necromancy M\
Speak with Plants Transmutation —\
Water Breathing Transmutation R\
Water Walk Transmutation R\
Wind Wall Evocation C\
Level 4 Ranger Spells\
Spell School Special\
Conjure Woodland Beings Conjuration C\
Dominate Beast Enchantment C\
Freedom of Movement Abjuration —\
Locate Creature Divination C\
Stoneskin Transmutation C, M\
Level 5 Ranger Spells\
Spell School Special\
Commune with Nature Divination R\
Greater Restoration Abjuration M\
Tree Stride Conjuration C\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/ranger_ranger_spell_list ` |
| ` implementation ` | ` dnd2024:rules_ranger_ranger_spell_list ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` sorting ` | ` ["spell_level", "name"] ` |

#### Class

<!-- splat:values path=splat_rule/parameters/class -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/ranger ` |

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | level | school | concentration | ritual | specific_material |
| --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` abjuration ` | ` false ` | ` true ` | ` false ` |
| ` 1 ` | ` 1 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 2 ` | ` 1 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 3 ` | ` 1 ` | ` divination ` | ` true ` | ` true ` | ` false ` |
| ` 4 ` | ` 1 ` | ` divination ` | ` true ` | ` true ` | ` false ` |
| ` 5 ` | ` 1 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 6 ` | ` 1 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 7 ` | ` 1 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 8 ` | ` 1 ` | ` conjuration ` | ` false ` | ` false ` | ` false ` |
| ` 9 ` | ` 1 ` | ` divination ` | ` true ` | ` false ` | ` false ` |
| ` 10 ` | ` 1 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 11 ` | ` 1 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 12 ` | ` 1 ` | ` divination ` | ` false ` | ` true ` | ` false ` |
| ` 13 ` | ` 2 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 14 ` | ` 2 ` | ` enchantment ` | ` false ` | ` true ` | ` false ` |
| ` 15 ` | ` 2 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 16 ` | ` 2 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 17 ` | ` 2 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 18 ` | ` 2 ` | ` divination ` | ` false ` | ` false ` | ` false ` |
| ` 19 ` | ` 2 ` | ` evocation ` | ` true ` | ` false ` | ` false ` |
| ` 20 ` | ` 2 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 21 ` | ` 2 ` | ` divination ` | ` false ` | ` true ` | ` false ` |
| ` 22 ` | ` 2 ` | ` divination ` | ` true ` | ` false ` | ` false ` |
| ` 23 ` | ` 2 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 24 ` | ` 2 ` | ` abjuration ` | ` true ` | ` false ` | ` false ` |
| ` 25 ` | ` 2 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 26 ` | ` 2 ` | ` illusion ` | ` true ` | ` true ` | ` false ` |
| ` 27 ` | ` 2 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 28 ` | ` 3 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 29 ` | ` 3 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 30 ` | ` 3 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 31 ` | ` 3 ` | ` transmutation ` | ` false ` | ` true ` | ` false ` |
| ` 32 ` | ` 3 ` | ` abjuration ` | ` false ` | ` false ` | ` true ` |
| ` 33 ` | ` 3 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 34 ` | ` 3 ` | ` abjuration ` | ` true ` | ` false ` | ` false ` |
| ` 35 ` | ` 3 ` | ` necromancy ` | ` false ` | ` false ` | ` true ` |
| ` 36 ` | ` 3 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 37 ` | ` 3 ` | ` transmutation ` | ` false ` | ` true ` | ` false ` |
| ` 38 ` | ` 3 ` | ` transmutation ` | ` false ` | ` true ` | ` false ` |
| ` 39 ` | ` 3 ` | ` evocation ` | ` true ` | ` false ` | ` false ` |
| ` 40 ` | ` 4 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 41 ` | ` 4 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 42 ` | ` 4 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 43 ` | ` 4 ` | ` divination ` | ` true ` | ` false ` | ` false ` |
| ` 44 ` | ` 4 ` | ` transmutation ` | ` true ` | ` false ` | ` true ` |
| ` 45 ` | ` 5 ` | ` divination ` | ` false ` | ` true ` | ` false ` |
| ` 46 ` | ` 5 ` | ` abjuration ` | ` false ` | ` false ` | ` true ` |
| ` 47 ` | ` 5 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/alarm ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/1/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/animal_friendship ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/2/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/cure_wounds ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/3/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/detect_magic ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/4/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/detect_poison_and_disease ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/5/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/ensnaring_strike ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/6/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/entangle ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/7/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/fog_cloud ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/8/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/goodberry ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/9/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/hunter_s_mark ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/10/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/jump ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/11/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/longstrider ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/12/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/speak_with_animals ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/13/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/aid ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/14/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/animal_messenger ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/15/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/barkskin ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/16/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/darkvision ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/17/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/enhance_ability ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/18/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/find_traps ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/19/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/gust_of_wind ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/20/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/lesser_restoration ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/21/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/locate_animals_or_plants ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/22/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/locate_object ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/23/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/magic_weapon ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/24/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/pass_without_trace ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/25/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/protection_from_poison ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/26/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/silence ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/27/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/spike_growth ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/28/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/conjure_animals ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/29/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/daylight ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/30/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/dispel_magic ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/31/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/meld_into_stone ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/32/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/nondetection ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/33/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/plant_growth ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/34/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/protection_from_energy ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/35/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/revivify ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/36/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/speak_with_plants ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/37/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/water_breathing ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/38/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/water_walk ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/39/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/wind_wall ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/40/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/conjure_woodland_beings ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/41/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/dominate_beast ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/42/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/freedom_of_movement ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/43/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/locate_creature ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/44/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/stoneskin ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/45/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/commune_with_nature ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/46/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/greater_restoration ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/47/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/tree_stride ` |
