---
splat_schema: 2
splat_id: 6f0a27ec-46de-51ee-952c-eac79c4f4d4b
splat_kind: spell
splat_key: dnd2024:spells/plant_growth
splat_name: Plant Growth
splat_origin: dnd2024:reference/spell/plant_growth
splat_sources:
- SRD 5.2.1, pp. 153, https://www.dndbeyond.com/srd
---

# Plant Growth


## Spell

| Field | Value |
| --- | --- |
| Level | 3 |
| School | Transmutation |
| Classes | Bard, Druid, Ranger |
| Casting Time | Action (Overgrowth) or 8 hours (Enrichment) |
| Range | 150 feet |
| Components | V, S |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 153 -->

Level 3 Transmutation (Bard, Druid, Ranger)\
Casting Time: Action (Overgrowth) or 8 hours (Enrichment)\
Range: 150 feet\
Components: V, S\
Duration: Instantaneous

This spell channels vitality into plants. The casting time you use determines whether the spell has the Overgrowth or the Enrichment effect below.

Overgrowth. Choose a point within range. All normal plants in a 100-foot-radius Sphere centered on that point become thick and overgrown. A creature moving through that area must spend 4 feet of movement for every 1 foot it moves. You can exclude one or more areas of any size within the spell’s area from being affected.

Enrichment. All plants in a half-mile radius centered on a point within range become enriched for 365 days. The plants yield twice the normal amount of food when harvested. They can benefit from only one Plant Growth per year.

Original layout: [[../../srd-5.2.1/pages/153|page 153]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/plant_growth ` |
| ` implementation ` | ` dnd2024:spell_plant_growth ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` instantaneous ` | ` true ` |

#### Mode choice

<!-- splat:values path=splat_rule/parameters/mode_choice -->

| Field | Value |
| --- | --- |
| ` caster_choice ` | ` true ` |
| ` determined_by ` | ` casting_time ` |

#### Overgrowth

<!-- splat:yaml path=splat_rule/parameters/overgrowth -->

```yaml
casting_economy: action
origin:
  chosen_point: true
  range_feet: 150
shape: sphere
radius_feet: 100
affected_plants: normal
growth: thick_overgrown
movement_feet_cost_per_foot: 4
exclusions:
  chooser: caster
  count: any
  area_size: any
  within_spell_area: true
```

#### Enrichment

<!-- splat:yaml path=splat_rule/parameters/enrichment -->

```yaml
casting_seconds: 28800
origin:
  chosen_point: true
  range_feet: 150
radius_miles: 0.5
affected_plants: all
duration_days: 365
harvest_food_multiplier: 2
benefit_limit:
  count: 1
  per: year
  scope: plant
```

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |
