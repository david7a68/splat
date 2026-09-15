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

<!-- splat:table
path: splat_rule/parameters/spells
key: Spell
order:
- alarm
- animal_friendship
- cure_wounds
- detect_magic
- detect_poison_and_disease
- ensnaring_strike
- entangle
- fog_cloud
- goodberry
- hunter_s_mark
- jump
- longstrider
- speak_with_animals
- aid
- animal_messenger
- barkskin
- darkvision
- enhance_ability
- find_traps
- gust_of_wind
- lesser_restoration
- locate_animals_or_plants
- locate_object
- magic_weapon
- pass_without_trace
- protection_from_poison
- silence
- spike_growth
- conjure_animals
- daylight
- dispel_magic
- meld_into_stone
- nondetection
- plant_growth
- protection_from_energy
- revivify
- speak_with_plants
- water_breathing
- water_walk
- wind_wall
- conjure_woodland_beings
- dominate_beast
- freedom_of_movement
- locate_creature
- stoneskin
- commune_with_nature
- greater_restoration
- tree_stride
columns:
  Spell:
    path: spell
    type: reference
  Level:
    path: level
    type: integer
  School:
    path: school
    type: symbol
  Concentration:
    path: concentration
    type: boolean
  Ritual:
    path: ritual
    type: boolean
  Specific material:
    path: specific_material
    type: boolean
-->

