---
splat_schema: 2
splat_id: 4ff38385-cf51-5684-abd1-6ed1274c5916
splat_kind: spell
splat_key: dnd2024:spells/cure_wounds
splat_name: Cure Wounds
splat_origin: dnd2024:reference/spell/cure_wounds
splat_sources:
- SRD 5.2.1, pp. 121, https://www.dndbeyond.com/srd
---

# Cure Wounds


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Abjuration |
| Classes | Bard, Cleric, Druid, Paladin, Ranger |
| Casting Time | Action |
| Range |  |
| Components | V, S |
| Duration | Instantaneous |


## Source text

<!-- SRD page 121 -->

Level 1 Abjuration (Bard, Cleric, Druid, Paladin, Ranger)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Instantaneous

A creature you touch regains a number of Hit Points equal to 2d8 plus your spellcasting ability modifier.

Using a Higher-Level Spell Slot. The healing increases by 2d8 for each spell slot level above 1.

Original layout: [[../../srd-5.2.1/pages/121|page 121]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/cure_wounds ` |
| ` implementation ` | ` dnd2024:spell_cure_wounds ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |
| ` touch ` | ` true ` |

#### Healing

<!-- splat:yaml path=splat_rule/parameters/healing -->

```yaml
dice:
  count: 2
  sides: 8
  add:
    binding: spellcasting_modifier
```

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 1 ` |
| ` healing_dice ` | ` 2 ` |
| ` sides ` | ` 8 ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: touch
> ```

