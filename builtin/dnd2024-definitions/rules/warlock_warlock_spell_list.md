---
splat_schema: 1
splat_id: d0845196-22dd-5389-8fdd-9aefe4307c38
splat_kind: rule
splat_key: dnd2024:rules/warlock_warlock_spell_list
splat_name: Warlock Spell List
splat_origin:
  key: dnd2024:reference/pages/074
splat_sources:
- title: SRD 5.2.1
  locator: pp. 74, 75, 76
  url: https://www.dndbeyond.com/srd
---

# Warlock Spell List

## Source text

<!-- SRD page 74 -->

This section presents the Warlock spell list. The\
spells are organized by spell level and then alpha-\
betized, and each spell’s school of magic is listed.\
In the Special column, C means the spell requires\
Concentration, R means it’s a Ritual, and M means it\
requires a specific Material component.\

<!-- SRD page 75 -->

Cantrips (Level 0 Warlock Spells)\
Spell School Special\
Chill Touch Necromancy —\
Eldritch Blast Evocation —\
Mage Hand Conjuration —\
Minor Illusion Illusion —\
Poison Spray Necromancy —\
Prestidigitation Transmutation —\
True Strike Divination —\
Level 1 Warlock Spells\
Spell School Special\
Bane Enchantment C\
Charm Person Enchantment —\
Comprehend Languages Divination R\
Detect Magic Divination C, R\
Expeditious Retreat Transmutation C\
Hellish Rebuke Evocation —\
Hex Enchantment C\
Hideous Laughter Enchantment C\
Illusory Script Illusion R, M\
Protection from Evil and Good Abjuration C, M\
Speak with Animals Divination R\
Unseen Servant Conjuration R\
Level 2 Warlock Spells\
Spell School Special\
Darkness Evocation C\
Enthrall Enchantment C\
Hold Person Enchantment C\
Invisibility Illusion C\
Mind Spike Divination C\
Mirror Image Illusion —\
Misty Step Conjuration —\
Ray of Enfeeblement Necromancy C\
Spider Climb Transmutation C\
Suggestion Enchantment C\
Level 3 Warlock Spells\
Spell School Special\
Counterspell Abjuration —\
Dispel Magic Abjuration —\
Fear Illusion C\
Fly Transmutation C\
Spell School Special\
Gaseous Form Transmutation C\
Hypnotic Pattern Illusion C\
Magic Circle Abjuration M\
Major Image Illusion C\
Remove Curse Abjuration —\
Tongues Divination —\
Vampiric Touch Necromancy C\
Level 4 Warlock Spells\
Spell School Special\
Banishment Abjuration C\
Blight Necromancy —\
Charm Monster Enchantment —\
Dimension Door Conjuration —\
Hallucinatory Terrain Illusion —\
Level 5 Warlock Spells\
Spell School Special\
Contact Other Plane Divination R\
Dream Illusion —\
Hold Monster Enchantment C\
Mislead Illusion C\
Planar Binding Abjuration M\
Scrying Divination C, M\
Teleportation Circle Conjuration M\
Level 6 Warlock Spells\
Spell School Special\
Circle of Death Necromancy M\
Create Undead Necromancy M\
Eyebite Necromancy C\
True Seeing Divination M\
Level 7 Warlock Spells\
Spell School Special\
Etherealness Conjuration —\
Finger of Death Necromancy —\
Forcecage Evocation C, M\
Plane Shift Conjuration M\

<!-- SRD page 76 -->

Level 8 Warlock Spells\
Spell School Special\
Befuddlement Enchantment —\
Demiplane Conjuration —\
Dominate Monster Enchantment C\
Glibness Enchantment —\
Power Word Stun Enchantment —\
Level 9 Warlock Spells\
Spell School Special\
Astral Projection Necromancy M\
Foresight Divination —\
Gate Conjuration C, M\
Imprisonment Abjuration M\
Power Word Kill Enchantment —\
True Polymorph Transmutation C\
Weird Illusion C\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_warlock_spell_list ` |
| ` implementation ` | ` dnd2024:rules_warlock_warlock_spell_list ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` sorting ` | ` ["spell_level", "name"] ` |

