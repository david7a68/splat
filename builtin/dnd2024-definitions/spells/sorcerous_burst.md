---
splat_schema: 1
splat_id: 8ec51fa9-2142-5cb4-8a48-0506239318cf
splat_kind: spell
splat_key: dnd2024:spells/sorcerous_burst
splat_name: Sorcerous Burst
splat_origin:
  key: dnd2024:reference/spell/sorcerous_burst
splat_sources:
- title: SRD 5.2.1
  locator: pp. 163
  url: https://www.dndbeyond.com/srd
---

# Sorcerous Burst

## Source text

<!-- SRD page 163 -->

Evocation Cantrip (Sorcerer)\
Casting Time: Action\
Range: 120 feet\
Component: V, S\
Duration: Instantaneous

You cast sorcerous energy at one creature or object within range. Make a ranged spell attack against the target. On a hit, the target takes 1d8 damage of a type you choose: Acid, Cold, Fire, Lightning, Poison, Psychic, or Thunder.

If you roll an 8 on a d8 for this spell, you can roll another d8, and add it to the damage. When you cast this spell, the maximum number of these d8s you can add to the spell’s damage equals your spellcasting ability modifier.

Cantrip Upgrade. The damage increases by 1d8 when you reach levels 5 (2d8), 11 (3d8), and 17 (4d8).

Original layout: [[../../srd-5.2.1/pages/163|page 163]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 0 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["sorcerer"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 120 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/sorcerous_burst ` |
| ` implementation ` | ` dnd2024:spell_sorcerous_burst ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature_or_object ` |
| ` count ` | ` 1 ` |
| ` within_range ` | ` true ` |

#### Attack

<!-- splat:yaml path=splat_rule/parameters/attack -->

```yaml
kind: ranged_spell
hit:
  damage:
    dice: 1d8
    type_choice:
    - acid
    - cold
    - fire
    - lightning
    - poison
    - psychic
    - thunder
```

#### Exploding dice

<!-- splat:yaml path=splat_rule/parameters/exploding_dice -->

```yaml
trigger: any_spell_d8_result_is_8
optional: true
additional_die: 1d8
add_to_damage: true
added_dice_can_trigger: true
total_added_dice_limit_per_cast:
  stat: caster_spellcasting_ability_modifier
```

#### Cantrip scaling

<!-- splat:yaml path=splat_rule/parameters/cantrip_scaling -->

```yaml
character_level_dice:
  '1': 1d8
  '5': 2d8
  '11': 3d8
  '17': 4d8
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```
