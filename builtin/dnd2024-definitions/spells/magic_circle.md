---
splat_schema: 1
splat_id: 9162702f-cef3-5a07-bcea-28e0ad7971ad
splat_kind: spell
splat_key: dnd2024:spells/magic_circle
splat_name: Magic Circle
splat_origin:
  key: dnd2024:reference/spell/magic_circle
splat_sources:
- title: SRD 5.2.1
  locator: pp. 145, 146
  url: https://www.dndbeyond.com/srd
---

# Magic Circle

## Source text

<!-- SRD page 145 -->

Level 3 Abjuration (Cleric, Paladin, Warlock, Wizard)\
Casting Time: 1 minute\
Range: 10 feet\
Components: V, S, M (salt and powdered silver worth\
100+ GP, which the spell consumes)\
Duration: 1 hour\
You create a 10-foot-radius, 20-foot-tall Cylinder of\
magical energy centered on a point on the ground\
that you can see within range. Glowing runes ap-\
pear wherever the Cylinder intersects with the floor\
or other surface.\
Choose one or more of the following types of crea-\
tures: Celestials, Elementals, Fey, Fiends, or Undead.\
The circle affects a creature of the chosen type in\
the following ways:\
•  The creature can’t willingly enter the Cylinder\
by nonmagical means. If the creature tries to use\
teleportation or interplanar travel to do so, it\
must first succeed on a Charisma saving throw.\
•  The creature has Disadvantage on attack rolls\
against targets within the Cylinder.\

<!-- SRD page 146 -->

•  Targets within the Cylinder can’t be possessed\
by or gain the Charmed or Frightened condition\
from the creature.\
Each time you cast this spell, you can cause its\
magic to operate in the reverse direction, prevent-\
ing a creature of the specified type from leaving the\
Cylinder and protecting targets outside it.\
Using a Higher-Level Spell Slot. The duration in-\
creases by 1 hour for each spell slot level above 3.\

Original layout: [[../../srd-5.2.1/pages/145|page 145]], [[../../srd-5.2.1/pages/146|page 146]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["cleric", "paladin", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 hour ` |
| ` casting_time ` | ` 1 minute ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 10 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` salt and powdered silver worth 100+ GP, which the spell consumes ` |
| ` consumed ` | ` true ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/magic_circle ` |
| ` implementation ` | ` dnd2024:spell_magic_circle ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` mode_choice ` | ` ["normal", "reversed"] ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cylinder ` |
| ` radius_feet ` | ` 10 ` |
| ` height_feet ` | ` 20 ` |
| ` origin ` | ` visible_ground_point ` |
| ` glowing_runes_on_surface_intersections ` | ` true ` |

#### Type choice

<!-- splat:values path=splat_rule/parameters/type_choice -->

| Field | Value |
| --- | --- |
| ` minimum ` | ` 1 ` |
| ` options ` | ` ["celestial", "elemental", "fey", "fiend", "undead"] ` |

#### Normal

<!-- splat:yaml path=splat_rule/parameters/normal -->

```yaml
nonmagical_willing_entry: false
teleport_or_interplanar_entry:
  save_ability: charisma
  success_required: true
attack_rolls_against_inside: disadvantage
protect_inside_against:
- possession
- charmed
- frightened
```

#### Reversed

<!-- splat:values path=splat_rule/parameters/reversed -->

| Field | Value |
| --- | --- |
| ` restricted_direction ` | ` exit ` |
| ` protected_targets ` | ` outside ` |

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 3 ` |
| ` duration_hours ` | ` 1 ` |
