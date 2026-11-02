---
splat_schema: 1
splat_id: 51b5ed68-3c2c-5058-bcfe-8df2339c6679
splat_kind: spell
splat_key: dnd2024:spells/major_image
splat_name: Major Image
splat_origin:
  key: dnd2024:reference/spell/major_image
splat_sources:
- title: SRD 5.2.1
  locator: pp. 147
  url: https://www.dndbeyond.com/srd
---

# Major Image

## Source text

<!-- SRD page 147 -->

Level 3 Illusion (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a bit of fleece)\
Duration: Concentration, up to 10 minutes

You create the image of an object, a creature, or some other visible phenomenon that is no larger than a 20-foot Cube. The image appears at a spot that you can see within range and lasts for the duration. It seems real, including sounds, smells, and temperature appropriate to the thing depicted, but it can’t deal damage or cause conditions.

If you are within range of the illusion, you can take a Magic action to cause the image to move to any other spot within range. As the image changes location, you can alter its appearance so that its movements appear natural for the image. For example, if you create an image of a creature and move it, you can alter the image so that it appears to be walking. Similarly, you can cause the illusion to make different sounds at different times, even making it carry on a conversation, for example.

Physical interaction with the image reveals it to be an illusion, for things can pass through it. A creature that takes a Study action to examine the image can determine that it is an illusion with a successful Intelligence (Investigation) check against your spell save DC. If a creature discerns the illusion for what it is, the creature can see through the image, and its other sensory qualities become faint to the creature.

Using a Higher-Level Spell Slot. The spell lasts until dispelled, without requiring Concentration, if cast with a level 4+ spell slot.

Original layout: [[../../srd-5.2.1/pages/147|page 147]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` illusion ` |
| ` classes ` | ` ["bard", "sorcerer", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 10 minutes ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 120 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a bit of fleece ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/major_image ` |
| ` implementation ` | ` dnd2024:spell_major_image ` |

### Parameters

#### Illusion

<!-- splat:yaml path=splat_rule/parameters/illusion -->

```yaml
depiction_choices:
- object
- creature
- visible_phenomenon
maximum_cube_side_feet: 20
location:
  visible: true
  within_spell_range: true
senses:
- visual
- sound
- smell
- temperature
sensory_qualities: appropriate_to_depiction
can_deal_damage: false
can_cause_conditions: false
```

#### Manipulation

<!-- splat:values path=splat_rule/parameters/manipulation -->

| Field | Value |
| --- | --- |
| ` requires_caster_within_spell_range_of_illusion ` | ` true ` |
| ` cost ` | ` magic_action ` |
| ` move_to ` | ` spot_within_spell_range ` |
| ` alter_appearance_for_natural_movement ` | ` true ` |
| ` vary_sound_over_time ` | ` true ` |
| ` conversation ` | ` true ` |

#### Discern

<!-- splat:yaml path=splat_rule/parameters/discern -->

```yaml
physical_interaction:
  automatic: true
  objects_pass_through: true
study:
  cost: study_action
  ability: intelligence
  skill: investigation
  dc:
    binding: save_dc
on_discern:
  see_through_image: true
  other_sensory_qualities: faint
```

#### Duration by slot

<!-- splat:yaml path=splat_rule/parameters/duration_by_slot -->

```yaml
'3':
  minutes: 10
  concentration: true
4_plus:
  until: dispelled
  concentration: false
```
