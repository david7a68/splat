---
splat_schema: 2
splat_id: 4cac4ccd-a035-56ef-accb-deae22998694
splat_kind: spell
splat_key: dnd2024:spells/regenerate
splat_name: Regenerate
splat_origin: dnd2024:reference/spell/regenerate
splat_sources:
- SRD 5.2.1, pp. 158, https://www.dndbeyond.com/srd
---

# Regenerate


## Spell

| Field | Value |
| --- | --- |
| Level | 7 |
| School | Transmutation |
| Classes | Bard, Cleric, Druid |
| Casting Time | 1 minute |
| Range |  |
| Components | V, S, M (a prayer wheel) |
| Duration | 1 hour |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 158 -->

Level 7 Transmutation (Bard, Cleric, Druid)\
Casting Time: 1 minute\
Range: Touch\
Components: V, S, M (a prayer wheel)\
Duration: 1 hour

A creature you touch regains 4d8 + 15 Hit Points. For the duration, the target regains 1 Hit Point at the start of each of its turns, and any severed body parts regrow after 2 minutes.

Original layout: [[../../srd-5.2.1/pages/158|page 158]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/regenerate ` |
| ` implementation ` | ` dnd2024:spell_regenerate ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` touch ` | ` true ` |

#### Initial healing

<!-- splat:values path=splat_rule/parameters/initial_healing -->

| Field | Value |
| --- | --- |
| ` dice ` | ` 4d8 ` |
| ` add ` | ` 15 ` |

#### Ongoing

<!-- splat:yaml path=splat_rule/parameters/ongoing -->

```yaml
duration: spell_duration
healing:
  event: target.turn_started
  amount: 1
severed_part_regrowth_minutes: 2
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  value: 1
  unit: hour
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: touch
> ```

