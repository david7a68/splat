---
splat_schema: 2
splat_id: 33e0cba6-30a4-5de9-98df-c9152ea6c596
splat_kind: spell
splat_key: dnd2024:spells/shield
splat_name: Shield
splat_origin: dnd2024:reference/spell/shield
splat_sources:
- SRD 5.2.1, pp. 161, 162, https://www.dndbeyond.com/srd
---

# Shield


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Abjuration |
| Classes | Sorcerer, Wizard |
| Casting Time | Reaction, which you take when you are hit by an attack roll or targeted by the Magic Missile spell |
| Range |  |
| Components | V, S |
| Duration | 1 round |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 161 -->

Level 1 Abjuration (Sorcerer, Wizard)\
Casting Time: Reaction, which you take when you are hit by an attack roll or targeted by the Magic Missile spell\
Range: Self\
Components: V, S\
Duration: 1 round

An imperceptible barrier of magical force protects you. Until the start of your next turn, you have a +5 bonus to AC, including against the triggering attack, and you take no damage from Magic Missile.

Original layout: [[../../srd-5.2.1/pages/161|page 161]], [[../../srd-5.2.1/pages/162|page 162]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/shield ` |
| ` implementation ` | ` dnd2024:spell_shield ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` caster ` |

#### Protection

<!-- splat:yaml path=splat_rule/parameters/protection -->

```yaml
until: caster_next_turn_start
ac_bonus: 5
includes_triggering_attack: true
no_damage_from_spell:
  key: dnd2024:spells/magic_missile
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: self
> ```

