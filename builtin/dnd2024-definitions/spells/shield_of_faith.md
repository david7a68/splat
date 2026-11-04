---
splat_schema: 2
splat_id: f8a6cb55-7317-5890-9e47-339778a5c94b
splat_kind: spell
splat_key: dnd2024:spells/shield_of_faith
splat_name: Shield of Faith
splat_origin: dnd2024:reference/spell/shield_of_faith
splat_sources:
- SRD 5.2.1, pp. 162, https://www.dndbeyond.com/srd
---

# Shield of Faith


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Abjuration |
| Classes | Cleric, Paladin |
| Casting Time | Bonus Action |
| Range | 60 feet |
| Components | V, S, M (a prayer scroll) |
| Duration | Concentration, up to 10 minutes |


## Source text

<!-- SRD page 162 -->

Level 1 Abjuration (Cleric, Paladin)\
Casting Time: Bonus Action\
Range: 60 feet\
Components: V, S, M (a prayer scroll)\
Duration: Concentration, up to 10 minutes

A shimmering field surrounds a creature of your choice within range, granting it a +2 bonus to AC for the duration.

Original layout: [[../../srd-5.2.1/pages/162|page 162]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/shield_of_faith ` |
| ` implementation ` | ` dnd2024:spell_shield_of_faith ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 600 ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` chooser ` | ` caster ` |
| ` range_feet ` | ` 60 ` |

#### Modifier

<!-- splat:values path=splat_rule/parameters/modifier -->

| Field | Value |
| --- | --- |
| ` stat ` | ` armor_class ` |
| ` add ` | ` 2 ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

