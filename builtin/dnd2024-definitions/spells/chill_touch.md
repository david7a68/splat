---
splat_schema: 1
splat_id: 828fb932-2962-5fb5-928f-2fd5e0ea624b
splat_kind: spell
splat_key: dnd2024:spells/chill_touch
splat_name: Chill Touch
splat_origin:
  key: dnd2024:reference/spell/chill_touch
splat_sources:
- title: SRD 5.2.1
  locator: pp. 115
  url: https://www.dndbeyond.com/srd
---

# Chill Touch

## Source text

<!-- SRD page 115 -->

Necromancy Cantrip (Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Instantaneous\
Channeling the chill of the grave, make a melee spell\
attack against a target within reach. On a hit, the\
target takes 1d10 Necrotic damage, and it can’t re-\
gain Hit Points until the end of your next turn.\
Cantrip Upgrade. The damage increases by 1d10\
when you reach levels 5 (2d10), 11 (3d10), and 17\
(4d10).\

Original layout: [[../../srd-5.2.1/pages/115|page 115]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 0 ` |
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
| ` mode ` | ` touch ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/chill_touch ` |
| ` implementation ` | ` dnd2024:spell_chill_touch ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` target ` |
| ` within_reach ` | ` true ` |

#### Attack

<!-- splat:yaml path=splat_rule/parameters/attack -->

```yaml
kind: melee_spell
hit:
  damage:
    dice: 1d10
    type: necrotic
  cannot_regain_hp:
    until: caster_next_turn_end
```

#### Cantrip scaling

<!-- splat:yaml path=splat_rule/parameters/cantrip_scaling -->

```yaml
character_level_dice:
  '1': 1d10
  '5': 2d10
  '11': 3d10
  '17': 4d10
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```
