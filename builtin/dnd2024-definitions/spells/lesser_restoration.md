---
splat_schema: 2
splat_id: f9ed276d-2674-5d36-ae3b-0e5c34192ee7
splat_kind: spell
splat_key: dnd2024:spells/lesser_restoration
splat_name: Lesser Restoration
splat_origin: dnd2024:reference/spell/lesser_restoration
splat_sources:
- SRD 5.2.1, pp. 144, https://www.dndbeyond.com/srd
---

# Lesser Restoration


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Abjuration |
| Classes | Bard, Cleric, Druid, Paladin, Ranger |
| Casting Time | Bonus Action |
| Range |  |
| Components | V, S |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 144 -->

Level 2 Abjuration (Bard, Cleric, Druid, Paladin, Ranger)\
Casting Time: Bonus Action\
Range: Touch\
Components: V, S\
Duration: Instantaneous

You touch a creature and end one condition on it: Blinded, Deafened, Paralyzed, or Poisoned.

Original layout: [[../../srd-5.2.1/pages/144|page 144]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/lesser_restoration ` |
| ` implementation ` | ` dnd2024:spell_lesser_restoration ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 1 ` |
| ` within_spell_range ` | ` true ` |
| ` touched ` | ` true ` |

#### Remove condition

<!-- splat:values path=splat_rule/parameters/remove_condition -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` choices ` | ` ["blinded", "deafened", "paralyzed", "poisoned"] ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: touch
> ```

