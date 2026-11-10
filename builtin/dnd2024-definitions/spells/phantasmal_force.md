---
splat_schema: 2
splat_id: 0331a325-1bf8-55b5-8810-62b0fed75e95
splat_kind: spell
splat_key: dnd2024:spells/phantasmal_force
splat_name: Phantasmal Force
splat_origin: dnd2024:reference/spell/phantasmal_force
splat_sources:
- SRD 5.2.1, pp. 151, 152, https://www.dndbeyond.com/srd
---

# Phantasmal Force


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Illusion |
| Classes | Bard, Sorcerer, Wizard |
| Casting Time | Action |
| Range | 60 feet |
| Components | V, S, M (a bit of fleece) |
| Duration | Concentration, up to 1 minute |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 151 -->

Level 2 Illusion (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a bit of fleece)\
Duration: Concentration, up to 1 minute

You attempt to craft an illusion in the mind of a creature you can see within range. The target makes an Intelligence saving throw. On a failed save, you create a phantasmal object, creature, or other phenomenon that is no larger than a 10-foot Cube and that is perceivable only to the target for the duration. The phantasm includes sound, temperature, and other stimuli.

The target can take a Study action to examine the phantasm with an Intelligence (Investigation) check against your spell save DC. If the check succeeds, the target realizes that the phantasm is an illusion, and the spell ends.

<!-- SRD page 152 -->

While affected by the spell, the target treats the phantasm as if it were real and rationalizes any illogical outcomes from interacting with it. For example, if the target steps through a phantasmal bridge and survives the fall, it believes the bridge exists and something else caused it to fall.

An affected target can even take damage from the illusion if the phantasm represents a dangerous creature or hazard. On each of your turns, such a phantasm can deal 2d8 Psychic damage to the target if it is in the phantasm’s area or within 5 feet of the phantasm. The target perceives the damage as a type appropriate to the illusion.

Original layout: [[../../srd-5.2.1/pages/151|page 151]], [[../../srd-5.2.1/pages/152|page 152]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/phantasmal_force ` |
| ` implementation ` | ` dnd2024:spell_phantasmal_force ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible ` | ` true ` |
| ` within_range ` | ` true ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` intelligence ` |
| ` failure ` | ` create_phantasm ` |
| ` success ` | ` no_effect ` |

#### Phantasm

<!-- splat:values path=splat_rule/parameters/phantasm -->

| Field | Value |
| --- | --- |
| ` choices ` | ` ["object", "creature", "phenomenon"] ` |
| ` max_cube_side_feet ` | ` 10 ` |
| ` perceived_by ` | ` target_only ` |
| ` duration ` | ` spell_duration ` |
| ` stimuli ` | ` ["visual", "sound", "temperature", "other_sensory"] ` |
| ` target_believes_real ` | ` true ` |
| ` illogical_interaction ` | ` rationalized ` |

#### Discern

<!-- splat:yaml path=splat_rule/parameters/discern -->

```yaml
actor: target
economy: study_action
check:
  ability: intelligence
  skill: investigation
  dc: spell_save_dc
success:
  realizes_illusion: true
  end_spell: true
```

#### Damage

<!-- splat:values path=splat_rule/parameters/damage -->

| Field | Value |
| --- | --- |
| ` requires ` | ` phantasm_depicts_dangerous_creature_or_hazard ` |
| ` timing ` | ` each_caster_turn ` |
| ` optional ` | ` true ` |
| ` target_location ` | ` inside_or_within_5_feet_of_phantasm ` |
| ` dice ` | ` 2d8 ` |
| ` type ` | ` psychic ` |
| ` perceived_type ` | ` appropriate_to_illusion ` |

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
> material:
>   consumed: false
> ```

