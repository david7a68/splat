---
splat_schema: 2
splat_id: 1f2b7ac1-fa40-53ca-9bae-203376d8160c
splat_kind: spell
splat_key: dnd2024:spells/telepathic_bond
splat_name: Telepathic Bond
splat_origin: dnd2024:reference/spell/telepathic_bond
splat_sources:
- SRD 5.2.1, pp. 168, https://www.dndbeyond.com/srd
---

# Telepathic Bond


## Spell

| Field | Value |
| --- | --- |
| Level | 5 |
| School | Divination |
| Classes | Bard, Wizard |
| Casting Time | Action or Ritual |
| Range | 30 feet |
| Components | V, S, M (two eggs) |
| Duration | 1 hour |
| Ritual | Yes |
| Concentration | No |


## Source text

<!-- SRD page 168 -->

Level 5 Divination (Bard, Wizard)\
Casting Time: Action or Ritual\
Range: 30 feet\
Components: V, S, M (two eggs)\
Duration: 1 hour

You forge a telepathic link among up to eight willing creatures of your choice within range, psychically linking each creature to all the others for the duration. Creatures that can’t communicate in any languages aren’t affected by this spell.

Until the spell ends, the targets can communicate telepathically through the bond whether or not they share a language. The communication is possible over any distance, though it can’t extend to other planes of existence.

Original layout: [[../../srd-5.2.1/pages/168|page 168]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/telepathic_bond ` |
| ` implementation ` | ` dnd2024:spell_telepathic_bond ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creatures ` |
| ` max_count ` | ` 8 ` |
| ` willing ` | ` true ` |
| ` chosen ` | ` true ` |
| ` within_range ` | ` true ` |
| ` must_communicate_in_at_least_one_language ` | ` true ` |

#### Bond

<!-- splat:values path=splat_rule/parameters/bond -->

| Field | Value |
| --- | --- |
| ` topology ` | ` each_to_all ` |
| ` channel ` | ` telepathic ` |
| ` shared_language_required ` | ` false ` |
| ` distance_limit ` | ` none ` |
| ` planar_crossing ` | ` false ` |
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
>   consumed: false
> ```

