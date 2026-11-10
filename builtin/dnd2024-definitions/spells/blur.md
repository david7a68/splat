---
splat_schema: 2
splat_id: 481e80c6-2f2e-5a32-b487-836f6b1955e4
splat_kind: spell
splat_key: dnd2024:spells/blur
splat_name: Blur
splat_origin: dnd2024:reference/spell/blur
splat_sources:
- SRD 5.2.1, pp. 114, https://www.dndbeyond.com/srd
---

# Blur


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Illusion |
| Classes | Sorcerer, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V |
| Duration | Concentration, up to 1 minute |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 114 -->

Level 2 Illusion (Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V\
Duration: Concentration, up to 1 minute

Your body becomes blurred. For the duration, any creature has Disadvantage on attack rolls against you. An attacker is immune to this effect if it perceives you with Blindsight or Truesight.

Original layout: [[../../srd-5.2.1/pages/114|page 114]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/blur ` |
| ` implementation ` | ` dnd2024:spell_blur ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` self ` |

#### Incoming attacks

<!-- splat:values path=splat_rule/parameters/incoming_attacks -->

| Field | Value |
| --- | --- |
| ` disadvantage ` | ` true ` |
| ` duration ` | ` spell_duration ` |
| ` immune_if_attacker_perceives_caster_using ` | ` ["blindsight", "truesight"] ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: self
> ```

