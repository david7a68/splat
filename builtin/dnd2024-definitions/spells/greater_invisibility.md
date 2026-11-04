---
splat_schema: 2
splat_id: b557d649-7130-5904-ad90-1d256cce72cb
splat_kind: spell
splat_key: dnd2024:spells/greater_invisibility
splat_name: Greater Invisibility
splat_origin: dnd2024:reference/spell/greater_invisibility
splat_sources:
- SRD 5.2.1, pp. 137, https://www.dndbeyond.com/srd
---

# Greater Invisibility


## Spell

| Field | Value |
| --- | --- |
| Level | 4 |
| School | Illusion |
| Classes | Bard, Sorcerer, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S |
| Duration | Concentration, up to 1 minute |


## Source text

<!-- SRD page 137 -->

Level 4 Illusion (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Concentration, up to 1 minute

A creature you touch has the Invisible condition until the spell ends.

Original layout: [[../../srd-5.2.1/pages/137|page 137]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/greater_invisibility ` |
| ` implementation ` | ` dnd2024:spell_greater_invisibility ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |
| ` touch ` | ` true ` |

#### Condition

<!-- splat:values path=splat_rule/parameters/condition -->

| Field | Value |
| --- | --- |
| ` name ` | ` invisible ` |
| ` duration ` | ` spell ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: touch
> ```

