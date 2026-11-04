---
splat_schema: 2
splat_id: a179fbfa-9463-5466-9827-e2e0c45259e7
splat_kind: spell
splat_key: dnd2024:spells/light
splat_name: Light
splat_origin: dnd2024:reference/spell/light
splat_sources:
- SRD 5.2.1, pp. 144, https://www.dndbeyond.com/srd
---

# Light


## Spell

| Field | Value |
| --- | --- |
| Level | Cantrip |
| School | Evocation |
| Classes | Bard, Cleric, Sorcerer, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, M (a firefly or phosphorescent moss) |
| Duration | 1 hour |


## Source text

<!-- SRD page 144 -->

Evocation Cantrip (Bard, Cleric, Sorcerer, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, M (a firefly or phosphorescent moss)\
Duration: 1 hour

You touch one Large or smaller object that isn’t being worn or carried by someone else. Until the spell ends, the object sheds Bright Light in a 20-foot radius and Dim Light for an additional 20 feet. The light can be colored as you like.

Covering the object with something opaque blocks the light. The spell ends if you cast it again.

Original layout: [[../../srd-5.2.1/pages/144|page 144]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/light ` |
| ` implementation ` | ` dnd2024:spell_light ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` object ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |
| ` touch ` | ` true ` |
| ` maximum_size ` | ` large ` |
| ` not_worn_or_carried_by_other ` | ` true ` |

#### Light

<!-- splat:values path=splat_rule/parameters/light -->

| Field | Value |
| --- | --- |
| ` bright_radius_feet ` | ` 20 ` |
| ` additional_dim_feet ` | ` 20 ` |
| ` color_chooser ` | ` caster ` |
| ` opaque_cover_blocks ` | ` true ` |

#### End

<!-- splat:values path=splat_rule/parameters/end -->

| Field | Value |
| --- | --- |
| ` event ` | ` caster.recasts_same_spell ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: touch
> ```

