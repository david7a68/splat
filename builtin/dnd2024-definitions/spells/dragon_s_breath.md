---
splat_schema: 1
splat_id: cc701950-d55b-5b07-bea0-f54931e8b41e
splat_kind: spell
splat_key: dnd2024:spells/dragon_s_breath
splat_name: Dragon’s Breath
splat_origin:
  key: dnd2024:reference/spell/dragon_s_breath
splat_sources:
- title: SRD 5.2.1
  locator: pp. 126
  url: https://www.dndbeyond.com/srd
---

# Dragon’s Breath

## Source text

<!-- SRD page 126 -->

Level 2 Transmutation (Sorcerer, Wizard)\
Casting Time: Bonus Action\
Range: Touch\
Components: V, S, M (a hot pepper)\
Duration: Concentration, up to 1 minute

You touch one willing creature, and choose Acid, Cold, Fire, Lightning, or Poison. Until the spell ends, the target can take a Magic action to exhale a 15-foot Cone. Each creature in that area makes a Dexterity saving throw, taking 3d6 damage of the chosen type on a failed save or half as much damage on a successful one.

Using a Higher-Level Spell Slot. The damage increases by 1d6 for each spell slot level above 2.

Original layout: [[../../srd-5.2.1/pages/126|page 126]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Bonus Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a hot pepper ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/dragon_s_breath ` |
| ` implementation ` | ` dnd2024:spell_dragon_s_breath ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_choice ` | ` ["acid", "cold", "fire", "lightning", "poison"] ` |

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` willing ` | ` true ` |
| ` touch ` | ` true ` |

#### Granted action

<!-- splat:yaml path=splat_rule/parameters/granted_action -->

```yaml
duration: spell_duration
economy: magic_action
area:
  shape: cone
  length_feet: 15
  origin: target
targets: all_creatures_in_area
save:
  ability: dexterity
  dc: caster_spell_save_dc
  failure:
    damage:
      dice: 3d6
      type: chosen_type
  success:
    damage_fraction:
      numerator: 1
      denominator: 2
```

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
