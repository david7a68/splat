---
splat_schema: 1
splat_id: af9bbc02-fe63-5b5f-8af3-c07cd85d8a85
splat_kind: spell
splat_key: dnd2024:spells/mending
splat_name: Mending
splat_origin:
  key: dnd2024:reference/spell/mending
splat_sources:
- title: SRD 5.2.1
  locator: pp. 148
  url: https://www.dndbeyond.com/srd
---

# Mending

## Source text

<!-- SRD page 148 -->

Transmutation Cantrip (Bard, Cleric, Druid, Sorcerer,\
Wizard)\
Casting Time: 1 minute\
Range: Touch\
Components: V, S, M (two lodestones)\
Duration: Instantaneous\
This spell repairs a single break or tear in an object\
you touch, such as a broken chain link, two halves of\
a broken key, a torn cloak, or a leaking wineskin. As\
long as the break or tear is no larger than 1 foot in\
any dimension, you mend it, leaving no trace of the\
former damage.\
This spell can physically repair a magic item, but\
it can’t restore magic to such an object.\

Original layout: [[../../srd-5.2.1/pages/148|page 148]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 0 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["bard", "cleric", "druid", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` 1 minute ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` two lodestones ` |
| ` consumed ` | ` false ` |

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
