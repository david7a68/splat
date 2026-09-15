---
splat_schema: 1
splat_id: 0533a1be-f938-5436-908d-f50e2c3a505b
splat_kind: spell
splat_key: dnd2024:spells/divine_favor
splat_name: Divine Favor
splat_origin:
  key: dnd2024:reference/spell/divine_favor
splat_sources:
- title: SRD 5.2.1
  locator: pp. 125
  url: https://www.dndbeyond.com/srd
---

# Divine Favor

## Source text

<!-- SRD page 125 -->

Level 1 Transmutation (Paladin)\
Casting Time: Bonus Action\
Range: Self\
Components: V, S\
Duration: 1 minute\
Until the spell ends, your attacks with weapons deal\
an extra 1d4 Radiant damage on a hit.\

Original layout: [[../../srd-5.2.1/pages/125|page 125]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["paladin"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 minute ` |
| ` casting_time ` | ` Bonus Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/divine_favor ` |
| ` implementation ` | ` dnd2024:spell_divine_favor ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` maximum_seconds ` | ` 60 ` |

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` event ` | ` attack.hit ` |
| ` attacker ` | ` caster ` |
| ` weapon_required ` | ` true ` |

#### Extra damage

<!-- splat:values path=splat_rule/parameters/extra_damage -->

| Field | Value |
| --- | --- |
| ` dice ` | ` 1d4 ` |
| ` type ` | ` radiant ` |
