---
splat_schema: 2
splat_id: b67983cc-4eb9-5253-8c2a-8480d86cc59d
splat_kind: spell
splat_key: dnd2024:spells/glibness
splat_name: Glibness
splat_origin: dnd2024:reference/spell/glibness
splat_sources:
- SRD 5.2.1, pp. 136, https://www.dndbeyond.com/srd
---

# Glibness


## Spell

| Field | Value |
| --- | --- |
| Level | 8 |
| School | Enchantment |
| Classes | Bard, Warlock |
| Casting Time | Action |
| Range |  |
| Components | V |
| Duration | 1 hour |


## Source text

<!-- SRD page 136 -->

Level 8 Enchantment (Bard, Warlock)\
Casting Time: Action\
Range: Self\
Components: V\
Duration: 1 hour

Until the spell ends, when you make a Charisma check, you can replace the number you roll with a 15. Additionally, no matter what you say, magic that would determine if you are telling the truth indicates that you are being truthful.

Original layout: [[../../srd-5.2.1/pages/136|page 136]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/glibness ` |
| ` implementation ` | ` dnd2024:spell_glibness ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` target ` | ` caster ` |

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` maximum_seconds ` | ` 3600 ` |

#### Roll replacement

<!-- splat:values path=splat_rule/parameters/roll_replacement -->

| Field | Value |
| --- | --- |
| ` event ` | ` check.rolled ` |
| ` ability ` | ` charisma ` |
| ` optional ` | ` true ` |
| ` chooser ` | ` caster ` |
| ` replace ` | ` die_result ` |
| ` value ` | ` 15 ` |

#### Truth detection

<!-- splat:values path=splat_rule/parameters/truth_detection -->

| Field | Value |
| --- | --- |
| ` source_kind ` | ` magic ` |
| ` actual_statement_truth ` | ` any ` |
| ` reported_truth ` | ` truthful ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: self
> ```

