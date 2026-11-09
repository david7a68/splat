---
splat_schema: 2
splat_id: 56396d62-7912-5e51-9052-467334315b70
splat_kind: spell
splat_key: dnd2024:spells/private_sanctum
splat_name: Private Sanctum
splat_origin: dnd2024:reference/spell/private_sanctum
splat_sources:
- SRD 5.2.1, pp. 156, https://www.dndbeyond.com/srd
---

# Private Sanctum


## Spell

| Field | Value |
| --- | --- |
| Level | 4 |
| School | Abjuration |
| Classes | Wizard |
| Casting Time | 10 minutes |
| Range | 120 feet |
| Components | V, S, M (a thin sheet of lead) |
| Duration | 24 hours |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 156 -->

Level 4 Abjuration (Wizard)\
Casting Time: 10 minutes\
Range: 120 feet\
Components: V, S, M (a thin sheet of lead)\
Duration: 24 hours

You make an area within range magically secure. The area is a Cube that can be as small as 5 feet to as large as 100 feet on each side. The spell lasts for the duration.

When you cast the spell, you decide what sort of security the spell provides, choosing any of the following properties:

•  Sound can’t pass through the barrier at the edge of the warded area.

•  The barrier of the warded area appears dark and foggy, preventing vision (including Darkvision) through it.

•  Sensors created by Divination spells can’t appear inside the protected area or pass through the barrier at its perimeter.

•  Creatures in the area can’t be targeted by Divination spells.

•  Nothing can teleport into or out of the warded area.

•  Planar travel is blocked within the warded area.

Casting this spell on the same spot every day for 365 days makes the spell last until dispelled.

Using a Higher-Level Spell Slot. You can increase the size of the Cube by 100 feet for each spell slot level above 4.

Original layout: [[../../srd-5.2.1/pages/156|page 156]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/private_sanctum ` |
| ` implementation ` | ` dnd2024:spell_private_sanctum ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` duration ` | ` spell_duration ` |

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` area ` |
| ` shape ` | ` cube ` |
| ` min_side_feet ` | ` 5 ` |
| ` max_side_feet ` | ` 100 ` |
| ` within_range ` | ` true ` |

#### Security

<!-- splat:values path=splat_rule/parameters/security -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster_at_cast ` |
| ` count ` | ` any ` |
| ` choices ` | ` ["block_sound_crossing_boundary", "dark_foggy_boundary_blocks_vision_including_darkvision", "block_divination_sensor_appearance_inside_and_perimeter_crossing", "creatures_inside_untargetable_by_divination_spells", "block_teleportation_in_and_out", "block_planar_travel_within"] ` |

#### Persistence

<!-- splat:values path=splat_rule/parameters/persistence -->

| Field | Value |
| --- | --- |
| ` same_spot ` | ` true ` |
| ` frequency ` | ` daily ` |
| ` consecutive_days ` | ` 365 ` |
| ` resulting_duration ` | ` until_dispelled ` |

#### Slot scaling

<!-- splat:values path=splat_rule/parameters/slot_scaling -->

| Field | Value |
| --- | --- |
| ` above ` | ` 4 ` |
| ` max_cube_side_feet_per_level ` | ` 100 ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  value: 24
  unit: hour
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

