---
splat_schema: 2
splat_id: 732c6263-2133-5754-a5da-900b6d33f302
splat_kind: spell
splat_key: dnd2024:spells/color_spray
splat_name: Color Spray
splat_origin: dnd2024:reference/spell/color_spray
splat_sources:
- SRD 5.2.1, pp. 116, https://www.dndbeyond.com/srd
---

# Color Spray


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Illusion |
| Classes | Bard, Sorcerer, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (a pinch of colorful sand) |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 116 -->

Level 1 Illusion (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a pinch of colorful sand)\
Duration: Instantaneous

You launch a dazzling array of flashing, colorful light. Each creature in a 15-foot Cone originating from you must succeed on a Constitution saving throw or have the Blinded condition until the end of your next turn.

Original layout: [[../../srd-5.2.1/pages/116|page 116]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/color_spray ` |
| ` implementation ` | ` dnd2024:spell_color_spray ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` instantaneous ` | ` true ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cone ` |
| ` length_feet ` | ` 15 ` |
| ` origin ` | ` caster ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` selection ` | ` all_in_area ` |

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
condition: blinded
until:
  event: turn.end
  subject: caster
  occurrence: next
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
> range:
>   mode: self
> ```

