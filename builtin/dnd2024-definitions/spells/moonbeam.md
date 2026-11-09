---
splat_schema: 2
splat_id: 38502ecc-19d6-5bc4-8492-c5031eda6016
splat_kind: spell
splat_key: dnd2024:spells/moonbeam
splat_name: Moonbeam
splat_origin: dnd2024:reference/spell/moonbeam
splat_sources:
- SRD 5.2.1, pp. 150, 151, https://www.dndbeyond.com/srd
---

# Moonbeam


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Evocation |
| Classes | Druid |
| Casting Time | Action |
| Range | 120 feet |
| Components | V, S, M (a moonseed leaf) |
| Duration | Concentration, up to 1 minute |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 150 -->

Level 2 Evocation (Druid)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a moonseed leaf)\
Duration: Concentration, up to 1 minute

<!-- SRD page 151 -->

A silvery beam of pale light shines down in a 5-foot-radius, 40-foot-high Cylinder centered on a point within range. Until the spell ends, Dim Light fills the Cylinder, and you can take a Magic action on later turns to move the Cylinder up to 60 feet.

When the Cylinder appears, each creature in it makes a Constitution saving throw. On a failed save, a creature takes 2d10 Radiant damage, and if the creature is shape-shifted (as a result of the Polymorph spell, for example), it reverts to its true form and can’t shape-shift until it leaves the Cylinder. On a successful save, a creature takes half as much damage only. A creature also makes this save when the spell’s area moves into its space and when it enters the spell’s area or ends its turn there. A creature makes this save only once per turn.

Using a Higher-Level Spell Slot. The damage increases by 1d10 for each spell slot level above 2.

Original layout: [[../../srd-5.2.1/pages/150|page 150]], [[../../srd-5.2.1/pages/151|page 151]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/moonbeam ` |
| ` implementation ` | ` dnd2024:spell_moonbeam ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` triggers ` | ` ["spell.cast_completed", "area.moves_into_creature", "area.enter", "turn.end"] ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cylinder ` |
| ` radius_feet ` | ` 5 ` |
| ` height_feet ` | ` 40 ` |
| ` light ` | ` dim ` |

#### Move

<!-- splat:values path=splat_rule/parameters/move -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` timing ` | ` later_caster_turns ` |
| ` maximum_feet ` | ` 60 ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
dc:
  binding: save_dc
failure:
  damage:
    amount:
      dice:
        count: 2
        sides: 10
        add: 0
    type: radiant
  if_shape_shifted:
    revert_true_form: true
    prevent_shape_shift_until: leaves_cylinder
success:
  damage_fraction:
    numerator: 1
    denominator: 2
  revert: false
```

#### Limit

<!-- splat:values path=splat_rule/parameters/limit -->

| Field | Value |
| --- | --- |
| ` saves_per_creature_per_turn ` | ` 1 ` |

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 2 ` |
| ` damage_dice ` | ` 1 ` |
| ` sides ` | ` 10 ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

