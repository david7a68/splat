---
splat_schema: 2
splat_id: 8938815b-ac6e-599b-a3f5-60cbc66a3bc6
splat_kind: spell
splat_key: dnd2024:spells/gentle_repose
splat_name: Gentle Repose
splat_origin: dnd2024:reference/spell/gentle_repose
splat_sources:
- SRD 5.2.1, pp. 135, https://www.dndbeyond.com/srd
---

# Gentle Repose


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Necromancy |
| Classes | Cleric, Paladin, Wizard |
| Casting Time | Action or Ritual |
| Range |  |
| Components | V, S, M (2 Copper Pieces, which the spell consumes) |
| Duration | 10 days |


## Source text

<!-- SRD page 135 -->

Level 2 Necromancy (Cleric, Paladin, Wizard)\
Casting Time: Action or Ritual\
Range: Touch\
Components: V, S, M (2 Copper Pieces, which the spell consumes)\
Duration: 10 days

You touch a corpse or other remains. For the duration, the target is protected from decay and can’t become Undead.

The spell also effectively extends the time limit on raising the target from the dead, since days spent under the influence of this spell don’t count against the time limit of spells such as Raise Dead.

Original layout: [[../../srd-5.2.1/pages/135|page 135]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/gentle_repose ` |
| ` implementation ` | ` dnd2024:spell_gentle_repose ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` duration ` | ` spell_duration ` |

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` corpse_or_remains ` |
| ` touch ` | ` true ` |

#### Protection

<!-- splat:values path=splat_rule/parameters/protection -->

| Field | Value |
| --- | --- |
| ` decay ` | ` prevented ` |
| ` becoming_undead ` | ` prevented ` |

#### Resurrection deadline

<!-- splat:values path=splat_rule/parameters/resurrection_deadline -->

| Field | Value |
| --- | --- |
| ` exclude_days ` | ` days_under_this_effect ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  value: 10
  unit: day
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: true
> range:
>   mode: touch
> ```

