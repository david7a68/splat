---
splat_schema: 1
splat_id: 6c2d30e1-4f6e-58d9-969d-cc0b02db8430
splat_kind: spell
splat_key: dnd2024:spells/mislead
splat_name: Mislead
splat_origin:
  key: dnd2024:reference/spell/mislead
splat_sources:
- title: SRD 5.2.1
  locator: pp. 150
  url: https://www.dndbeyond.com/srd
---

# Mislead

## Source text

<!-- SRD page 150 -->

Level 5 Illusion (Bard, Warlock, Wizard)\
Casting Time: Action\
Range: Self\
Components: S\
Duration: Concentration, up to 1 hour

You gain the Invisible condition at the same time that an illusory double of you appears where you are standing. The double lasts for the duration, but the invisibility ends immediately after you make an attack roll, deal damage, or cast a spell.

As a Magic action, you can move the illusory double up to twice your Speed and make it gesture, speak, and behave in whatever way you choose. It is intangible and invulnerable.

You can see through its eyes and hear through its ears as if you were located where it is.

Original layout: [[../../srd-5.2.1/pages/150|page 150]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 5 ` |
| ` school ` | ` illusion ` |
| ` classes ` | ` ["bard", "warlock", "wizard"] ` |
| ` components ` | ` ["somatic"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 hour ` |
| ` casting_time ` | ` Action ` |
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
| ` mechanic ` | ` dnd2024:spells/mislead ` |
| ` implementation ` | ` dnd2024:spell_mislead ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` self ` |

#### Invisible

<!-- splat:values path=splat_rule/parameters/invisible -->

| Field | Value |
| --- | --- |
| ` condition ` | ` invisible ` |
| ` duration ` | ` spell_duration ` |
| ` ends_immediately_after ` | ` ["caster_attack_roll", "caster_damage_dealt", "caster_spell_cast"] ` |
| ` double_ends_with_invisibility ` | ` false ` |

#### Double

<!-- splat:yaml path=splat_rule/parameters/double -->

```yaml
spawn_location: caster_space_at_cast
duration: spell_duration
intangible: true
invulnerable: true
control:
  economy: magic_action
  movement_max:
    op: multiply
    args:
    - 2
    - stat: caster_speed
  behaviors:
  - gesture
  - speak
  - caster_chosen_behavior
shared_senses:
  channels:
  - vision
  - hearing
  origin: double_location
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 1
  unit: hour
```
