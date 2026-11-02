---
splat_schema: 1
splat_id: ebbd596b-a1bb-5673-a0cb-89aab8f3dd7f
splat_kind: spell
splat_key: dnd2024:spells/arcane_lock
splat_name: Arcane Lock
splat_origin:
  key: dnd2024:reference/spell/arcane_lock
splat_sources:
- title: SRD 5.2.1
  locator: pp. 110
  url: https://www.dndbeyond.com/srd
---

# Arcane Lock

## Source text

<!-- SRD page 110 -->

Level 2 Abjuration (Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (gold dust worth 25+ GP, which the spell consumes)\
Duration: Until dispelled

You touch a closed door, window, gate, container, or hatch and magically lock it for the duration. This lock can’t be unlocked by any nonmagical means. You and any creatures you designate when you cast the spell can open and close the object despite the lock. You can also set a password that, when spoken within 5 feet of the object, unlocks it for 1 minute.

Original layout: [[../../srd-5.2.1/pages/110|page 110]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Until dispelled ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` gold dust worth 25+ GP, which the spell consumes ` |
| ` consumed ` | ` true ` |

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
