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

<!-- splat:table
path: splat_rule/parameters/spells
key: Spell
order:
- chill_touch
- eldritch_blast
- mage_hand
- minor_illusion
- poison_spray
- prestidigitation
- true_strike
- bane
- charm_person
- comprehend_languages
- detect_magic
- expeditious_retreat
- hellish_rebuke
- hex
- hideous_laughter
- illusory_script
- protection_from_evil_and_good
- speak_with_animals
- unseen_servant
- darkness
- enthrall
- hold_person
- invisibility
- mind_spike
- mirror_image
- misty_step
- ray_of_enfeeblement
- spider_climb
- suggestion
- counterspell
- dispel_magic
- fear
- fly
- gaseous_form
- hypnotic_pattern
- magic_circle
- major_image
- remove_curse
- tongues
- vampiric_touch
- banishment
- blight
- charm_monster
- dimension_door
- hallucinatory_terrain
- contact_other_plane
- dream
- hold_monster
- mislead
- planar_binding
- scrying
- teleportation_circle
- circle_of_death
- create_undead
- eyebite
- true_seeing
- etherealness
- finger_of_death
- forcecage
- plane_shift
- befuddlement
- demiplane
- dominate_monster
- glibness
- power_word_stun
- astral_projection
- foresight
- gate
- imprisonment
- power_word_kill
- true_polymorph
- weird
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
| [Chill Touch](<../spells/chill_touch.md> "dnd2024:spells/chill_touch") | ` 0 ` | ` Necromancy ` | false | false | false |
| [Eldritch Blast](<../spells/eldritch_blast.md> "dnd2024:spells/eldritch_blast") | ` 0 ` | ` Evocation ` | false | false | false |
| [Mage Hand](<../spells/mage_hand.md> "dnd2024:spells/mage_hand") | ` 0 ` | ` Conjuration ` | false | false | false |
| [Minor Illusion](<../spells/minor_illusion.md> "dnd2024:spells/minor_illusion") | ` 0 ` | ` Illusion ` | false | false | false |
| [Poison Spray](<../spells/poison_spray.md> "dnd2024:spells/poison_spray") | ` 0 ` | ` Necromancy ` | false | false | false |
| [Prestidigitation](<../spells/prestidigitation.md> "dnd2024:spells/prestidigitation") | ` 0 ` | ` Transmutation ` | false | false | false |
| [True Strike](<../spells/true_strike.md> "dnd2024:spells/true_strike") | ` 0 ` | ` Divination ` | false | false | false |
| [Bane](<../spells/bane.md> "dnd2024:spells/bane") | ` 1 ` | ` Enchantment ` | true | false | false |
| [Charm Person](<../spells/charm_person.md> "dnd2024:spells/charm_person") | ` 1 ` | ` Enchantment ` | false | false | false |
| [Comprehend Languages](<../spells/comprehend_languages.md> "dnd2024:spells/comprehend_languages") | ` 1 ` | ` Divination ` | false | true | false |
| [Detect Magic](<../spells/detect_magic.md> "dnd2024:spells/detect_magic") | ` 1 ` | ` Divination ` | true | true | false |
| [Expeditious Retreat](<../spells/expeditious_retreat.md> "dnd2024:spells/expeditious_retreat") | ` 1 ` | ` Transmutation ` | true | false | false |
| [Hellish Rebuke](<../spells/hellish_rebuke.md> "dnd2024:spells/hellish_rebuke") | ` 1 ` | ` Evocation ` | false | false | false |
| [Hex](<../spells/hex.md> "dnd2024:spells/hex") | ` 1 ` | ` Enchantment ` | true | false | false |
| [Hideous Laughter](<../spells/hideous_laughter.md> "dnd2024:spells/hideous_laughter") | ` 1 ` | ` Enchantment ` | true | false | false |
| [Illusory Script](<../spells/illusory_script.md> "dnd2024:spells/illusory_script") | ` 1 ` | ` Illusion ` | false | true | true |
| [Protection from Evil and Good](<../spells/protection_from_evil_and_good.md> "dnd2024:spells/protection_from_evil_and_good") | ` 1 ` | ` Abjuration ` | true | false | true |
| [Speak with Animals](<../spells/speak_with_animals.md> "dnd2024:spells/speak_with_animals") | ` 1 ` | ` Divination ` | false | true | false |
| [Unseen Servant](<../spells/unseen_servant.md> "dnd2024:spells/unseen_servant") | ` 1 ` | ` Conjuration ` | false | true | false |
| [Darkness](<../spells/darkness.md> "dnd2024:spells/darkness") | ` 2 ` | ` Evocation ` | true | false | false |
| [Enthrall](<../spells/enthrall.md> "dnd2024:spells/enthrall") | ` 2 ` | ` Enchantment ` | true | false | false |
| [Hold Person](<../spells/hold_person.md> "dnd2024:spells/hold_person") | ` 2 ` | ` Enchantment ` | true | false | false |
| [Invisibility](<../spells/invisibility.md> "dnd2024:spells/invisibility") | ` 2 ` | ` Illusion ` | true | false | false |
| [Mind Spike](<../spells/mind_spike.md> "dnd2024:spells/mind_spike") | ` 2 ` | ` Divination ` | true | false | false |
| [Mirror Image](<../spells/mirror_image.md> "dnd2024:spells/mirror_image") | ` 2 ` | ` Illusion ` | false | false | false |
| [Misty Step](<../spells/misty_step.md> "dnd2024:spells/misty_step") | ` 2 ` | ` Conjuration ` | false | false | false |
| [Ray of Enfeeblement](<../spells/ray_of_enfeeblement.md> "dnd2024:spells/ray_of_enfeeblement") | ` 2 ` | ` Necromancy ` | true | false | false |
| [Spider Climb](<../spells/spider_climb.md> "dnd2024:spells/spider_climb") | ` 2 ` | ` Transmutation ` | true | false | false |
| [Suggestion](<../spells/suggestion.md> "dnd2024:spells/suggestion") | ` 2 ` | ` Enchantment ` | true | false | false |
| [Counterspell](<../spells/counterspell.md> "dnd2024:spells/counterspell") | ` 3 ` | ` Abjuration ` | false | false | false |
| [Dispel Magic](<../spells/dispel_magic.md> "dnd2024:spells/dispel_magic") | ` 3 ` | ` Abjuration ` | false | false | false |
| [Fear](<../spells/fear.md> "dnd2024:spells/fear") | ` 3 ` | ` Illusion ` | true | false | false |
| [Fly](<../spells/fly.md> "dnd2024:spells/fly") | ` 3 ` | ` Transmutation ` | true | false | false |
| [Gaseous Form](<../spells/gaseous_form.md> "dnd2024:spells/gaseous_form") | ` 3 ` | ` Transmutation ` | true | false | false |
| [Hypnotic Pattern](<../spells/hypnotic_pattern.md> "dnd2024:spells/hypnotic_pattern") | ` 3 ` | ` Illusion ` | true | false | false |
| [Magic Circle](<../spells/magic_circle.md> "dnd2024:spells/magic_circle") | ` 3 ` | ` Abjuration ` | false | false | true |
| [Major Image](<../spells/major_image.md> "dnd2024:spells/major_image") | ` 3 ` | ` Illusion ` | true | false | false |
| [Remove Curse](<../spells/remove_curse.md> "dnd2024:spells/remove_curse") | ` 3 ` | ` Abjuration ` | false | false | false |
| [Tongues](<../spells/tongues.md> "dnd2024:spells/tongues") | ` 3 ` | ` Divination ` | false | false | false |
| [Vampiric Touch](<../spells/vampiric_touch.md> "dnd2024:spells/vampiric_touch") | ` 3 ` | ` Necromancy ` | true | false | false |
| [Banishment](<../spells/banishment.md> "dnd2024:spells/banishment") | ` 4 ` | ` Abjuration ` | true | false | false |
| [Blight](<../spells/blight.md> "dnd2024:spells/blight") | ` 4 ` | ` Necromancy ` | false | false | false |
| [Charm Monster](<../spells/charm_monster.md> "dnd2024:spells/charm_monster") | ` 4 ` | ` Enchantment ` | false | false | false |
| [Dimension Door](<../spells/dimension_door.md> "dnd2024:spells/dimension_door") | ` 4 ` | ` Conjuration ` | false | false | false |
| [Hallucinatory Terrain](<../spells/hallucinatory_terrain.md> "dnd2024:spells/hallucinatory_terrain") | ` 4 ` | ` Illusion ` | false | false | false |
| [Contact Other Plane](<../spells/contact_other_plane.md> "dnd2024:spells/contact_other_plane") | ` 5 ` | ` Divination ` | false | true | false |
| [Dream](<../spells/dream.md> "dnd2024:spells/dream") | ` 5 ` | ` Illusion ` | false | false | false |
| [Hold Monster](<../spells/hold_monster.md> "dnd2024:spells/hold_monster") | ` 5 ` | ` Enchantment ` | true | false | false |
| [Mislead](<../spells/mislead.md> "dnd2024:spells/mislead") | ` 5 ` | ` Illusion ` | true | false | false |
| [Planar Binding](<../spells/planar_binding.md> "dnd2024:spells/planar_binding") | ` 5 ` | ` Abjuration ` | false | false | true |
| [Scrying](<../spells/scrying.md> "dnd2024:spells/scrying") | ` 5 ` | ` Divination ` | true | false | true |
| [Teleportation Circle](<../spells/teleportation_circle.md> "dnd2024:spells/teleportation_circle") | ` 5 ` | ` Conjuration ` | false | false | true |
| [Circle of Death](<../spells/circle_of_death.md> "dnd2024:spells/circle_of_death") | ` 6 ` | ` Necromancy ` | false | false | true |
| [Create Undead](<../spells/create_undead.md> "dnd2024:spells/create_undead") | ` 6 ` | ` Necromancy ` | false | false | true |
| [Eyebite](<../spells/eyebite.md> "dnd2024:spells/eyebite") | ` 6 ` | ` Necromancy ` | true | false | false |
| [True Seeing](<../spells/true_seeing.md> "dnd2024:spells/true_seeing") | ` 6 ` | ` Divination ` | false | false | true |
| [Etherealness](<../spells/etherealness.md> "dnd2024:spells/etherealness") | ` 7 ` | ` Conjuration ` | false | false | false |
| [Finger of Death](<../spells/finger_of_death.md> "dnd2024:spells/finger_of_death") | ` 7 ` | ` Necromancy ` | false | false | false |
| [Forcecage](<../spells/forcecage.md> "dnd2024:spells/forcecage") | ` 7 ` | ` Evocation ` | true | false | true |
| [Plane Shift](<../spells/plane_shift.md> "dnd2024:spells/plane_shift") | ` 7 ` | ` Conjuration ` | false | false | true |
| [Befuddlement](<../spells/befuddlement.md> "dnd2024:spells/befuddlement") | ` 8 ` | ` Enchantment ` | false | false | false |
| [Demiplane](<../spells/demiplane.md> "dnd2024:spells/demiplane") | ` 8 ` | ` Conjuration ` | false | false | false |
| [Dominate Monster](<../spells/dominate_monster.md> "dnd2024:spells/dominate_monster") | ` 8 ` | ` Enchantment ` | true | false | false |
| [Glibness](<../spells/glibness.md> "dnd2024:spells/glibness") | ` 8 ` | ` Enchantment ` | false | false | false |
| [Power Word Stun](<../spells/power_word_stun.md> "dnd2024:spells/power_word_stun") | ` 8 ` | ` Enchantment ` | false | false | false |
| [Astral Projection](<../spells/astral_projection.md> "dnd2024:spells/astral_projection") | ` 9 ` | ` Necromancy ` | false | false | true |
| [Foresight](<../spells/foresight.md> "dnd2024:spells/foresight") | ` 9 ` | ` Divination ` | false | false | false |
| [Gate](<../spells/gate.md> "dnd2024:spells/gate") | ` 9 ` | ` Conjuration ` | true | false | true |
| [Imprisonment](<../spells/imprisonment.md> "dnd2024:spells/imprisonment") | ` 9 ` | ` Abjuration ` | false | false | true |
| [Power Word Kill](<../spells/power_word_kill.md> "dnd2024:spells/power_word_kill") | ` 9 ` | ` Enchantment ` | false | false | false |
| [True Polymorph](<../spells/true_polymorph.md> "dnd2024:spells/true_polymorph") | ` 9 ` | ` Transmutation ` | true | false | false |
| [Weird](<../spells/weird.md> "dnd2024:spells/weird") | ` 9 ` | ` Illusion ` | true | false | false |
