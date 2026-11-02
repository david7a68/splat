---
splat_schema: 1
splat_id: 33a35a43-1180-5533-92d2-9a887f91fa1c
splat_kind: spell
splat_key: dnd2024:spells/shining_smite
splat_name: Shining Smite
splat_origin:
  key: dnd2024:reference/spell/shining_smite
splat_sources:
- title: SRD 5.2.1
  locator: pp. 162
  url: https://www.dndbeyond.com/srd
---

# Shining Smite

## Source text

<!-- SRD page 162 -->

Level 2 Transmutation (Paladin)\
Casting Time: Bonus Action, which you take immediately after hitting a creature with a Melee weapon or an Unarmed Strike\
Range: Self\
Component: V\
Duration: Concentration, up to 1 minute

The target hit by the strike takes an extra 2d6 Radiant damage from the attack. Until the spell ends, the target sheds Bright Light in a 5-foot radius, attack rolls against it have Advantage, and it can’t benefit from the Invisible condition.

Using a Higher-Level Spell Slot. The damage increases by 1d6 for each spell slot level above 2.

Original layout: [[../../srd-5.2.1/pages/162|page 162]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["paladin"] ` |
| ` components ` | ` ["verbal"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Bonus Action, which you take immediately after hitting a creature with a Melee weapon or an Unarmed Strike ` |
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
| ` mechanic ` | ` dnd2024:spells/shining_smite ` |
| ` implementation ` | ` dnd2024:spell_shining_smite ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature_hit_by_triggering_melee_weapon_or_unarmed_strike ` |

#### Extra attack damage

<!-- splat:values path=splat_rule/parameters/extra_attack_damage -->

| Field | Value |
| --- | --- |
| ` dice ` | ` 2d6 ` |
| ` type ` | ` radiant ` |

#### Rider

<!-- splat:values path=splat_rule/parameters/rider -->

| Field | Value |
| --- | --- |
| ` duration ` | ` spell_duration ` |
| ` bright_light_radius_feet ` | ` 5 ` |
| ` attacks_against_target ` | ` advantage ` |
| ` invisible_condition_benefits ` | ` none ` |

#### Slot scaling

<!-- splat:values path=splat_rule/parameters/slot_scaling -->

| Field | Value |
| --- | --- |
| ` above ` | ` 2 ` |
| ` damage_per_level ` | ` 1d6 ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 1
  unit: minute
```
