---
splat_schema: 1
splat_id: 95e3592c-7670-5a21-82ef-7e2a93485251
splat_kind: spell
splat_key: dnd2024:spells/magnificent_mansion
splat_name: Magnificent Mansion
splat_origin:
  key: dnd2024:reference/spell/magnificent_mansion
splat_sources:
- title: SRD 5.2.1
  locator: pp. 147
  url: https://www.dndbeyond.com/srd
---

# Magnificent Mansion

## Source text

<!-- SRD page 147 -->

Level 7 Conjuration (Bard, Wizard)\
Casting Time: 1 minute\
Range: 300 feet\
Components: V, S, M (a miniature door worth 15+ GP)\
Duration: 24 hours

You conjure a shimmering door in range that lasts for the duration. The door leads to an extradimensional dwelling and is 5 feet wide and 10 feet tall. You and any creature you designate when you cast the spell can enter the extradimensional dwelling as long as the door remains open. You can open or close it (no action required) if you are within 30 feet of it. While closed, the door is imperceptible.

Beyond the door is a magnificent foyer with numerous chambers beyond. The dwelling’s atmosphere is clean, fresh, and warm.

You can create any floor plan you like for the dwelling, but it can’t exceed 50 contiguous 10-foot Cubes. The place is furnished and decorated as you choose. It contains sufficient food to serve a nine-course banquet for up to 100 people. Furnishings and other objects created by this spell dissipate into smoke if removed from it.

A staff of 100 near-transparent servants attends all who enter. You determine the appearance of these servants and their attire. They are invulnerable and obey your commands. Each servant can perform tasks that a human could perform, but they can’t attack or take any action that would directly harm another creature. Thus the servants can fetch things, clean, mend, fold clothes, light fires, serve food, pour wine, and so on. The servants can’t leave the dwelling.

When the spell ends, any creatures or objects left inside the extradimensional space are expelled into the unoccupied spaces nearest to the entrance.

Original layout: [[../../srd-5.2.1/pages/147|page 147]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 7 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["bard", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 24 hours ` |
| ` casting_time ` | ` 1 minute ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 300 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a miniature door worth 15+ GP ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/magnificent_mansion ` |
| ` implementation ` | ` dnd2024:spell_magnificent_mansion ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` doorway_location ` |
| ` within_range ` | ` true ` |

#### Doorway

<!-- splat:yaml path=splat_rule/parameters/doorway -->

```yaml
width_feet: 5
height_feet: 10
duration: spell_duration
entry_allowed: caster_and_creatures_designated_at_cast
entry_requires: open
open_close:
  economy: none
  caster_range_feet: 30
closed: imperceptible
```

#### Dwelling

<!-- splat:yaml path=splat_rule/parameters/dwelling -->

```yaml
extradimensional: true
air:
- clean
- fresh
- warm
floor_plan: caster_choice
max_contiguous_cubes: 50
cube_side_feet: 10
furnishings: caster_choice
banquet:
  courses: 9
  people: 100
created_objects_removed: dissipate_into_smoke
```

#### Servants

<!-- splat:values path=splat_rule/parameters/servants -->

| Field | Value |
| --- | --- |
| ` count ` | ` 100 ` |
| ` appearance_and_attire ` | ` caster_choice ` |
| ` near_transparent ` | ` true ` |
| ` invulnerable ` | ` true ` |
| ` obeys ` | ` caster_commands ` |
| ` tasks ` | ` human_capable ` |
| ` forbidden ` | ` ["attack", "directly_harm_creature", "leave_dwelling"] ` |
| ` serves ` | ` all_entrants ` |

#### End

<!-- splat:values path=splat_rule/parameters/end -->

| Field | Value |
| --- | --- |
| ` expel ` | ` all_remaining_creatures_and_objects ` |
| ` destination ` | ` nearest_unoccupied_spaces_to_entrance ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  value: 24
  unit: hour
```
