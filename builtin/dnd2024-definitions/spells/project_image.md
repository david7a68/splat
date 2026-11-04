---
splat_schema: 2
splat_id: 4d7518a5-bc6c-5ea8-9fde-7510551642fe
splat_kind: spell
splat_key: dnd2024:spells/project_image
splat_name: Project Image
splat_origin: dnd2024:reference/spell/project_image
splat_sources:
- SRD 5.2.1, pp. 156, 157, https://www.dndbeyond.com/srd
---

# Project Image


## Spell

| Field | Value |
| --- | --- |
| Level | 7 |
| School | Illusion |
| Classes | Bard, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (a statuette of yourself worth 5+ GP) |
| Duration | Concentration, up to 1 day |


## Source text

<!-- SRD page 156 -->

Level 7 Illusion (Bard, Wizard)\
Casting Time: Action\
Range: 500 miles\
Components: V, S, M (a statuette of yourself worth 5+ GP)\
Duration: Concentration, up to 1 day

You create an illusory copy of yourself that lasts for the duration. The copy can appear at any location within range that you have seen before, regardless of intervening obstacles. The illusion looks and sounds like you, but it is intangible. If the illusion takes any damage, it disappears, and the spell ends.

You can see through the illusion’s eyes and hear through its ears as if you were in its space. As a Magic action, you can move it up to 60 feet and make it gesture, speak, and behave in whatever way you choose. It mimics your mannerisms perfectly.

Physical interaction with the image reveals it to be illusory, since things can pass through it. A creature that takes the Study action to examine the image can determine that it is an illusion with a successful Intelligence (Investigation) check against your spell save DC. If a creature discerns the illusion for what it is, the creature can see through the image, and any noise it makes sounds hollow to the creature.

Original layout: [[../../srd-5.2.1/pages/156|page 156]], [[../../srd-5.2.1/pages/157|page 157]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/project_image ` |
| ` implementation ` | ` dnd2024:spell_project_image ` |

### Parameters

#### Image

<!-- splat:values path=splat_rule/parameters/image -->

| Field | Value |
| --- | --- |
| ` appearance_and_voice ` | ` caster ` |
| ` location ` | ` previously_seen_within_range ` |
| ` obstacles_ignored ` | ` true ` |
| ` intangible ` | ` true ` |
| ` any_damage ` | ` disappear_and_end_spell ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` sight_origin ` | ` image ` |
| ` hearing_origin ` | ` image ` |

#### Control

<!-- splat:values path=splat_rule/parameters/control -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` move_feet ` | ` 60 ` |
| ` choices ` | ` ["gesture", "speech", "behavior"] ` |
| ` perfect_mannerism_mimicry ` | ` true ` |

#### Discern

<!-- splat:yaml path=splat_rule/parameters/discern -->

```yaml
physical_interaction: automatic
study:
  ability: intelligence
  skill: investigation
  dc:
    binding: save_dc
after_discern:
  see_through: true
  sound: hollow
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: 500 miles
> ```

