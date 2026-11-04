---
splat_schema: 2
splat_id: 0533a1be-f938-5436-908d-f50e2c3a505b
splat_kind: spell
splat_key: dnd2024:spells/divine_favor
splat_name: Divine Favor
splat_origin: dnd2024:reference/spell/divine_favor
splat_sources:
- SRD 5.2.1, pp. 125, https://www.dndbeyond.com/srd
---

# Divine Favor


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Transmutation |
| Classes | Paladin |
| Casting Time | Bonus Action |
| Range |  |
| Components | V, S |
| Duration | 1 minute |


## Source text

<!-- SRD page 125 -->

Level 1 Transmutation (Paladin)\
Casting Time: Bonus Action\
Range: Self\
Components: V, S\
Duration: 1 minute

Until the spell ends, your attacks with weapons deal an extra 1d4 Radiant damage on a hit.

Original layout: [[../../srd-5.2.1/pages/125|page 125]].





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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: self
> ```

