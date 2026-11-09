---
splat_schema: 2
splat_id: ba44ffb5-d3ba-51e7-b550-a6a5e0ec0729
splat_kind: spell
splat_key: dnd2024:spells/demiplane
splat_name: Demiplane
splat_origin: dnd2024:reference/spell/demiplane
splat_sources:
- SRD 5.2.1, pp. 122, 123, https://www.dndbeyond.com/srd
---

# Demiplane


## Spell

| Field | Value |
| --- | --- |
| Level | 8 |
| School | Conjuration |
| Classes | Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range | 60 feet |
| Components | S |
| Duration | 1 hour |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 122 -->

Level 8 Conjuration (Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: S\
Duration: 1 hour

<!-- SRD page 123 -->

You create a shadowy Medium door on a flat solid surface that you can see within range. This door can be opened and closed, and it leads to a demiplane that is an empty room 30 feet in each dimension, made of wood or stone (your choice).

When the spell ends, the door vanishes, and any objects inside the demiplane remain there. Any creatures inside also remain unless they opt to be shunted through the door as it vanishes, landing with the Prone condition in the unoccupied spaces closest to the door’s former space.

Each time you cast this spell, you can create a new demiplane or connect the shadowy door to a demiplane you created with a previous casting of this spell. Additionally, if you know the nature and contents of a demiplane created by a casting of this spell by another creature, you can connect the shadowy door to that demiplane instead.

Original layout: [[../../srd-5.2.1/pages/122|page 122]], [[../../srd-5.2.1/pages/123|page 123]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/demiplane ` |
| ` implementation ` | ` dnd2024:spell_demiplane ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` maximum_seconds ` | ` 3600 ` |

#### Doorway

<!-- splat:yaml path=splat_rule/parameters/doorway -->

```yaml
size: medium
range_feet: 60
surface:
  flat: true
  solid: true
  visible: true
can_open_close: true
```

#### Destination choices

<!-- splat:values path=splat_rule/parameters/destination_choices -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` options ` | ` ["new_demiplane", "caster_previous_demiplane", "other_caster_demiplane"] ` |
| ` other_caster_requirement ` | ` know_nature_and_contents ` |

#### New room

<!-- splat:yaml path=splat_rule/parameters/new_room -->

```yaml
empty: true
length_feet: 30
width_feet: 30
height_feet: 30
material:
  chooser: caster
  options:
  - wood
  - stone
```

#### End

<!-- splat:yaml path=splat_rule/parameters/end -->

```yaml
door_vanishes: true
objects_remain: true
creatures:
  chooser: each_creature
  options:
  - remain
  - exit
  exit:
    destination: nearest_unoccupied_space_to_former_door
    condition: prone
```
