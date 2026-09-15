---
splat_schema: 1
splat_id: b2e653d1-62be-5c31-b6cd-e16c63e1ced0
splat_kind: spell
splat_key: dnd2024:spells/finger_of_death
splat_name: Finger of Death
splat_origin:
  key: dnd2024:reference/spell/finger_of_death
splat_sources:
- title: SRD 5.2.1
  locator: pp. 131
  url: https://www.dndbeyond.com/srd
---

# Finger of Death

## Source text

<!-- SRD page 131 -->

Level 7 Necromancy (Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous\
You unleash negative energy toward a creature you\
can see within range. The target makes a Constitu-\
tion saving throw, taking 7d8 + 30 Necrotic damage\
on a failed save or half as much damage on a suc-\
cessful one.\
A Humanoid killed by this spell rises at the start\
of your next turn as a Zombie (see “Monsters”) that\
follows your verbal orders.\

Original layout: [[../../srd-5.2.1/pages/131|page 131]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 7 ` |
| ` school ` | ` necromancy ` |
| ` classes ` | ` ["sorcerer", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/finger_of_death ` |
| ` implementation ` | ` dnd2024:spell_finger_of_death ` |

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

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
failure:
  damage:
    dice: 7d8
    add: 30
    type: necrotic
success:
  damage_fraction:
    numerator: 1
    denominator: 2
```

#### Killed humanoid

<!-- splat:values path=splat_rule/parameters/killed_humanoid -->

| Field | Value |
| --- | --- |
| ` event ` | ` caster.next_turn_started ` |
| ` form ` | ` zombie ` |
| ` obeys ` | ` caster_verbal_orders ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```
