---
splat_schema: 2
splat_id: a5e28636-7477-5376-a7d6-7a91d1928c65
splat_kind: spell
splat_key: dnd2024:spells/revivify
splat_name: Revivify
splat_origin: dnd2024:reference/spell/revivify
splat_sources:
- SRD 5.2.1, pp. 159, https://www.dndbeyond.com/srd
---

# Revivify


## Spell

| Field | Value |
| --- | --- |
| Level | 3 |
| School | Necromancy |
| Classes | Cleric, Druid, Paladin, Ranger |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (a diamond worth 300+ GP, which the spell consumes) |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 159 -->

Level 3 Necromancy (Cleric, Druid, Paladin, Ranger)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a diamond worth 300+ GP, which the spell consumes)\
Duration: Instantaneous

You touch a creature that has died within the last minute. That creature revives with 1 Hit Point. This spell can’t revive a creature that has died of old age, nor does it restore any missing body parts.

Original layout: [[../../srd-5.2.1/pages/159|page 159]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/revivify ` |
| ` implementation ` | ` dnd2024:spell_revivify ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |
| ` touch ` | ` true ` |
| ` dead_within_minutes ` | ` 1 ` |
| ` death_by_old_age ` | ` false ` |

#### Revive

<!-- splat:values path=splat_rule/parameters/revive -->

| Field | Value |
| --- | --- |
| ` hp ` | ` 1 ` |
| ` restore_missing_parts ` | ` false ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: true
> range:
>   mode: touch
> ```

