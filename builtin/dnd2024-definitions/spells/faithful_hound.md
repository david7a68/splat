---
splat_schema: 2
splat_id: 8de80790-d19b-520d-8f3a-3e6c7a91eba8
splat_kind: spell
splat_key: dnd2024:spells/faithful_hound
splat_name: Faithful Hound
splat_origin: dnd2024:reference/spell/faithful_hound
splat_sources:
- SRD 5.2.1, pp. 129, https://www.dndbeyond.com/srd
---

# Faithful Hound


## Spell

| Field | Value |
| --- | --- |
| Level | 4 |
| School | Conjuration |
| Classes | Wizard |
| Casting Time | Action |
| Range | 30 feet |
| Components | V, S, M (a silver whistle) |
| Duration | 8 hours |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 129 -->

Level 4 Conjuration (Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a silver whistle)\
Duration: 8 hours

You conjure a phantom watchdog in an unoccupied space that you can see within range. The hound remains for the duration or until the two of you are more than 300 feet apart from each other.

No one but you can see the hound, and it is intangible and invulnerable. When a Small or larger creature comes within 30 feet of it without first speaking the password that you specify when you cast this spell, the hound starts barking loudly. The hound has Truesight with a range of 30 feet.

At the start of each of your turns, the hound attempts to bite one enemy within 5 feet of it. That enemy must succeed on a Dexterity saving throw or take 4d8 Force damage.

On your later turns, you can take a Magic action to move the hound up to 30 feet.

Original layout: [[../../srd-5.2.1/pages/129|page 129]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/faithful_hound ` |
| ` implementation ` | ` dnd2024:spell_faithful_hound ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` location ` |
| ` unoccupied ` | ` true ` |
| ` visible ` | ` true ` |
| ` within_range ` | ` true ` |

#### Hound

<!-- splat:values path=splat_rule/parameters/hound -->

| Field | Value |
| --- | --- |
| ` visible_to ` | ` caster_only ` |
| ` intangible ` | ` true ` |
| ` invulnerable ` | ` true ` |
| ` truesight_feet ` | ` 30 ` |
| ` until_any ` | ` ["spell_duration", "caster_distance_exceeds_300_feet"] ` |

#### Alarm

<!-- splat:values path=splat_rule/parameters/alarm -->

| Field | Value |
| --- | --- |
| ` creature_size_min ` | ` small ` |
| ` proximity_feet ` | ` 30 ` |
| ` exemption ` | ` previously_spoken_caster_specified_password ` |
| ` effect ` | ` loud_barking ` |

#### Bite

<!-- splat:yaml path=splat_rule/parameters/bite -->

```yaml
event: caster.turn_started
target: one_enemy
reach_feet: 5
save:
  ability: dexterity
  failure:
    damage:
      dice: 4d8
      type: force
  success: no_effect
```

#### Move

<!-- splat:values path=splat_rule/parameters/move -->

| Field | Value |
| --- | --- |
| ` economy ` | ` magic_action ` |
| ` timing ` | ` later_caster_turns ` |
| ` max_feet ` | ` 30 ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  value: 8
  unit: hour
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

