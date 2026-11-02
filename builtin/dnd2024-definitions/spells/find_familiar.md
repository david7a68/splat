---
splat_schema: 1
splat_id: d87a20fc-113d-55bc-8803-7c508c15cc5c
splat_kind: spell
splat_key: dnd2024:spells/find_familiar
splat_name: Find Familiar
splat_origin:
  key: dnd2024:reference/spell/find_familiar
splat_sources:
- title: SRD 5.2.1
  locator: pp. 130
  url: https://www.dndbeyond.com/srd
---

# Find Familiar

## Source text

<!-- SRD page 130 -->

Level 1 Conjuration (Wizard)\
Casting Time: 1 hour or Ritual\
Range: 10 feet\
Components: V, S, M (burning incense worth 10+ GP, which the spell consumes)\
Duration: Instantaneous

You gain the service of a familiar, a spirit that takes an animal form you choose: Bat, Cat, Frog, Hawk, Lizard, Octopus, Owl, Rat, Raven, Spider, Weasel, or another Beast that has a Challenge Rating of 0. Appearing in an unoccupied space within range, the familiar has the statistics of the chosen form (see “Monsters”), though it is a Celestial, Fey, or Fiend (your choice) instead of a Beast. Your familiar acts independently of you, but it obeys your commands.

Telepathic Connection. While your familiar is within 100 feet of you, you can communicate with it telepathically. Additionally, as a Bonus Action, you can see through the familiar’s eyes and hear what it hears until the start of your next turn, gaining the benefits of any special senses it has.

Finally, when you cast a spell with a range of touch, your familiar can deliver the touch. Your familiar must be within 100 feet of you, and it must take a Reaction to deliver the touch when you cast the spell.

Combat. The familiar is an ally to you and your allies. It rolls its own Initiative and acts on its own turn. A familiar can’t attack, but it can take other actions as normal.

Disappearance of the Familiar. When the familiar drops to 0 Hit Points, it disappears. It reappears after you cast this spell again. As a Magic action, you can temporarily dismiss the familiar to a pocket dimension. Alternatively, you can dismiss it forever. As a Magic action while it is temporarily dismissed, you can cause it to reappear in an unoccupied space within 30 feet of you. Whenever the familiar drops to 0 Hit Points or disappears into the pocket dimension, it leaves behind in its space anything it was wearing or carrying.

One Familiar Only. You can’t have more than one familiar at a time. If you cast this spell while you have a familiar, you instead cause it to adopt a new eligible form.

Original layout: [[../../srd-5.2.1/pages/130|page 130]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` 1 hour or Ritual ` |
| ` ritual ` | ` true ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 10 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` burning incense worth 10+ GP, which the spell consumes ` |
| ` consumed ` | ` true ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/find_familiar ` |
| ` implementation ` | ` dnd2024:spell_find_familiar ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` location ` |
| ` unoccupied ` | ` true ` |
| ` within_range ` | ` true ` |

#### Form

<!-- splat:yaml path=splat_rule/parameters/form -->

```yaml
chooser: caster
eligible:
  type: beast
  challenge_rating: 0
examples:
- bat
- cat
- frog
- hawk
- lizard
- octopus
- owl
- rat
- raven
- spider
- weasel
statistics: chosen_form
replace_type_choice:
- celestial
- fey
- fiend
```

#### Behavior

<!-- splat:values path=splat_rule/parameters/behavior -->

| Field | Value |
| --- | --- |
| ` independent ` | ` true ` |
| ` obeys_commands ` | ` true ` |
| ` allies ` | ` ["caster", "caster_allies"] ` |
| ` initiative ` | ` own_roll ` |
| ` turn ` | ` own ` |
| ` attack_allowed ` | ` false ` |
| ` other_actions ` | ` normal ` |

#### Connection

<!-- splat:yaml path=splat_rule/parameters/connection -->

```yaml
telepathy_range_feet: 100
shared_senses:
  economy: bonus_action
  duration: until_caster_next_turn_start
  channels:
  - vision
  - hearing
  special_senses: included
```

#### Deliver spell

<!-- splat:values path=splat_rule/parameters/deliver_spell -->

| Field | Value |
| --- | --- |
| ` spell_range ` | ` touch ` |
| ` familiar_distance_to_caster_max_feet ` | ` 100 ` |
| ` familiar_economy ` | ` reaction ` |
| ` timing ` | ` caster_casts_spell ` |
| ` delivery ` | ` familiar_touch ` |

#### Disappearance

<!-- splat:yaml path=splat_rule/parameters/disappearance -->

```yaml
zero_hp: disappears_until_recast
dismissal:
  economy: magic_action
  options:
  - temporary_pocket_dimension
  - permanent
return:
  requires: temporarily_dismissed
  economy: magic_action
  unoccupied: true
  distance_feet: 30
on_zero_hp_or_pocket_dimension: leave_worn_and_carried_items_in_space
```

#### Limit

<!-- splat:values path=splat_rule/parameters/limit -->

| Field | Value |
| --- | --- |
| ` per_caster ` | ` 1 ` |
| ` recast_with_existing ` | ` adopt_new_eligible_form ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```
