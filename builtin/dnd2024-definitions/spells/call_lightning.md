---
splat_schema: 2
splat_id: 129a5122-8826-5942-890f-0418c4dad6e7
splat_kind: spell
splat_key: dnd2024:spells/call_lightning
splat_name: Call Lightning
splat_origin: dnd2024:reference/spell/call_lightning
splat_sources:
- SRD 5.2.1, pp. 114, https://www.dndbeyond.com/srd
---

# Call Lightning


## Spell

| Field | Value |
| --- | --- |
| Level | 3 |
| School | Conjuration |
| Classes | Druid |
| Casting Time | Action |
| Range | 120 feet |
| Components | V, S |
| Duration | Concentration, up to 10 minutes |


## Source text

<!-- SRD page 114 -->

Level 3 Conjuration (Druid)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes

A storm cloud appears at a point within range that you can see above yourself. It takes the shape of a Cylinder that is 10 feet tall with a 60-foot radius.

When you cast the spell, choose a point you can see under the cloud. A lightning bolt shoots from the cloud to that point. Each creature within 5 feet of that point makes a Dexterity saving throw, taking 3d10 Lightning damage on a failed save or half as much damage on a successful one.

Until the spell ends, you can take a Magic action to call down lightning in that way again, targeting the same point or a different one.

If you’re outdoors in a storm when you cast this spell, the spell gives you control over that storm instead of creating a new one. Under such conditions, the spell’s damage increases by 1d10.

Using a Higher-Level Spell Slot. The damage increases by 1d10 for each spell slot level above 3.

Original layout: [[../../srd-5.2.1/pages/114|page 114]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/call_lightning ` |
| ` implementation ` | ` dnd2024:spell_call_lightning ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` triggers ` | ` ["spell.cast_completed"] ` |

#### Cloud

<!-- splat:values path=splat_rule/parameters/cloud -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cylinder ` |
| ` height_feet ` | ` 10 ` |
| ` radius_feet ` | ` 60 ` |
| ` origin_visible ` | ` true ` |
| ` above_caster ` | ` true ` |

#### Bolt

<!-- splat:yaml path=splat_rule/parameters/bolt -->

```yaml
target: visible_point_under_cloud
creatures_within_feet: 5
save:
  ability: dexterity
  dc:
    binding: save_dc
  failure:
    damage:
      amount:
        dice:
          count: 3
          sides: 10
          add: 0
      type: lightning
  success:
    damage_fraction:
      numerator: 1
      denominator: 2
```

#### Repeat

<!-- splat:values path=splat_rule/parameters/repeat -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` same_or_new_point ` | ` true ` |

#### Existing storm

<!-- splat:yaml path=splat_rule/parameters/existing_storm -->

```yaml
requires:
- outdoors
- storm_at_cast
replace_cloud_creation_with_storm_control: true
bonus_damage:
  dice:
    count: 1
    sides: 10
    add: 0
```

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 3 ` |
| ` damage_dice ` | ` 1 ` |
| ` sides ` | ` 10 ` |
