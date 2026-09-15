---
splat_schema: 1
splat_id: 59db096b-3ba9-530b-9514-f7084d964f31
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_damage_rolls
splat_name: Damage Rolls
splat_origin:
  key: dnd2024:reference/pages/016
splat_sources:
- title: SRD 5.2.1
  locator: pp. 16
  url: https://www.dndbeyond.com/srd
---

# Damage Rolls

## Source text

<!-- SRD page 16 -->

Each weapon, spell, and damaging monster ability\
specifies the damage it deals. You roll the damage\
dice, add any modifiers, and deal the damage to\
your target. If there’s a penalty to the damage, it’s\
possible to deal 0 damage but not negative damage.\
When attacking with a weapon, you add your abil-\
ity modifier—the same modifier used for the attack\
roll—to the damage roll. A spell tells you which dice\
to roll for damage and whether to add any modi-\
fiers. Unless a rule says otherwise, you don’t add\
your ability modifier to a fixed damage amount that\
doesn’t use a roll, such as the damage of a Blowgun.\
See “Equipment” for weapons’ damage dice and\
“Spells” for spells’ damage dice.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/damage_and_healing_damage_rolls ` |
| ` implementation ` | ` dnd2024:rules_damage_and_healing_damage_rolls ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weapon_roll_ability_modifier ` | ` same_as_attack ` |
| ` spell_modifiers ` | ` specified_by_spell ` |
| ` fixed_damage_ability_modifier_default ` | ` false ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
op: max
terms:
- 0
- op: sum
  terms:
  - damage_dice
  - modifiers
```
