---
splat_schema: 2
splat_id: 5df3db72-49ee-5d1c-aa5e-51e6b7e2496c
splat_kind: spell
splat_key: dnd2024:spells/incendiary_cloud
splat_name: Incendiary Cloud
splat_origin: dnd2024:reference/spell/incendiary_cloud
splat_sources:
- SRD 5.2.1, pp. 142, 143, https://www.dndbeyond.com/srd
---

# Incendiary Cloud


## Spell

| Field | Value |
| --- | --- |
| Level | 8 |
| School | Conjuration |
| Classes | Druid, Sorcerer, Wizard |
| Casting Time | Action |
| Range | 150 feet |
| Components | V, S |
| Duration | Concentration, up to 1 minute |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 142 -->

Level 8 Conjuration (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 150 feet\
Components: V, S\
Duration: Concentration, up to 1 minute

A swirling cloud of embers and smoke fills a 20-foot-radius Sphere centered on a point within range. The cloud’s area is Heavily Obscured. It lasts for the duration or until a strong wind (like that created by Gust of Wind) disperses it.

When the cloud appears, each creature in it makes a Dexterity saving throw, taking 10d8 Fire damage on a failed save or half as much damage on a successful one. A creature must also make this save when the Sphere moves into its space and when it enters the Sphere or ends its turn there. A creature makes this save only once per turn.

<!-- SRD page 143 -->

The cloud moves 10 feet away from you in a direction you choose at the start of each of your turns.

Original layout: [[../../srd-5.2.1/pages/142|page 142]], [[../../srd-5.2.1/pages/143|page 143]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/incendiary_cloud ` |
| ` implementation ` | ` dnd2024:spell_incendiary_cloud ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creatures_in_area ` |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 20 ` |
| ` origin ` | ` chosen_point_in_range ` |

#### Cloud

<!-- splat:values path=splat_rule/parameters/cloud -->

| Field | Value |
| --- | --- |
| ` heavily_obscured ` | ` true ` |
| ` until_any ` | ` ["spell_duration", "strong_wind_dispersal"] ` |

#### Damage triggers

<!-- splat:values path=splat_rule/parameters/damage_triggers -->

| Field | Value |
| --- | --- |
| ` events ` | ` ["cloud_appears", "cloud_moves_into_creature_space", "creature_enters_cloud", "creature_ends_turn_in_cloud"] ` |
| ` max_per_creature_per_turn ` | ` 1 ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: dexterity
failure:
  damage:
    dice: 10d8
    type: fire
success:
  damage_fraction:
    numerator: 1
    denominator: 2
```

#### Movement

<!-- splat:values path=splat_rule/parameters/movement -->

| Field | Value |
| --- | --- |
| ` event ` | ` caster.turn_started ` |
| ` distance_feet ` | ` 10 ` |
| ` direction ` | ` caster_choice_away_from_caster ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 1
  unit: minute
```
