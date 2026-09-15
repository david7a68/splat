---
splat_schema: 1
splat_id: 2ed70c33-8b5f-5a39-bf67-f1d52f4b6a76
splat_kind: spell
splat_key: dnd2024:spells/divine_smite
splat_name: Divine Smite
splat_origin:
  key: dnd2024:reference/spell/divine_smite
splat_sources:
- title: SRD 5.2.1
  locator: pp. 125
  url: https://www.dndbeyond.com/srd
---

# Divine Smite

## Source text

<!-- SRD page 125 -->

Level 1 Evocation (Paladin)\
Casting Time: Bonus Action, which you take immedi-\
ately after hitting a target with a Melee weapon or an\
Unarmed Strike\
Range: Self\
Component: V\
Duration: Instantaneous\
The target takes an extra 2d8 Radiant damage from\
the attack. The damage increases by 1d8 if the tar-\
get is a Fiend or an Undead.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d8 for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/125|page 125]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["paladin"] ` |
| ` components ` | ` ["verbal"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Bonus Action, which you take immediately after hitting a target with a Melee weapon or an Unarmed Strike ` |
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
| ` mechanic ` | ` dnd2024:spells/divine_smite ` |
| ` implementation ` | ` dnd2024:spell_divine_smite ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` source ` | ` triggering_attack_target ` |

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` event ` | ` attack.hit ` |
| ` weapon_kinds ` | ` ["melee_weapon", "unarmed_strike"] ` |
| ` timing ` | ` immediately_after ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
amount:
  dice:
    count: 2
    sides: 8
    add: 0
type: radiant
extra_attack_damage: true
bonus_if_type:
- fiend
- undead
type_bonus:
  dice:
    count: 1
    sides: 8
    add: 0
```

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 1 ` |
| ` damage_dice ` | ` 1 ` |
| ` sides ` | ` 8 ` |
