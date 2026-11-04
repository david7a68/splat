---
splat_schema: 2
splat_id: ae0c00ab-82fb-57bf-aebd-52aba223b012
splat_kind: spell
splat_key: dnd2024:spells/true_strike
splat_name: True Strike
splat_origin: dnd2024:reference/spell/true_strike
splat_sources:
- SRD 5.2.1, pp. 171, https://www.dndbeyond.com/srd
---

# True Strike


## Spell

| Field | Value |
| --- | --- |
| Level | Cantrip |
| School | Divination |
| Classes | Bard, Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range |  |
| Components | S, M (a weapon with which you have proficiency and that is worth 1+ CP) |
| Duration | Instantaneous |


## Source text

<!-- SRD page 171 -->

Divination Cantrip (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Self\
Components: S, M (a weapon with which you have proficiency and that is worth 1+ CP)\
Duration: Instantaneous

Guided by a flash of magical insight, you make one attack with the weapon used in the spell’s casting. The attack uses your spellcasting ability for the attack and damage rolls instead of using Strength or Dexterity. If the attack deals damage, it can be Radiant damage or the weapon’s normal damage type (your choice).

Cantrip Upgrade. Whether you deal Radiant damage or the weapon’s normal damage type, the attack deals extra Radiant damage when you reach levels 5 (1d6), 11 (2d6), and 17 (3d6).

Original layout: [[../../srd-5.2.1/pages/171|page 171]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/true_strike ` |
| ` implementation ` | ` dnd2024:spell_true_strike ` |

### Parameters

#### Attack

<!-- splat:values path=splat_rule/parameters/attack -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` weapon ` | ` casting_material_weapon ` |
| ` attack_ability ` | ` spellcasting ` |
| ` damage_ability ` | ` spellcasting ` |
| ` replace_abilities ` | ` ["strength", "dexterity"] ` |
| ` damage_type_choice ` | ` ["radiant", "weapon_normal"] ` |

##### Extra damage

<!-- splat:values path=splat_rule/parameters/attack/extra_damage -->

| Field | Value |
| --- | --- |
| ` type ` | ` radiant ` |
| ` applies_regardless_of_chosen_damage_type ` | ` true ` |

###### Dice

<!-- splat:values path=splat_rule/parameters/attack/extra_damage/dice -->

| Field | Value |
| --- | --- |
| ` sides ` | ` 6 ` |
| ` add ` | ` 0 ` |

###### Count

###### By caster level

<!-- splat:table
path: splat_rule/parameters/attack/extra_damage/dice/count/by_caster_level
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Minimum:
    path: minimum
    type: integer
  Count:
    path: count
    type: integer
-->

| Key | Minimum | Count |
| --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 0 ` |
| ` row_2 ` | ` 5 ` | ` 1 ` |
| ` row_3 ` | ` 11 ` | ` 2 ` |
| ` row_4 ` | ` 17 ` | ` 3 ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: self
> ```

