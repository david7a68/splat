---
splat_schema: 2
splat_id: af9bbc02-fe63-5b5f-8af3-c07cd85d8a85
splat_kind: spell
splat_key: dnd2024:spells/mending
splat_name: Mending
splat_origin: dnd2024:reference/spell/mending
splat_sources:
- SRD 5.2.1, pp. 148, https://www.dndbeyond.com/srd
---

# Mending


## Spell

| Field | Value |
| --- | --- |
| Level | Cantrip |
| School | Transmutation |
| Classes | Bard, Cleric, Druid, Sorcerer, Wizard |
| Casting Time | 1 minute |
| Range |  |
| Components | V, S, M (two lodestones) |
| Duration | Instantaneous |


## Source text

<!-- SRD page 148 -->

Transmutation Cantrip (Bard, Cleric, Druid, Sorcerer, Wizard)\
Casting Time: 1 minute\
Range: Touch\
Components: V, S, M (two lodestones)\
Duration: Instantaneous

This spell repairs a single break or tear in an object you touch, such as a broken chain link, two halves of a broken key, a torn cloak, or a leaking wineskin. As long as the break or tear is no larger than 1 foot in any dimension, you mend it, leaving no trace of the former damage.

This spell can physically repair a magic item, but it can’t restore magic to such an object.

Original layout: [[../../srd-5.2.1/pages/148|page 148]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/mending ` |
| ` implementation ` | ` dnd2024:spell_mending ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` object ` |
| ` touch ` | ` true ` |
| ` break_or_tear_count ` | ` 1 ` |
| ` break_or_tear_max_dimension_feet ` | ` 1 ` |

#### Repair

<!-- splat:values path=splat_rule/parameters/repair -->

| Field | Value |
| --- | --- |
| ` physical ` | ` true ` |
| ` damage_trace_remaining ` | ` false ` |
| ` magic_item_eligible ` | ` true ` |
| ` restores_magic ` | ` false ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: touch
> ```

