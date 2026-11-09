---
splat_schema: 2
splat_id: 9c0b220a-a3f4-5c12-b884-dc929fe32d70
splat_kind: spell
splat_key: dnd2024:spells/continual_flame
splat_name: Continual Flame
splat_origin: dnd2024:reference/spell/continual_flame
splat_sources:
- SRD 5.2.1, pp. 119, https://www.dndbeyond.com/srd
---

# Continual Flame


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Evocation |
| Classes | Cleric, Druid, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (ruby dust worth 50+ GP, which the spell consumes) |
| Duration | Until dispelled |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 119 -->

Level 2 Evocation (Cleric, Druid, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (ruby dust worth 50+ GP, which the spell consumes)\
Duration: Until dispelled

A flame springs from an object that you touch. The effect casts Bright Light in a 20-foot radius and Dim Light for an additional 20 feet. It looks like a regular flame, but it creates no heat and consumes no fuel. The flame can be covered or hidden but not smothered or quenched.

Original layout: [[../../srd-5.2.1/pages/119|page 119]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/continual_flame ` |
| ` implementation ` | ` dnd2024:spell_continual_flame ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` object ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |
| ` touch ` | ` true ` |

#### Light

<!-- splat:values path=splat_rule/parameters/light -->

| Field | Value |
| --- | --- |
| ` bright_radius_feet ` | ` 20 ` |
| ` additional_dim_feet ` | ` 20 ` |

#### Flame

<!-- splat:values path=splat_rule/parameters/flame -->

| Field | Value |
| --- | --- |
| ` heat ` | ` false ` |
| ` fuel_consumption ` | ` false ` |
| ` coverable ` | ` true ` |
| ` hideable ` | ` true ` |
| ` smotherable ` | ` false ` |
| ` quenchable ` | ` false ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: true
> range:
>   mode: touch
> ```

