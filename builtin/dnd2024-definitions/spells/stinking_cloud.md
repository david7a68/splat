---
splat_schema: 2
splat_id: b4589ffa-998a-52df-bd77-d3db52107ad2
splat_kind: spell
splat_key: dnd2024:spells/stinking_cloud
splat_name: Stinking Cloud
splat_origin: dnd2024:reference/spell/stinking_cloud
splat_sources:
- SRD 5.2.1, pp. 165, https://www.dndbeyond.com/srd
---

# Stinking Cloud


## Spell

| Field | Value |
| --- | --- |
| Level | 3 |
| School | Conjuration |
| Classes | Bard, Sorcerer, Wizard |
| Casting Time | Action |
| Range | 90 feet |
| Components | V, S, M (a rotten egg) |
| Duration | Concentration, up to 1 minute |


## Source text

<!-- SRD page 165 -->

Level 3 Conjuration (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 90 feet\
Components: V, S, M (a rotten egg)\
Duration: Concentration, up to 1 minute

You create a 20-foot-radius Sphere of yellow, nauseating gas centered on a point within range. The cloud is Heavily Obscured. The cloud lingers in the air for the duration or until a strong wind (such as the one created by Gust of Wind) disperses it.

Each creature that starts its turn in the Sphere must succeed on a Constitution saving throw or have the Poisoned condition until the end of the current turn. While Poisoned in this way, the creature can’t take an action or a Bonus Action.

Original layout: [[../../srd-5.2.1/pages/165|page 165]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/stinking_cloud ` |
| ` implementation ` | ` dnd2024:spell_stinking_cloud ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 60 ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 20 ` |
| ` origin ` | ` chosen_point ` |
| ` range_feet ` | ` 90 ` |

#### Properties

<!-- splat:values path=splat_rule/parameters/properties -->

| Field | Value |
| --- | --- |
| ` obscurement ` | ` heavy ` |
| ` medium ` | ` gas ` |

#### End early

<!-- splat:yaml path=splat_rule/parameters/end_early -->

```yaml
cause: strong_wind_dispersal
example_spell:
  key: dnd2024:spells/gust_of_wind
```

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` event ` | ` turn.start ` |
| ` subjects ` | ` creatures_in_area ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` constitution ` |
| ` dc ` | ` spell_save_dc ` |
| ` success ` | ` unaffected ` |

#### Failure

<!-- splat:yaml path=splat_rule/parameters/failure -->

```yaml
condition: poisoned
until:
  event: turn.end
  subject: target
  occurrence: current
while_poisoned:
  action: forbidden
  bonus_action: forbidden
```

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

