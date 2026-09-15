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

<!-- splat:table
path: splat_rule/parameters/spell_list
key: Spell
order:
- druidcraft
- elementalism
- guidance
- mending
- message
- poison_spray
- produce_flame
- resistance
- shillelagh
- spare_the_dying
- starry_wisp
- animal_friendship
- charm_person
- create_or_destroy_water
- cure_wounds
- detect_magic
- detect_poison_and_disease
- entangle
- faerie_fire
- fog_cloud
- goodberry
- healing_word
- ice_knife
- jump
- longstrider
- protection_from_evil_and_good
- purify_food_and_drink
- speak_with_animals
- thunderwave
- aid
- animal_messenger
- augury
- barkskin
- continual_flame
- darkvision
- enhance_ability
- enlarge_reduce
- find_traps
- flame_blade
- flaming_sphere
- gust_of_wind
- heat_metal
- hold_person
- lesser_restoration
- locate_animals_or_plants
- locate_object
- moonbeam
- pass_without_trace
- protection_from_poison
- spike_growth
- call_lightning
- conjure_animals
- daylight
- dispel_magic
- meld_into_stone
- plant_growth
- protection_from_energy
- revivify
- sleet_storm
- speak_with_plants
- water_breathing
- water_walk
- wind_wall
- blight
- charm_monster
- confusion
- conjure_minor_elementals
- conjure_woodland_beings
- control_water
- divination
- dominate_beast
- fire_shield
- freedom_of_movement
- giant_insect
- hallucinatory_terrain
- ice_storm
- locate_creature
- polymorph
- stone_shape
- stoneskin
- wall_of_fire
- antilife_shell
- awaken
- commune_with_nature
- cone_of_cold
- conjure_elemental
- contagion
- geas
- greater_restoration
- insect_plague
- mass_cure_wounds
- planar_binding
- reincarnate
- scrying
- tree_stride
- wall_of_stone
- conjure_fey
- find_the_path
- flesh_to_stone
- heal
- heroes_feast
- move_earth
- sunbeam
- transport_via_plants
- wall_of_thorns
- wind_walk
- fire_storm
- mirage_arcane
- plane_shift
- regenerate
- reverse_gravity
- symbol
- animal_shapes
- antipathy_sympathy
- befuddlement
- control_weather
- earthquake
- incendiary_cloud
- sunburst
- tsunami
- foresight
- shapechange
- storm_of_vengeance
- true_resurrection
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
| [Druidcraft](<../spells/druidcraft.md> "dnd2024:spells/druidcraft") | ` 0 ` | ` Transmutation ` | false | false | false |
| [Elementalism](<../spells/elementalism.md> "dnd2024:spells/elementalism") | ` 0 ` | ` Transmutation ` | false | false | false |
| [Guidance](<../spells/guidance.md> "dnd2024:spells/guidance") | ` 0 ` | ` Divination ` | true | false | false |
| [Mending](<../spells/mending.md> "dnd2024:spells/mending") | ` 0 ` | ` Transmutation ` | false | false | false |
| [Message](<../spells/message.md> "dnd2024:spells/message") | ` 0 ` | ` Transmutation ` | false | false | false |
| [Poison Spray](<../spells/poison_spray.md> "dnd2024:spells/poison_spray") | ` 0 ` | ` Necromancy ` | false | false | false |
| [Produce Flame](<../spells/produce_flame.md> "dnd2024:spells/produce_flame") | ` 0 ` | ` Conjuration ` | false | false | false |
| [Resistance](<../spells/resistance.md> "dnd2024:spells/resistance") | ` 0 ` | ` Abjuration ` | true | false | false |
| [Shillelagh](<../spells/shillelagh.md> "dnd2024:spells/shillelagh") | ` 0 ` | ` Transmutation ` | false | false | false |
| [Spare the Dying](<../spells/spare_the_dying.md> "dnd2024:spells/spare_the_dying") | ` 0 ` | ` Necromancy ` | false | false | false |
| [Starry Wisp](<../spells/starry_wisp.md> "dnd2024:spells/starry_wisp") | ` 0 ` | ` Evocation ` | false | false | false |
| [Animal Friendship](<../spells/animal_friendship.md> "dnd2024:spells/animal_friendship") | ` 1 ` | ` Enchantment ` | false | false | false |
| [Charm Person](<../spells/charm_person.md> "dnd2024:spells/charm_person") | ` 1 ` | ` Enchantment ` | false | false | false |
| [Create or Destroy Water](<../spells/create_or_destroy_water.md> "dnd2024:spells/create_or_destroy_water") | ` 1 ` | ` Transmutation ` | false | false | false |
| [Cure Wounds](<../spells/cure_wounds.md> "dnd2024:spells/cure_wounds") | ` 1 ` | ` Abjuration ` | false | false | false |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | ` 1 ` | ` Divination ` | true | true | false |
| [Detect Poison and Disease](<../spells/detect_poison_and_disease.md> "dnd2024:spells/detect_poison_and_disease") | ` 1 ` | ` Divination ` | true | true | false |
| [Entangle](<../spells/entangle.md> "dnd2024:spells/entangle") | ` 1 ` | ` Conjuration ` | true | false | false |
| [Faerie Fire](<../spells/faerie_fire.md> "dnd2024:spells/faerie_fire") | ` 1 ` | ` Evocation ` | true | false | false |
| [Fog Cloud](<../spells/fog_cloud.md> "dnd2024:spells/fog_cloud") | ` 1 ` | ` Conjuration ` | true | false | false |
| [Goodberry](<../spells/goodberry.md> "dnd2024:spells/goodberry") | ` 1 ` | ` Conjuration ` | false | false | false |
| [Healing Word](<../spells/healing_word.md> "dnd2024:spells/healing_word") | ` 1 ` | ` Abjuration ` | false | false | false |
| [Ice Knife](<../spells/ice_knife.md> "dnd2024:spells/ice_knife") | ` 1 ` | ` Conjuration ` | false | false | false |
| [Jump](<../spells/jump.md> "dnd2024:spells/jump") | ` 1 ` | ` Transmutation ` | false | false | false |
| [Longstrider](<../spells/longstrider.md> "dnd2024:spells/longstrider") | ` 1 ` | ` Transmutation ` | false | false | false |
| [Protection from Evil and Good](<../spells/protection_from_evil_and_good.md> "dnd2024:spells/protection_from_evil_and_good") | ` 1 ` | ` Abjuration ` | true | false | true |
| [Purify Food and Drink](<../spells/purify_food_and_drink.md> "dnd2024:spells/purify_food_and_drink") | ` 1 ` | ` Transmutation ` | false | true | false |
| [Speak with Animals](<../spells/speak_with_animals.md> "dnd2024:spells/speak_with_animals") | ` 1 ` | ` Divination ` | false | true | false |
| [Thunderwave](<../spells/thunderwave.md> "dnd2024:spells/thunderwave") | ` 1 ` | ` Evocation ` | false | false | false |
| [Aid](<../spells/aid.md> "dnd2024:spells/aid") | ` 2 ` | ` Abjuration ` | false | false | false |
| [Animal Messenger](<../spells/animal_messenger.md> "dnd2024:spells/animal_messenger") | ` 2 ` | ` Enchantment ` | false | true | false |
| [Augury](<../spells/augury.md> "dnd2024:spells/augury") | ` 2 ` | ` Divination ` | false | true | true |
| [Barkskin](<../spells/barkskin.md> "dnd2024:spells/barkskin") | ` 2 ` | ` Transmutation ` | false | false | false |
| [Continual Flame](<../spells/continual_flame.md> "dnd2024:spells/continual_flame") | ` 2 ` | ` Evocation ` | false | false | true |
| [Darkvision](<../spells/darkvision.md> "dnd2024:spells/darkvision") | ` 2 ` | ` Transmutation ` | false | false | false |
| [Enhance Ability](<../spells/enhance_ability.md> "dnd2024:spells/enhance_ability") | ` 2 ` | ` Transmutation ` | true | false | false |
| [Enlarge/Reduce](<../spells/enlarge_reduce.md> "dnd2024:spells/enlarge_reduce") | ` 2 ` | ` Transmutation ` | true | false | false |
| [Find Traps](<../spells/find_traps.md> "dnd2024:spells/find_traps") | ` 2 ` | ` Divination ` | false | false | false |
| [Flame Blade](<../spells/flame_blade.md> "dnd2024:spells/flame_blade") | ` 2 ` | ` Evocation ` | true | false | false |
| [Flaming Sphere](<../spells/flaming_sphere.md> "dnd2024:spells/flaming_sphere") | ` 2 ` | ` Evocation ` | true | false | false |
| [Gust of Wind](<../spells/gust_of_wind.md> "dnd2024:spells/gust_of_wind") | ` 2 ` | ` Evocation ` | true | false | false |
| [Heat Metal](<../spells/heat_metal.md> "dnd2024:spells/heat_metal") | ` 2 ` | ` Transmutation ` | true | false | false |
| [Hold Person](<../spells/hold_person.md> "dnd2024:spells/hold_person") | ` 2 ` | ` Enchantment ` | true | false | false |
| [Lesser Restoration](<../spells/lesser_restoration.md> "dnd2024:spells/lesser_restoration") | ` 2 ` | ` Abjuration ` | false | false | false |
| [Locate Animals or Plants](<../spells/locate_animals_or_plants.md> "dnd2024:spells/locate_animals_or_plants") | ` 2 ` | ` Divination ` | false | true | false |
| [Locate Object](<../spells/locate_object.md> "dnd2024:spells/locate_object") | ` 2 ` | ` Divination ` | true | false | false |
| [Moonbeam](<../spells/moonbeam.md> "dnd2024:spells/moonbeam") | ` 2 ` | ` Evocation ` | true | false | false |
| [Pass without Trace](<../spells/pass_without_trace.md> "dnd2024:spells/pass_without_trace") | ` 2 ` | ` Abjuration ` | true | false | false |
| [Protection from Poison](<../spells/protection_from_poison.md> "dnd2024:spells/protection_from_poison") | ` 2 ` | ` Abjuration ` | false | false | false |
| [Spike Growth](<../spells/spike_growth.md> "dnd2024:spells/spike_growth") | ` 2 ` | ` Transmutation ` | true | false | false |
| [Call Lightning](<../spells/call_lightning.md> "dnd2024:spells/call_lightning") | ` 3 ` | ` Conjuration ` | true | false | false |
| [Conjure Animals](<../spells/conjure_animals.md> "dnd2024:spells/conjure_animals") | ` 3 ` | ` Conjuration ` | true | false | false |
| [Daylight](<../spells/daylight.md> "dnd2024:spells/daylight") | ` 3 ` | ` Evocation ` | false | false | false |
| [Dispel Magic](<../spells/dispel_magic.md> "dnd2024:spells/dispel_magic") | ` 3 ` | ` Abjuration ` | false | false | false |
| [Meld into Stone](<../spells/meld_into_stone.md> "dnd2024:spells/meld_into_stone") | ` 3 ` | ` Transmutation ` | false | true | false |
| [Plant Growth](<../spells/plant_growth.md> "dnd2024:spells/plant_growth") | ` 3 ` | ` Transmutation ` | false | false | false |
| [Protection from Energy](<../spells/protection_from_energy.md> "dnd2024:spells/protection_from_energy") | ` 3 ` | ` Abjuration ` | true | false | false |
| [Revivify](<../spells/revivify.md> "dnd2024:spells/revivify") | ` 3 ` | ` Necromancy ` | false | false | true |
| [Sleet Storm](<../spells/sleet_storm.md> "dnd2024:spells/sleet_storm") | ` 3 ` | ` Conjuration ` | true | false | false |
| [Speak with Plants](<../spells/speak_with_plants.md> "dnd2024:spells/speak_with_plants") | ` 3 ` | ` Transmutation ` | false | false | false |
| [Water Breathing](<../spells/water_breathing.md> "dnd2024:spells/water_breathing") | ` 3 ` | ` Transmutation ` | false | true | false |
| [Water Walk](<../spells/water_walk.md> "dnd2024:spells/water_walk") | ` 3 ` | ` Transmutation ` | false | true | false |
| [Wind Wall](<../spells/wind_wall.md> "dnd2024:spells/wind_wall") | ` 3 ` | ` Evocation ` | true | false | false |
| [Blight](<../spells/blight.md> "dnd2024:spells/blight") | ` 4 ` | ` Necromancy ` | false | false | false |
| [Charm Monster](<../spells/charm_monster.md> "dnd2024:spells/charm_monster") | ` 4 ` | ` Enchantment ` | false | false | false |
| [Confusion](<../spells/confusion.md> "dnd2024:spells/confusion") | ` 4 ` | ` Enchantment ` | true | false | false |
| [Conjure Minor Elementals](<../spells/conjure_minor_elementals.md> "dnd2024:spells/conjure_minor_elementals") | ` 4 ` | ` Conjuration ` | true | false | false |
| [Conjure Woodland Beings](<../spells/conjure_woodland_beings.md> "dnd2024:spells/conjure_woodland_beings") | ` 4 ` | ` Conjuration ` | true | false | false |
| [Control Water](<../spells/control_water.md> "dnd2024:spells/control_water") | ` 4 ` | ` Transmutation ` | true | false | false |
| [Divination](<../spells/divination.md> "dnd2024:spells/divination") | ` 4 ` | ` Divination ` | false | true | true |
| [Dominate Beast](<../spells/dominate_beast.md> "dnd2024:spells/dominate_beast") | ` 4 ` | ` Enchantment ` | true | false | false |
| [Fire Shield](<../spells/fire_shield.md> "dnd2024:spells/fire_shield") | ` 4 ` | ` Evocation ` | false | false | false |
| [Freedom of Movement](<../spells/freedom_of_movement.md> "dnd2024:spells/freedom_of_movement") | ` 4 ` | ` Abjuration ` | false | false | false |
| [Giant Insect](<../spells/giant_insect.md> "dnd2024:spells/giant_insect") | ` 4 ` | ` Conjuration ` | true | false | false |
| [Hallucinatory Terrain](<../spells/hallucinatory_terrain.md> "dnd2024:spells/hallucinatory_terrain") | ` 4 ` | ` Illusion ` | false | false | false |
| [Ice Storm](<../spells/ice_storm.md> "dnd2024:spells/ice_storm") | ` 4 ` | ` Evocation ` | false | false | false |
| [Locate Creature](<../spells/locate_creature.md> "dnd2024:spells/locate_creature") | ` 4 ` | ` Divination ` | true | false | false |
| [Polymorph](<../spells/polymorph.md> "dnd2024:spells/polymorph") | ` 4 ` | ` Transmutation ` | true | false | false |
| [Stone Shape](<../spells/stone_shape.md> "dnd2024:spells/stone_shape") | ` 4 ` | ` Transmutation ` | false | false | false |
| [Stoneskin](<../spells/stoneskin.md> "dnd2024:spells/stoneskin") | ` 4 ` | ` Transmutation ` | true | false | true |
| [Wall of Fire](<../spells/wall_of_fire.md> "dnd2024:spells/wall_of_fire") | ` 4 ` | ` Evocation ` | true | false | false |
| [Antilife Shell](<../spells/antilife_shell.md> "dnd2024:spells/antilife_shell") | ` 5 ` | ` Abjuration ` | true | false | false |
| [Awaken](<../spells/awaken.md> "dnd2024:spells/awaken") | ` 5 ` | ` Transmutation ` | false | false | true |
| [Commune with Nature](<../spells/commune_with_nature.md> "dnd2024:spells/commune_with_nature") | ` 5 ` | ` Divination ` | false | true | false |
| [Cone of Cold](<../spells/cone_of_cold.md> "dnd2024:spells/cone_of_cold") | ` 5 ` | ` Evocation ` | false | false | false |
| [Conjure Elemental](<../spells/conjure_elemental.md> "dnd2024:spells/conjure_elemental") | ` 5 ` | ` Conjuration ` | true | false | false |
| [Contagion](<../spells/contagion.md> "dnd2024:spells/contagion") | ` 5 ` | ` Necromancy ` | false | false | false |
| [Geas](<../spells/geas.md> "dnd2024:spells/geas") | ` 5 ` | ` Enchantment ` | false | false | false |
| [Greater Restoration](<../spells/greater_restoration.md> "dnd2024:spells/greater_restoration") | ` 5 ` | ` Abjuration ` | false | false | true |
| [Insect Plague](<../spells/insect_plague.md> "dnd2024:spells/insect_plague") | ` 5 ` | ` Conjuration ` | true | false | false |
| [Mass Cure Wounds](<../spells/mass_cure_wounds.md> "dnd2024:spells/mass_cure_wounds") | ` 5 ` | ` Abjuration ` | false | false | false |
| [Planar Binding](<../spells/planar_binding.md> "dnd2024:spells/planar_binding") | ` 5 ` | ` Abjuration ` | false | false | true |
| [Reincarnate](<../spells/reincarnate.md> "dnd2024:spells/reincarnate") | ` 5 ` | ` Necromancy ` | false | false | true |
| [Scrying](<../spells/scrying.md> "dnd2024:spells/scrying") | ` 5 ` | ` Divination ` | true | false | true |
| [Tree Stride](<../spells/tree_stride.md> "dnd2024:spells/tree_stride") | ` 5 ` | ` Conjuration ` | true | false | false |
| [Wall of Stone](<../spells/wall_of_stone.md> "dnd2024:spells/wall_of_stone") | ` 5 ` | ` Evocation ` | true | false | false |
| [Conjure Fey](<../spells/conjure_fey.md> "dnd2024:spells/conjure_fey") | ` 6 ` | ` Conjuration ` | true | false | false |
| [Find the Path](<../spells/find_the_path.md> "dnd2024:spells/find_the_path") | ` 6 ` | ` Divination ` | true | false | true |
| [Flesh to Stone](<../spells/flesh_to_stone.md> "dnd2024:spells/flesh_to_stone") | ` 6 ` | ` Transmutation ` | true | false | false |
| [Heal](<../spells/heal.md> "dnd2024:spells/heal") | ` 6 ` | ` Abjuration ` | false | false | false |
| [Heroes’ Feast](<../spells/heroes_feast.md> "dnd2024:spells/heroes_feast") | ` 6 ` | ` Conjuration ` | false | false | true |
| [Move Earth](<../spells/move_earth.md> "dnd2024:spells/move_earth") | ` 6 ` | ` Transmutation ` | true | false | false |
| [Sunbeam](<../spells/sunbeam.md> "dnd2024:spells/sunbeam") | ` 6 ` | ` Evocation ` | true | false | false |
| [Transport via Plants](<../spells/transport_via_plants.md> "dnd2024:spells/transport_via_plants") | ` 6 ` | ` Conjuration ` | false | false | false |
| [Wall of Thorns](<../spells/wall_of_thorns.md> "dnd2024:spells/wall_of_thorns") | ` 6 ` | ` Conjuration ` | true | false | false |
| [Wind Walk](<../spells/wind_walk.md> "dnd2024:spells/wind_walk") | ` 6 ` | ` Transmutation ` | false | false | false |
| [Fire Storm](<../spells/fire_storm.md> "dnd2024:spells/fire_storm") | ` 7 ` | ` Evocation ` | false | false | false |
| [Mirage Arcane](<../spells/mirage_arcane.md> "dnd2024:spells/mirage_arcane") | ` 7 ` | ` Illusion ` | false | false | false |
| [Plane Shift](<../spells/plane_shift.md> "dnd2024:spells/plane_shift") | ` 7 ` | ` Conjuration ` | false | false | true |
| [Regenerate](<../spells/regenerate.md> "dnd2024:spells/regenerate") | ` 7 ` | ` Transmutation ` | false | false | false |
| [Reverse Gravity](<../spells/reverse_gravity.md> "dnd2024:spells/reverse_gravity") | ` 7 ` | ` Transmutation ` | true | false | false |
| [Symbol](<../spells/symbol.md> "dnd2024:spells/symbol") | ` 7 ` | ` Abjuration ` | false | false | true |
| [Animal Shapes](<../spells/animal_shapes.md> "dnd2024:spells/animal_shapes") | ` 8 ` | ` Transmutation ` | false | false | false |
| [Antipathy/Sympathy](<../spells/antipathy_sympathy.md> "dnd2024:spells/antipathy_sympathy") | ` 8 ` | ` Enchantment ` | false | false | false |
| [Befuddlement](<../spells/befuddlement.md> "dnd2024:spells/befuddlement") | ` 8 ` | ` Enchantment ` | false | false | false |
| [Control Weather](<../spells/control_weather.md> "dnd2024:spells/control_weather") | ` 8 ` | ` Transmutation ` | true | false | false |
| [Earthquake](<../spells/earthquake.md> "dnd2024:spells/earthquake") | ` 8 ` | ` Transmutation ` | true | false | false |
| [Incendiary Cloud](<../spells/incendiary_cloud.md> "dnd2024:spells/incendiary_cloud") | ` 8 ` | ` Conjuration ` | true | false | false |
| [Sunburst](<../spells/sunburst.md> "dnd2024:spells/sunburst") | ` 8 ` | ` Evocation ` | false | false | false |
| [Tsunami](<../spells/tsunami.md> "dnd2024:spells/tsunami") | ` 8 ` | ` Conjuration ` | true | false | false |
| [Foresight](<../spells/foresight.md> "dnd2024:spells/foresight") | ` 9 ` | ` Divination ` | false | false | false |
| [Shapechange](<../spells/shapechange.md> "dnd2024:spells/shapechange") | ` 9 ` | ` Transmutation ` | true | false | true |
| [Storm of Vengeance](<../spells/storm_of_vengeance.md> "dnd2024:spells/storm_of_vengeance") | ` 9 ` | ` Conjuration ` | true | false | false |
| [True Resurrection](<../spells/true_resurrection.md> "dnd2024:spells/true_resurrection") | ` 9 ` | ` Necromancy ` | false | false | true |
