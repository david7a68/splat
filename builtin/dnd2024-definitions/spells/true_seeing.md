---
splat_schema: 2
splat_id: 97c986c3-b654-5430-ab0f-97c7ecf40a31
splat_kind: spell
splat_key: dnd2024:spells/true_seeing
splat_name: True Seeing
splat_origin: dnd2024:reference/spell/true_seeing
splat_sources:
- SRD 5.2.1, pp. 171, https://www.dndbeyond.com/srd
---

# True Seeing


## Spell

| Field | Value |
| --- | --- |
| Level | 6 |
| School | Divination |
| Classes | Bard, Cleric, Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (mushroom powder worth 25+ GP, which the spell consumes) |
| Duration | 1 hour |


## Source text

<!-- SRD page 171 -->

Level 6 Divination (Bard, Cleric, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (mushroom powder worth 25+ GP, which the spell consumes)\
Duration: 1 hour

For the duration, the willing creature you touch has Truesight with a range of 120 feet.

Original layout: [[../../srd-5.2.1/pages/171|page 171]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/true_seeing ` |
| ` implementation ` | ` dnd2024:spell_true_seeing ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` willing ` | ` true ` |
| ` touch ` | ` true ` |

#### Sense

<!-- splat:values path=splat_rule/parameters/sense -->

| Field | Value |
| --- | --- |
| ` kind ` | ` truesight ` |
| ` range_feet ` | ` 120 ` |
| ` duration ` | ` spell_duration ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  value: 1
  unit: hour
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: true
> range:
>   mode: touch
> ```

