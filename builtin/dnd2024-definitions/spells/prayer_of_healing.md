---
splat_schema: 2
splat_id: 38054dda-f99e-5a3d-9ae7-627cfeba4b88
splat_kind: spell
splat_key: dnd2024:spells/prayer_of_healing
splat_name: Prayer of Healing
splat_origin: dnd2024:reference/spell/prayer_of_healing
splat_sources:
- SRD 5.2.1, pp. 154, https://www.dndbeyond.com/srd
---

# Prayer of Healing


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Abjuration |
| Classes | Cleric, Paladin |
| Casting Time | 10 minutes |
| Range | 30 feet |
| Components | V |
| Duration | Instantaneous |


## Source text

<!-- SRD page 154 -->

Level 2 Abjuration (Cleric, Paladin)\
Casting Time: 10 minutes\
Range: 30 feet\
Components: V\
Duration: Instantaneous

Up to five creatures of your choice who remain within range for the spell’s entire casting gain the benefits of a Short Rest and also regain 2d8 Hit Points. A creature can’t be affected by this spell again until that creature finishes a Long Rest.

Using a Higher-Level Spell Slot. The healing increases by 1d8 for each spell slot level above 2.

Original layout: [[../../srd-5.2.1/pages/154|page 154]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/prayer_of_healing ` |
| ` implementation ` | ` dnd2024:spell_prayer_of_healing ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creatures ` |
| ` max_count ` | ` 5 ` |
| ` chosen ` | ` true ` |
| ` within_range_for_entire_cast ` | ` true ` |
| ` eligibility ` | ` no_benefit_since_last_long_rest ` |

#### Benefits

<!-- splat:yaml path=splat_rule/parameters/benefits -->

```yaml
short_rest: true
healing:
  dice: 2d8
```

#### Repeat limit

<!-- splat:values path=splat_rule/parameters/repeat_limit -->

| Field | Value |
| --- | --- |
| ` scope ` | ` each_target ` |
| ` reset ` | ` target_long_rest_finished ` |

#### Slot scaling

<!-- splat:values path=splat_rule/parameters/slot_scaling -->

| Field | Value |
| --- | --- |
| ` above ` | ` 2 ` |
| ` healing_per_level ` | ` 1d8 ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```
