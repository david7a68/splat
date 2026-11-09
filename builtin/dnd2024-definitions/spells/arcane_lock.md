---
splat_schema: 2
splat_id: ebbd596b-a1bb-5673-a0cb-89aab8f3dd7f
splat_kind: spell
splat_key: dnd2024:spells/arcane_lock
splat_name: Arcane Lock
splat_origin: dnd2024:reference/spell/arcane_lock
splat_sources:
- SRD 5.2.1, pp. 110, https://www.dndbeyond.com/srd
---

# Arcane Lock


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Abjuration |
| Classes | Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (gold dust worth 25+ GP, which the spell consumes) |
| Duration | Until dispelled |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 110 -->

Level 2 Abjuration (Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (gold dust worth 25+ GP, which the spell consumes)\
Duration: Until dispelled

You touch a closed door, window, gate, container, or hatch and magically lock it for the duration. This lock can’t be unlocked by any nonmagical means. You and any creatures you designate when you cast the spell can open and close the object despite the lock. You can also set a password that, when spoken within 5 feet of the object, unlocks it for 1 minute.

Original layout: [[../../srd-5.2.1/pages/110|page 110]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/arcane_lock ` |
| ` implementation ` | ` dnd2024:spell_arcane_lock ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` until ` | ` dispelled_or_source_specific_end ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kinds ` | ` ["door", "window", "gate", "container", "hatch"] ` |
| ` closed ` | ` true ` |
| ` touch ` | ` true ` |

#### Lock

<!-- splat:values path=splat_rule/parameters/lock -->

| Field | Value |
| --- | --- |
| ` magical ` | ` true ` |
| ` nonmagical_unlock ` | ` impossible ` |

#### Authorized open close

<!-- splat:yaml path=splat_rule/parameters/authorized_open_close -->

```yaml
caster: true
designated_creatures:
  chooser: caster
  timing: cast
```

#### Password

<!-- splat:values path=splat_rule/parameters/password -->

| Field | Value |
| --- | --- |
| ` optional ` | ` true ` |
| ` chooser ` | ` caster ` |
| ` spoken_within_feet ` | ` 5 ` |
| ` unlock_seconds ` | ` 60 ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: true
> range:
>   mode: touch
> ```

