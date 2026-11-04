---
splat_schema: 2
splat_id: 96942747-6e7d-5727-af60-b822a8017e8e
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_1_martial_arts
splat_name: 'Level 1: Martial Arts'
splat_origin: dnd2024:reference/pages/050
splat_sources:
- SRD 5.2.1, pp. 50, https://www.dndbeyond.com/srd
---

# Level 1: Martial Arts

## Source text

<!-- SRD page 50 -->

Your practice of martial arts gives you mastery of combat styles that use your Unarmed Strike and Monk weapons, which are the following:

•  Simple Melee weapons

•  Martial Melee weapons that have the Light property

You gain the following benefits while you are unarmed or wielding only Monk weapons and you aren’t wearing armor or wielding a Shield.

Bonus Unarmed Strike. You can make an Unarmed Strike as a Bonus Action.

Martial Arts Die. You can roll 1d6 in place of the normal damage of your Unarmed Strike or Monk weapons. This die changes as you gain Monk levels, as shown in the Martial Arts column of the Monk Features table.

Dexterous Attacks. You can use your Dexterity modifier instead of your Strength modifier for the attack and damage rolls of your Unarmed Strikes and Monk weapons. In addition, when you use the Grapple or Shove option of your Unarmed Strike, you can use your Dexterity modifier instead of your Strength modifier to determine the save DC.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/monk_level_1_martial_arts ` |
| ` implementation ` | ` dnd2024:class_features_monk_level_1_martial_arts ` |

### Parameters

#### Monk weapon categories

<!-- splat:table
path: splat_rule/parameters/monk_weapon_categories
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Category:
    path: category
    type: symbol
  Range:
    path: range
    type: symbol
  Requires property:
    path: requires_property
    type: symbol
    optional: true
-->

| Key | Category | Range | Requires property |
| --- | --- | --- | --- |
| ` row_1 ` | ` Simple ` | ` Melee ` |  |
| ` row_2 ` | ` Martial ` | ` Melee ` | ` Light ` |

#### Benefit requirements

<!-- splat:values path=splat_rule/parameters/benefit_requirements -->

| Field | Value |
| --- | --- |
| ` armor_worn ` | ` false ` |
| ` shield_wielded ` | ` false ` |
| ` weapons_wielded ` | ` none_or_only_monk_weapons ` |

#### Bonus unarmed strike

<!-- splat:values path=splat_rule/parameters/bonus_unarmed_strike -->

| Field | Value |
| --- | --- |
| ` action ` | ` bonus_action ` |
| ` count ` | ` 1 ` |

#### Damage die substitution

<!-- splat:yaml path=splat_rule/parameters/damage_die_substitution -->

```yaml
optional: true
attacks:
- unarmed_strike
- monk_weapon
dice_by_monk_level:
  '1':
    dice:
      count: 1
      sides: 6
  '5':
    dice:
      count: 1
      sides: 8
  '11':
    dice:
      count: 1
      sides: 10
  '17':
    dice:
      count: 1
      sides: 12
```

#### Dexterity substitution

<!-- splat:values path=splat_rule/parameters/dexterity_substitution -->

| Field | Value |
| --- | --- |
| ` optional ` | ` true ` |
| ` replaces ` | ` strength ` |
| ` for ` | ` ["unarmed_strike_attack", "unarmed_strike_damage", "monk_weapon_attack", "monk_weapon_damage", "unarmed_grapple_save_dc", "unarmed_shove_save_dc"] ` |