#### Class

<!-- splat:values path=splat_rule/parameters/class -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/warlock ` |

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | level | school | concentration | ritual | specific_material |
| --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 0 ` | ` necromancy ` | ` false ` | ` false ` | ` false ` |
| ` 1 ` | ` 0 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 2 ` | ` 0 ` | ` conjuration ` | ` false ` | ` false ` | ` false ` |
| ` 3 ` | ` 0 ` | ` illusion ` | ` false ` | ` false ` | ` false ` |
| ` 4 ` | ` 0 ` | ` necromancy ` | ` false ` | ` false ` | ` false ` |
| ` 5 ` | ` 0 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 6 ` | ` 0 ` | ` divination ` | ` false ` | ` false ` | ` false ` |
| ` 7 ` | ` 1 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 8 ` | ` 1 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 9 ` | ` 1 ` | ` divination ` | ` false ` | ` true ` | ` false ` |
| ` 10 ` | ` 1 ` | ` divination ` | ` true ` | ` true ` | ` false ` |
| ` 11 ` | ` 1 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 12 ` | ` 1 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 13 ` | ` 1 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 14 ` | ` 1 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 15 ` | ` 1 ` | ` illusion ` | ` false ` | ` true ` | ` true ` |
| ` 16 ` | ` 1 ` | ` abjuration ` | ` true ` | ` false ` | ` true ` |
| ` 17 ` | ` 1 ` | ` divination ` | ` false ` | ` true ` | ` false ` |
| ` 18 ` | ` 1 ` | ` conjuration ` | ` false ` | ` true ` | ` false ` |
| ` 19 ` | ` 2 ` | ` evocation ` | ` true ` | ` false ` | ` false ` |
| ` 20 ` | ` 2 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 21 ` | ` 2 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 22 ` | ` 2 ` | ` illusion ` | ` true ` | ` false ` | ` false ` |
| ` 23 ` | ` 2 ` | ` divination ` | ` true ` | ` false ` | ` false ` |
| ` 24 ` | ` 2 ` | ` illusion ` | ` false ` | ` false ` | ` false ` |
| ` 25 ` | ` 2 ` | ` conjuration ` | ` false ` | ` false ` | ` false ` |
| ` 26 ` | ` 2 ` | ` necromancy ` | ` true ` | ` false ` | ` false ` |
| ` 27 ` | ` 2 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 28 ` | ` 2 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 29 ` | ` 3 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 30 ` | ` 3 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 31 ` | ` 3 ` | ` illusion ` | ` true ` | ` false ` | ` false ` |
| ` 32 ` | ` 3 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 33 ` | ` 3 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 34 ` | ` 3 ` | ` illusion ` | ` true ` | ` false ` | ` false ` |
| ` 35 ` | ` 3 ` | ` abjuration ` | ` false ` | ` false ` | ` true ` |
| ` 36 ` | ` 3 ` | ` illusion ` | ` true ` | ` false ` | ` false ` |
| ` 37 ` | ` 3 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 38 ` | ` 3 ` | ` divination ` | ` false ` | ` false ` | ` false ` |
| ` 39 ` | ` 3 ` | ` necromancy ` | ` true ` | ` false ` | ` false ` |
| ` 40 ` | ` 4 ` | ` abjuration ` | ` true ` | ` false ` | ` false ` |
| ` 41 ` | ` 4 ` | ` necromancy ` | ` false ` | ` false ` | ` false ` |
| ` 42 ` | ` 4 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 43 ` | ` 4 ` | ` conjuration ` | ` false ` | ` false ` | ` false ` |
| ` 44 ` | ` 4 ` | ` illusion ` | ` false ` | ` false ` | ` false ` |
| ` 45 ` | ` 5 ` | ` divination ` | ` false ` | ` true ` | ` false ` |
| ` 46 ` | ` 5 ` | ` illusion ` | ` false ` | ` false ` | ` false ` |
| ` 47 ` | ` 5 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 48 ` | ` 5 ` | ` illusion ` | ` true ` | ` false ` | ` false ` |
| ` 49 ` | ` 5 ` | ` abjuration ` | ` false ` | ` false ` | ` true ` |
| ` 50 ` | ` 5 ` | ` divination ` | ` true ` | ` false ` | ` true ` |
| ` 51 ` | ` 5 ` | ` conjuration ` | ` false ` | ` false ` | ` true ` |
| ` 52 ` | ` 6 ` | ` necromancy ` | ` false ` | ` false ` | ` true ` |
| ` 53 ` | ` 6 ` | ` necromancy ` | ` false ` | ` false ` | ` true ` |
| ` 54 ` | ` 6 ` | ` necromancy ` | ` true ` | ` false ` | ` false ` |
| ` 55 ` | ` 6 ` | ` divination ` | ` false ` | ` false ` | ` true ` |
| ` 56 ` | ` 7 ` | ` conjuration ` | ` false ` | ` false ` | ` false ` |
| ` 57 ` | ` 7 ` | ` necromancy ` | ` false ` | ` false ` | ` false ` |
| ` 58 ` | ` 7 ` | ` evocation ` | ` true ` | ` false ` | ` true ` |
| ` 59 ` | ` 7 ` | ` conjuration ` | ` false ` | ` false ` | ` true ` |
| ` 60 ` | ` 8 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 61 ` | ` 8 ` | ` conjuration ` | ` false ` | ` false ` | ` false ` |
| ` 62 ` | ` 8 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 63 ` | ` 8 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 64 ` | ` 8 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 65 ` | ` 9 ` | ` necromancy ` | ` false ` | ` false ` | ` true ` |
| ` 66 ` | ` 9 ` | ` divination ` | ` false ` | ` false ` | ` false ` |
| ` 67 ` | ` 9 ` | ` conjuration ` | ` true ` | ` false ` | ` true ` |
| ` 68 ` | ` 9 ` | ` abjuration ` | ` false ` | ` false ` | ` true ` |
| ` 69 ` | ` 9 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 70 ` | ` 9 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 71 ` | ` 9 ` | ` illusion ` | ` true ` | ` false ` | ` false ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/chill_touch ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/1/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/eldritch_blast ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/2/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/mage_hand ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/3/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/minor_illusion ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/4/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/poison_spray ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/5/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/prestidigitation ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/6/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/true_strike ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/7/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/bane ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/8/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/charm_person ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/9/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/comprehend_languages ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/10/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/detect_magic ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/11/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/expeditious_retreat ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/12/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/hellish_rebuke ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/13/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/hex ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/14/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/hideous_laughter ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/15/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/illusory_script ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/16/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/protection_from_evil_and_good ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/17/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/speak_with_animals ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/18/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/unseen_servant ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/19/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/darkness ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/20/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/enthrall ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/21/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/hold_person ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/22/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/invisibility ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/23/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/mind_spike ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/24/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/mirror_image ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/25/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/misty_step ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/26/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/ray_of_enfeeblement ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/27/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/spider_climb ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/28/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/suggestion ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/29/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/counterspell ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/30/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/dispel_magic ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/31/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/fear ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/32/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/fly ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/33/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/gaseous_form ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/34/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/hypnotic_pattern ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/35/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/magic_circle ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/36/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/major_image ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/37/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/remove_curse ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/38/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/tongues ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/39/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/vampiric_touch ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/40/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/banishment ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/41/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/blight ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/42/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/charm_monster ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/43/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/dimension_door ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/44/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/hallucinatory_terrain ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/45/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/contact_other_plane ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/46/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/dream ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/47/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/hold_monster ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/48/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/mislead ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/49/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/planar_binding ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/50/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/scrying ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/51/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/teleportation_circle ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/52/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/circle_of_death ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/53/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/create_undead ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/54/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/eyebite ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/55/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/true_seeing ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/56/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/etherealness ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/57/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/finger_of_death ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/58/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/forcecage ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/59/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/plane_shift ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/60/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/befuddlement ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/61/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/demiplane ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/62/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/dominate_monster ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/63/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/glibness ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/64/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/power_word_stun ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/65/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/astral_projection ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/66/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/foresight ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/67/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/gate ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/68/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/imprisonment ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/69/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/power_word_kill ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/70/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/true_polymorph ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/71/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/weird ` |
