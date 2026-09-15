---
splat_schema: 1
splat_id: ae0c00ab-82fb-57bf-aebd-52aba223b012
splat_kind: spell
splat_key: dnd2024:spells/true_strike
splat_name: True Strike
splat_origin:
  key: dnd2024:reference/spell/true_strike
splat_sources:
- title: SRD 5.2.1
  locator: pp. 171
  url: https://www.dndbeyond.com/srd
---

# True Strike

## Source text

<!-- SRD page 171 -->

Divination Cantrip (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Self\
Components: S, M (a weapon with which you have profi-\
ciency and that is worth 1+ CP)\
Duration: Instantaneous\
Guided by a flash of magical insight, you make one\
attack with the weapon used in the spell’s casting.\
The attack uses your spellcasting ability for the at-\
tack and damage rolls instead of using Strength or\
Dexterity. If the attack deals damage, it can be Ra-\
diant damage or the weapon’s normal damage type\
(your choice).\
Cantrip Upgrade. Whether you deal Radiant dam-\
age or the weapon’s normal damage type, the attack\
deals extra Radiant damage when you reach levels 5\
(1d6), 11 (2d6), and 17 (3d6).\

Original layout: [[../../srd-5.2.1/pages/171|page 171]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 0 ` |
| ` school ` | ` divination ` |
| ` classes ` | ` ["bard", "sorcerer", "warlock", "wizard"] ` |
| ` components ` | ` ["somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a weapon with which you have proficiency and that is worth 1+ CP ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/true_strike ` |
| ` implementation ` | ` dnd2024:spell_true_strike ` |

### Parameters

#### Attack

<!-- splat:yaml path=splat_rule/parameters/attack -->

```yaml
count: 1
weapon: casting_material_weapon
attack_ability: spellcasting
damage_ability: spellcasting
replace_abilities:
- strength
- dexterity
damage_type_choice:
- radiant
- weapon_normal
extra_damage:
  type: radiant
  dice:
    count:
      by_caster_level:
      - minimum: 1
        count: 0
      - minimum: 5
        count: 1
      - minimum: 11
        count: 2
      - minimum: 17
        count: 3
    sides: 6
    add: 0
  applies_regardless_of_chosen_damage_type: true
```
