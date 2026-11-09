---
splat_schema: 2
splat_id: 5340380a-57af-5094-9f38-02fe5c88b00b
splat_kind: spell
splat_key: dnd2024:spells/silence
splat_name: Silence
splat_origin: dnd2024:reference/spell/silence
splat_sources:
- SRD 5.2.1, pp. 162, https://www.dndbeyond.com/srd
---

# Silence


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Illusion |
| Classes | Bard, Cleric, Ranger |
| Casting Time | Action or Ritual |
| Range | 120 feet |
| Components | V, S |
| Duration | Concentration, up to 10 minutes |
| Ritual | Yes |
| Concentration | Yes |


## Source text

<!-- SRD page 162 -->

Level 2 Illusion (Bard, Cleric, Ranger)\
Casting Time: Action or Ritual\
Range: 120 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes

For the duration, no sound can be created within or pass through a 20-foot-radius Sphere centered on a point you choose within range. Any creature or object entirely inside the Sphere has Immunity to Thunder damage, and creatures have the Deafened condition while entirely inside it. Casting a spell that includes a Verbal component is impossible there.

Original layout: [[../../srd-5.2.1/pages/162|page 162]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/silence ` |
| ` implementation ` | ` dnd2024:spell_silence ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 600 ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 20 ` |
| ` origin ` | ` chosen_point ` |
| ` range_feet ` | ` 120 ` |

#### Sound

<!-- splat:values path=splat_rule/parameters/sound -->

| Field | Value |
| --- | --- |
| ` creation_inside ` | ` forbidden ` |
| ` passage_through ` | ` forbidden ` |

#### Entirely inside

<!-- splat:yaml path=splat_rule/parameters/entirely_inside -->

```yaml
creatures_and_objects:
  damage_immunity: thunder
creatures:
  condition: deafened
```

#### Spellcasting inside

<!-- splat:values path=splat_rule/parameters/spellcasting_inside -->

| Field | Value |
| --- | --- |
| ` verbal_component ` | ` forbidden ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` source ` | ` supplied_world_geometry ` |
| ` unknown_facts ` | ` require_resolution ` |
| ` infer_unprovided_geometry ` | ` false ` |
