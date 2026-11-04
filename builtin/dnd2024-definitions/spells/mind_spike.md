---
splat_schema: 2
splat_id: 7652eed9-eb1e-5d0c-8598-a193dc31cedd
splat_kind: spell
splat_key: dnd2024:spells/mind_spike
splat_name: Mind Spike
splat_origin: dnd2024:reference/spell/mind_spike
splat_sources:
- SRD 5.2.1, pp. 149, https://www.dndbeyond.com/srd
---

# Mind Spike


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Divination |
| Classes | Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range | 120 feet |
| Components | S |
| Duration | Concentration, up to 1 hour |


## Source text

<!-- SRD page 149 -->

Level 2 Divination (Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: S\
Duration: Concentration, up to 1 hour

You drive a spike of psionic energy into the mind of one creature you can see within range. The target makes a Wisdom saving throw, taking 3d8 Psychic damage on a failed save or half as much damage on a successful one. On a failed save, you also always know the target’s location until the spell ends, but only while the two of you are on the same plane of existence. While you have this knowledge, the target can’t become hidden from you, and if it has the Invisible condition, it gains no benefit from that condition against you.

Using a Higher-Level Spell Slot. The damage increases by 1d8 for each spell slot level above 2.

Original layout: [[../../srd-5.2.1/pages/149|page 149]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/mind_spike ` |
| ` implementation ` | ` dnd2024:spell_mind_spike ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible ` | ` true ` |
| ` within_range ` | ` true ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: wisdom
failure:
  damage:
    dice: 3d8
    type: psychic
  tracking:
    duration: spell_duration
    requires: same_plane
    knows_location: always
    target_cannot_hide_from_caster: true
    invisible_benefits_against_caster: none
success:
  damage_fraction:
    numerator: 1
    denominator: 2
```

#### Slot scaling

<!-- splat:values path=splat_rule/parameters/slot_scaling -->

| Field | Value |
| --- | --- |
| ` above ` | ` 2 ` |
| ` damage_per_level ` | ` 1d8 ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 1
  unit: hour
```
