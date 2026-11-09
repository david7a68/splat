---
splat_schema: 2
splat_id: cc8b21ff-926e-5637-9552-80b7ff746200
splat_kind: spell
splat_key: dnd2024:spells/silent_image
splat_name: Silent Image
splat_origin: dnd2024:reference/spell/silent_image
splat_sources:
- SRD 5.2.1, pp. 162, https://www.dndbeyond.com/srd
---

# Silent Image


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Illusion |
| Classes | Bard, Sorcerer, Wizard |
| Casting Time | Action |
| Range | 60 feet |
| Components | V, S, M (a bit of fleece) |
| Duration | Concentration, up to 10 minutes |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 162 -->

Level 1 Illusion (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a bit of fleece)\
Duration: Concentration, up to 10 minutes

You create the image of an object, a creature, or some other visible phenomenon that is no larger than a 15-foot Cube. The image appears at a spot within range and lasts for the duration. The image is purely visual; it isn’t accompanied by sound, smell, or other sensory effects.

As a Magic action, you can cause the image to move to any spot within range. As the image changes location, you can alter its appearance so that its movements appear natural for the image. For example, if you create an image of a creature and move it, you can alter the image so that it appears to be walking.

Physical interaction with the image reveals it to be an illusion, since things can pass through it. A creature that takes a Study action to examine the image can determine that it is an illusion with a successful Intelligence (Investigation) check against your spell save DC. If a creature discerns the illusion for what it is, the creature can see through the image.

Original layout: [[../../srd-5.2.1/pages/162|page 162]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/silent_image ` |
| ` implementation ` | ` dnd2024:spell_silent_image ` |

### Parameters

#### Image

<!-- splat:values path=splat_rule/parameters/image -->

| Field | Value |
| --- | --- |
| ` kind_choice ` | ` ["object", "creature", "visible_phenomenon"] ` |
| ` maximum_cube_feet ` | ` 15 ` |
| ` senses ` | ` ["vision"] ` |
| ` sound ` | ` false ` |
| ` smell ` | ` false ` |
| ` other_sensory_effects ` | ` false ` |

#### Move

<!-- splat:values path=splat_rule/parameters/move -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` destination ` | ` any_point_in_range ` |
| ` appearance_alteration ` | ` natural_movement ` |

#### Discern

<!-- splat:yaml path=splat_rule/parameters/discern -->

```yaml
physical_interaction: automatic
study:
  ability: intelligence
  skill: investigation
  dc:
    binding: save_dc
after_discern: see_through
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

