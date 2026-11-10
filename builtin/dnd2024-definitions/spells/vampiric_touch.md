---
splat_schema: 2
splat_id: a7b78458-d9b6-56c5-8c6b-a0d5278f759c
splat_kind: spell
splat_key: dnd2024:spells/vampiric_touch
splat_name: Vampiric Touch
splat_origin: dnd2024:reference/spell/vampiric_touch
splat_sources:
- SRD 5.2.1, pp. 171, https://www.dndbeyond.com/srd
---

# Vampiric Touch


## Spell

| Field | Value |
| --- | --- |
| Level | 3 |
| School | Necromancy |
| Classes | Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S |
| Duration | Concentration, up to 1 minute |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 171 -->

Level 3 Necromancy (Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Concentration, up to 1 minute

The touch of your shadow-wreathed hand can siphon life force from others to heal your wounds. Make a melee spell attack against one creature within reach. On a hit, the target takes 3d6 Necrotic damage, and you regain Hit Points equal to half the amount of Necrotic damage dealt.

Until the spell ends, you can make the attack again on each of your turns as a Magic action, targeting the same creature or a different one.

Using a Higher-Level Spell Slot. The damage increases by 1d6 for each spell slot level above 3.

Original layout: [[../../srd-5.2.1/pages/171|page 171]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/vampiric_touch ` |
| ` implementation ` | ` dnd2024:spell_vampiric_touch ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` within_reach ` | ` true ` |

#### Attack

<!-- splat:yaml path=splat_rule/parameters/attack -->

```yaml
kind: melee_spell
hit:
  damage:
    dice: 3d6
    type: necrotic
  caster_healing:
    fraction_of_necrotic_damage_dealt:
      numerator: 1
      denominator: 2
    rounding: down
```

#### Repeat

<!-- splat:values path=splat_rule/parameters/repeat -->

| Field | Value |
| --- | --- |
| ` economy ` | ` magic_action ` |
| ` timing ` | ` each_caster_turn ` |
| ` duration ` | ` spell_duration ` |
| ` target ` | ` same_or_different ` |

#### Slot scaling

<!-- splat:values path=splat_rule/parameters/slot_scaling -->

| Field | Value |
| --- | --- |
| ` above ` | ` 3 ` |
| ` damage_per_level ` | ` 1d6 ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 1
  unit: minute
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: self
> ```

