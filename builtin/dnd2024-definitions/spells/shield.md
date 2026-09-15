---
splat_schema: 1
splat_id: 33e0cba6-30a4-5de9-98df-c9152ea6c596
splat_kind: spell
splat_key: dnd2024:spells/shield
splat_name: Shield
splat_origin:
  key: dnd2024:reference/spell/shield
splat_sources:
- title: SRD 5.2.1
  locator: pp. 161, 162
  url: https://www.dndbeyond.com/srd
---

# Shield

## Source text

<!-- SRD page 161 -->

Level 1 Abjuration (Sorcerer, Wizard)\
Casting Time: Reaction, which you take when you are\
hit by an attack roll or targeted by the Magic Missile\
spell\
Range: Self\
Components: V, S\
Duration: 1 round\
An imperceptible barrier of magical force protects\
you. Until the start of your next turn, you have a +5\

<!-- SRD page 162 -->

bonus to AC, including against the triggering attack,\
and you take no damage from Magic Missile.\

Original layout: [[../../srd-5.2.1/pages/161|page 161]], [[../../srd-5.2.1/pages/162|page 162]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 round ` |
| ` casting_time ` | ` Reaction, which you take when you are hit by an attack roll or targeted by the Magic Missile spell ` |
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
