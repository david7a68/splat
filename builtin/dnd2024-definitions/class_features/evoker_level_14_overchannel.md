---
splat_schema: 2
splat_id: 200e5608-4d85-5036-b8e6-c0b00009f9ce
splat_kind: rule
splat_key: dnd2024:class_features/evoker_level_14_overchannel
splat_name: 'Level 14: Overchannel'
splat_origin: dnd2024:reference/pages/082
splat_sources:
- SRD 5.2.1, pp. 82, https://www.dndbeyond.com/srd
---

# Level 14: Overchannel

## Source text

<!-- SRD page 82 -->

You can increase the power of your spells. When you cast a Wizard spell with a spell slot of levels 1–5 that deals damage, you can deal maximum damage with that spell on the turn you cast it.

The first time you do so, you suffer no adverse effect. If you use this feature again before you finish a Long Rest, you take 2d12 Necrotic damage for each level of the spell slot immediately after you cast it. This damage ignores Resistance and Immunity.

Each time you use this feature again before finishing a Long Rest, the Necrotic damage per spell level increases by 1d12.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/evoker_level_14_overchannel ` |
| ` implementation ` | ` dnd2024:class_features_evoker_level_14_overchannel ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` actor.casts_damaging_wizard_spell ` |
| ` requires_slot_level ` | ` [1, 2, 3, 4, 5] ` |
| ` optional ` | ` true ` |
| ` spell_damage ` | ` maximum ` |
| ` spell_damage_scope ` | ` cast_turn ` |

#### Usage counter

<!-- splat:values path=splat_rule/parameters/usage_counter -->

| Field | Value |
| --- | --- |
| ` id ` | ` wizard.overchannel ` |
| ` increment_each_use ` | ` 1 ` |
| ` reset ` | ` rest.long_completed ` |

#### Backlash

<!-- splat:values path=splat_rule/parameters/backlash -->

| Field | Value |
| --- | --- |
| ` when_use_number_minimum ` | ` 2 ` |
| ` timing ` | ` immediately_after_cast ` |
| ` ignores ` | ` ["resistance", "immunity"] ` |

##### Damage

<!-- splat:values path=splat_rule/parameters/backlash/damage -->

| Field | Value |
| --- | --- |
| ` type ` | ` necrotic ` |

###### Dice

<!-- splat:values path=splat_rule/parameters/backlash/damage/dice -->

| Field | Value |
| --- | --- |
| ` sides ` | ` 12 ` |

###### Count

<!-- splat:values path=splat_rule/parameters/backlash/damage/dice/count -->

| Field | Value |
| --- | --- |
| ` op ` | ` multiply ` |

###### Args

<!-- splat:table
path: splat_rule/parameters/backlash/damage/dice/count/args
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Var:
    path: var
    type: string
-->

| Key | Var |
| --- | --- |
| ` row_1 ` | ` slot.level ` |
| ` row_2 ` | ` overchannel.use_number ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` wizard ` |
| ` minimum_level ` | ` 14 ` |
