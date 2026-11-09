---
splat_schema: 2
splat_id: 30bf5254-82bb-570e-b6c6-79897d4fcbf0
splat_kind: spell
splat_key: dnd2024:spells/misty_step
splat_name: Misty Step
splat_origin: dnd2024:reference/spell/misty_step
splat_sources:
- SRD 5.2.1, pp. 150, https://www.dndbeyond.com/srd
---

# Misty Step


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Conjuration |
| Classes | Sorcerer, Warlock, Wizard |
| Casting Time | Bonus Action |
| Range |  |
| Components | V |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 150 -->

Level 2 Conjuration (Sorcerer, Warlock, Wizard)\
Casting Time: Bonus Action\
Range: Self\
Components: V\
Duration: Instantaneous

Briefly surrounded by silvery mist, you teleport up to 30 feet to an unoccupied space you can see.

Original layout: [[../../srd-5.2.1/pages/150|page 150]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/misty_step ` |
| ` implementation ` | ` dnd2024:spell_misty_step ` |

### Parameters

#### Teleport

<!-- splat:values path=splat_rule/parameters/teleport -->

| Field | Value |
| --- | --- |
| ` subject ` | ` caster ` |
| ` maximum_feet ` | ` 30 ` |
| ` destination_unoccupied ` | ` true ` |
| ` destination_visible ` | ` true ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: self
> ```

