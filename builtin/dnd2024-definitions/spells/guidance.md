---
splat_schema: 2
splat_id: 25f83e04-8d94-5821-bd9f-8febd5436a55
splat_kind: spell
splat_key: dnd2024:spells/guidance
splat_name: Guidance
splat_origin: dnd2024:reference/spell/guidance
splat_sources:
- SRD 5.2.1, pp. 138, https://www.dndbeyond.com/srd
---

# Guidance


## Spell

| Field | Value |
| --- | --- |
| Level | Cantrip |
| School | Divination |
| Classes | Cleric, Druid |
| Casting Time | Action |
| Range |  |
| Components | V, S |
| Duration | Concentration, up to 1 minute |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 138 -->

Divination Cantrip (Cleric, Druid)\
Casting Time: Action\
Range: Touch\
Component: V, S\
Duration: Concentration, up to 1 minute

You touch a willing creature and choose a skill. Until the spell ends, the creature adds 1d4 to any ability check using the chosen skill.

Original layout: [[../../srd-5.2.1/pages/138|page 138]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/guidance ` |
| ` implementation ` | ` dnd2024:spell_guidance ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |
| ` willing ` | ` true ` |
| ` touch ` | ` true ` |

#### Skill choice

<!-- splat:values path=splat_rule/parameters/skill_choice -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` selection ` | ` skill ` |

#### Bonus

<!-- splat:yaml path=splat_rule/parameters/bonus -->

```yaml
roll: ability_check
requires_skill: chosen
amount:
  dice:
    count: 1
    sides: 4
    add: 0
all_matching_checks: true
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: touch
> ```

