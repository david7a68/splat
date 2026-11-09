---
splat_schema: 2
splat_id: b89c1b71-68a2-5dae-b127-22e491d91252
splat_kind: spell
splat_key: dnd2024:spells/forbiddance
splat_name: Forbiddance
splat_origin: dnd2024:reference/spell/forbiddance
splat_sources:
- SRD 5.2.1, pp. 133, https://www.dndbeyond.com/srd
---

# Forbiddance


## Spell

| Field | Value |
| --- | --- |
| Level | 6 |
| School | Abjuration |
| Classes | Cleric |
| Casting Time | 10 minutes or Ritual |
| Range |  |
| Components | V, S, M (ruby dust worth 1,000+ GP) |
| Duration | 1 day |
| Ritual | Yes |
| Concentration | No |


## Source text

<!-- SRD page 133 -->

Level 6 Abjuration (Cleric)\
Casting Time: 10 minutes or Ritual\
Range: Touch\
Components: V, S, M (ruby dust worth 1,000+ GP)\
Duration: 1 day

You create a ward against magical travel that protects up to 40,000 square feet of floor space to a height of 30 feet above the floor. For the duration, creatures can’t teleport into the area or use portals, such as those created by the Gate spell, to enter the area. The spell proofs the area against planar travel, and therefore prevents creatures from accessing the area by way of the Astral Plane, the Ethereal Plane, the Feywild, the Shadowfell, or the Plane Shift spell.

In addition, the spell damages types of creatures that you choose when you cast it. Choose one or more of the following: Aberrations, Celestials, Elementals, Fey, Fiends, and Undead. When a creature of a chosen type enters the spell’s area for the first time on a turn or ends its turn there, the creature takes 5d10 Radiant or Necrotic damage (your choice when you cast this spell).

You can designate a password when you cast the spell. A creature that speaks the password as it enters the area takes no damage from the spell.

The spell’s area can’t overlap with the area of another Forbiddance spell. If you cast Forbiddance every day for 30 days in the same location, the spell lasts until it is dispelled, and the Material components are consumed on the last casting.

Original layout: [[../../srd-5.2.1/pages/133|page 133]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/forbiddance ` |
| ` implementation ` | ` dnd2024:spell_forbiddance ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` floor_area ` |
| ` touch ` | ` true ` |
| ` max_square_feet ` | ` 40000 ` |
| ` height_feet ` | ` 30 ` |
| ` cannot_overlap ` | ` other_forbiddance ` |

#### Travel block

<!-- splat:values path=splat_rule/parameters/travel_block -->

| Field | Value |
| --- | --- |
| ` entry_modes ` | ` ["teleportation", "portals", "planar_travel"] ` |
| ` includes ` | ` ["astral", "ethereal", "feywild", "shadowfell", "plane_shift"] ` |
| ` duration ` | ` spell_duration ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
types_choice:
  minimum: 1
  choices:
  - aberration
  - celestial
  - elemental
  - fey
  - fiend
  - undead
events:
- first_area_entry_each_turn
- target_turn_end_inside
dice: 5d10
damage_type_choice:
- radiant
- necrotic
exemption: password_spoken_on_entry
password: caster_optional_at_cast
```

#### Persistence

<!-- splat:values path=splat_rule/parameters/persistence -->

| Field | Value |
| --- | --- |
| ` same_location ` | ` true ` |
| ` consecutive_days ` | ` 30 ` |
| ` frequency ` | ` daily ` |
| ` resulting_duration ` | ` until_dispelled ` |
| ` material_consumed ` | ` last_cast_only ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  value: 1
  unit: day
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: touch
> ```

