---
splat_schema: 2
splat_id: 0c3a1349-5ee0-5340-a9ab-d57829167f14
splat_kind: spell
splat_key: dnd2024:spells/barkskin
splat_name: Barkskin
splat_origin: dnd2024:reference/spell/barkskin
splat_sources:
- SRD 5.2.1, pp. 112, https://www.dndbeyond.com/srd
---

# Barkskin


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Transmutation |
| Classes | Druid, Ranger |
| Casting Time | Bonus Action |
| Range |  |
| Components | V, S, M (a handful of bark) |
| Duration | 1 hour |


## Source text

<!-- SRD page 112 -->

Level 2 Transmutation (Druid, Ranger)\
Casting Time: Bonus Action\
Range: Touch\
Component: V, S, M (a handful of bark)\
Duration: 1 hour

You touch a willing creature. Until the spell ends, the target’s skin assumes a bark-like appearance, and the target has an Armor Class of 17 if its AC is lower than that.

Original layout: [[../../srd-5.2.1/pages/112|page 112]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/barkskin ` |
| ` implementation ` | ` dnd2024:spell_barkskin ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` appearance ` | ` bark_skin ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |
| ` willing ` | ` true ` |
| ` touch ` | ` true ` |

#### Armor class

<!-- splat:values path=splat_rule/parameters/armor_class -->

| Field | Value |
| --- | --- |
| ` minimum ` | ` 17 ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: touch
> ```