| Spell | Level | School | Concentration | Ritual | Specific material |
| --- | --- | --- | --- | --- | --- |
| [Alarm](<../spells/alarm.md> "dnd2024:spells/alarm") | ` 1 ` | ` Abjuration ` | false | true | false |
| [Animal Friendship](<../spells/animal_friendship.md> "dnd2024:spells/animal_friendship") | ` 1 ` | ` Enchantment ` | false | false | false |
| [Cure Wounds](<../spells/cure_wounds.md> "dnd2024:spells/cure_wounds") | ` 1 ` | ` Abjuration ` | false | false | false |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | ` 1 ` | ` Divination ` | true | true | false |
| [Detect Poison and Disease](<../spells/detect_poison_and_disease.md> "dnd2024:spells/detect_poison_and_disease") | ` 1 ` | ` Divination ` | true | true | false |
| [Ensnaring Strike](<../spells/ensnaring_strike.md> "dnd2024:spells/ensnaring_strike") | ` 1 ` | ` Conjuration ` | true | false | false |
| [Entangle](<../spells/entangle.md> "dnd2024:spells/entangle") | ` 1 ` | ` Conjuration ` | true | false | false |
| [Fog Cloud](<../spells/fog_cloud.md> "dnd2024:spells/fog_cloud") | ` 1 ` | ` Conjuration ` | true | false | false |
| [Goodberry](<../spells/goodberry.md> "dnd2024:spells/goodberry") | ` 1 ` | ` Conjuration ` | false | false | false |
| [Hunter’s Mark](<../spells/hunter_s_mark.md> "dnd2024:spells/hunter_s_mark") | ` 1 ` | ` Divination ` | true | false | false |
| [Jump](<../spells/jump.md> "dnd2024:spells/jump") | ` 1 ` | ` Transmutation ` | false | false | false |
| [Longstrider](<../spells/longstrider.md> "dnd2024:spells/longstrider") | ` 1 ` | ` Transmutation ` | false | false | false |
| [Speak with Animals](<../spells/speak_with_animals.md> "dnd2024:spells/speak_with_animals") | ` 1 ` | ` Divination ` | false | true | false |
| [Aid](<../spells/aid.md> "dnd2024:spells/aid") | ` 2 ` | ` Abjuration ` | false | false | false |
| [Animal Messenger](<../spells/animal_messenger.md> "dnd2024:spells/animal_messenger") | ` 2 ` | ` Enchantment ` | false | true | false |
| [Barkskin](<../spells/barkskin.md> "dnd2024:spells/barkskin") | ` 2 ` | ` Transmutation ` | false | false | false |
| [Darkvision](<../spells/darkvision.md> "dnd2024:spells/darkvision") | ` 2 ` | ` Transmutation ` | false | false | false |
| [Enhance Ability](<../spells/enhance_ability.md> "dnd2024:spells/enhance_ability") | ` 2 ` | ` Transmutation ` | true | false | false |
| [Find Traps](<../spells/find_traps.md> "dnd2024:spells/find_traps") | ` 2 ` | ` Divination ` | false | false | false |
| [Gust of Wind](<../spells/gust_of_wind.md> "dnd2024:spells/gust_of_wind") | ` 2 ` | ` Evocation ` | true | false | false |
| [Lesser Restoration](<../spells/lesser_restoration.md> "dnd2024:spells/lesser_restoration") | ` 2 ` | ` Abjuration ` | false | false | false |
| [Locate Animals or Plants](<../spells/locate_animals_or_plants.md> "dnd2024:spells/locate_animals_or_plants") | ` 2 ` | ` Divination ` | false | true | false |
| [Locate Object](<../spells/locate_object.md> "dnd2024:spells/locate_object") | ` 2 ` | ` Divination ` | true | false | false |
| [Magic Weapon](<../spells/magic_weapon.md> "dnd2024:spells/magic_weapon") | ` 2 ` | ` Transmutation ` | false | false | false |
| [Pass without Trace](<../spells/pass_without_trace.md> "dnd2024:spells/pass_without_trace") | ` 2 ` | ` Abjuration ` | true | false | false |
| [Protection from Poison](<../spells/protection_from_poison.md> "dnd2024:spells/protection_from_poison") | ` 2 ` | ` Abjuration ` | false | false | false |
| [Silence](<../spells/silence.md> "dnd2024:spells/silence") | ` 2 ` | ` Illusion ` | true | true | false |
| [Spike Growth](<../spells/spike_growth.md> "dnd2024:spells/spike_growth") | ` 2 ` | ` Transmutation ` | true | false | false |
| [Conjure Animals](<../spells/conjure_animals.md> "dnd2024:spells/conjure_animals") | ` 3 ` | ` Conjuration ` | true | false | false |
| [Daylight](<../spells/daylight.md> "dnd2024:spells/daylight") | ` 3 ` | ` Evocation ` | false | false | false |
| [Dispel Magic](<../spells/dispel_magic.md> "dnd2024:spells/dispel_magic") | ` 3 ` | ` Abjuration ` | false | false | false |
| [Meld into Stone](<../spells/meld_into_stone.md> "dnd2024:spells/meld_into_stone") | ` 3 ` | ` Transmutation ` | false | true | false |
| [Nondetection](<../spells/nondetection.md> "dnd2024:spells/nondetection") | ` 3 ` | ` Abjuration ` | false | false | true |
| [Plant Growth](<../spells/plant_growth.md> "dnd2024:spells/plant_growth") | ` 3 ` | ` Transmutation ` | false | false | false |
| [Protection from Energy](<../spells/protection_from_energy.md> "dnd2024:spells/protection_from_energy") | ` 3 ` | ` Abjuration ` | true | false | false |
| [Revivify](<../spells/revivify.md> "dnd2024:spells/revivify") | ` 3 ` | ` Necromancy ` | false | false | true |
| [Speak with Plants](<../spells/speak_with_plants.md> "dnd2024:spells/speak_with_plants") | ` 3 ` | ` Transmutation ` | false | false | false |
| [Water Breathing](<../spells/water_breathing.md> "dnd2024:spells/water_breathing") | ` 3 ` | ` Transmutation ` | false | true | false |
| [Water Walk](<../spells/water_walk.md> "dnd2024:spells/water_walk") | ` 3 ` | ` Transmutation ` | false | true | false |
| [Wind Wall](<../spells/wind_wall.md> "dnd2024:spells/wind_wall") | ` 3 ` | ` Evocation ` | true | false | false |
| [Conjure Woodland Beings](<../spells/conjure_woodland_beings.md> "dnd2024:spells/conjure_woodland_beings") | ` 4 ` | ` Conjuration ` | true | false | false |
| [Dominate Beast](<../spells/dominate_beast.md> "dnd2024:spells/dominate_beast") | ` 4 ` | ` Enchantment ` | true | false | false |
| [Freedom of Movement](<../spells/freedom_of_movement.md> "dnd2024:spells/freedom_of_movement") | ` 4 ` | ` Abjuration ` | false | false | false |
| [Locate Creature](<../spells/locate_creature.md> "dnd2024:spells/locate_creature") | ` 4 ` | ` Divination ` | true | false | false |
| [Stoneskin](<../spells/stoneskin.md> "dnd2024:spells/stoneskin") | ` 4 ` | ` Transmutation ` | true | false | true |
| [Commune with Nature](<../spells/commune_with_nature.md> "dnd2024:spells/commune_with_nature") | ` 5 ` | ` Divination ` | false | true | false |
| [Greater Restoration](<../spells/greater_restoration.md> "dnd2024:spells/greater_restoration") | ` 5 ` | ` Abjuration ` | false | false | true |
| [Tree Stride](<../spells/tree_stride.md> "dnd2024:spells/tree_stride") | ` 5 ` | ` Conjuration ` | true | false | false |
