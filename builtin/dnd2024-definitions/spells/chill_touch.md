---
splat_schema: 2
splat_id: 828fb932-2962-5fb5-928f-2fd5e0ea624b
splat_kind: spell
splat_key: dnd2024:spells/chill_touch
splat_name: Chill Touch
splat_origin: dnd2024:reference/spell/chill_touch
splat_sources:
- SRD 5.2.1, pp. 115, https://www.dndbeyond.com/srd
---

# Chill Touch


## Spell

| Field | Value |
| --- | --- |
| Level | Cantrip |
| School | Necromancy |
| Classes | Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S |
| Duration | Instantaneous |


## Source text

<!-- SRD page 115 -->

Necromancy Cantrip (Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Instantaneous

Channeling the chill of the grave, make a melee spell attack against a target within reach. On a hit, the target takes 1d10 Necrotic damage, and it can’t regain Hit Points until the end of your next turn.

Cantrip Upgrade. The damage increases by 1d10 when you reach levels 5 (2d10), 11 (3d10), and 17 (4d10).

Original layout: [[../../srd-5.2.1/pages/115|page 115]].





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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: touch
> ```

