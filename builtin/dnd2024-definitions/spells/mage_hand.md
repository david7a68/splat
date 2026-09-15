---
splat_schema: 1
splat_id: 28b158ce-9809-5b95-80a9-304ff6e54c37
splat_kind: spell
splat_key: dnd2024:spells/mage_hand
splat_name: Mage Hand
splat_origin:
  key: dnd2024:reference/spell/mage_hand
splat_sources:
- title: SRD 5.2.1
  locator: pp. 145
  url: https://www.dndbeyond.com/srd
---

# Mage Hand

## Source text

<!-- SRD page 145 -->

Conjuration Cantrip (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: 1 minute\
A spectral, floating hand appears at a point you\
choose within range. The hand lasts for the dura-\
tion. The hand vanishes if it is ever more than 30\
feet away from you or if you cast this spell again.\
When you cast the spell, you can use the hand to\
manipulate an object, open an unlocked door or\
container, stow or retrieve an item from an open\
container, or pour the contents out of a vial.\
As a Magic action on your later turns, you can con-\
trol the hand thus again. As part of that action, you\
can move the hand up to 30 feet.\
The hand can’t attack, activate magic items, or\
carry more than 10 pounds.\

Original layout: [[../../srd-5.2.1/pages/145|page 145]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 0 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["bard", "sorcerer", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 30 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/mage_hand ` |
| ` implementation ` | ` dnd2024:spell_mage_hand ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` end_conditions ` | ` ["distance_from_caster_greater_than_30_feet", "caster_casts_mage_hand_again"] ` |

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` maximum_seconds ` | ` 60 ` |

#### Hand

<!-- splat:values path=splat_rule/parameters/hand -->

| Field | Value |
| --- | --- |
| ` spectral ` | ` true ` |
| ` floating ` | ` true ` |
| ` initial_point ` | ` caster_choice ` |
| ` range_feet ` | ` 30 ` |

#### Manipulation

<!-- splat:values path=splat_rule/parameters/manipulation -->

| Field | Value |
| --- | --- |
| ` at_cast ` | ` true ` |
| ` later_economy ` | ` magic_action ` |
| ` later_timing ` | ` caster_turn ` |
| ` options ` | ` ["manipulate_object", "open_unlocked_door", "open_unlocked_container", "stow_in_open_container", "retrieve_from_open_container", "pour_vial_contents"] ` |

#### Movement

<!-- splat:values path=splat_rule/parameters/movement -->

| Field | Value |
| --- | --- |
| ` as_part_of_later_control_action ` | ` true ` |
| ` distance_feet_max ` | ` 30 ` |

#### Limits

<!-- splat:values path=splat_rule/parameters/limits -->

| Field | Value |
| --- | --- |
| ` attack ` | ` false ` |
| ` activate_magic_item ` | ` false ` |
| ` carried_weight_pounds_max ` | ` 10 ` |
