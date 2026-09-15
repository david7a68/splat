---
splat_schema: 1
splat_id: e483c744-8b02-5b76-b952-e1203c8eb034
splat_kind: rule
splat_key: dnd2024:rules/warlock_pact_of_the_blade
splat_name: Pact of the Blade
splat_origin:
  key: dnd2024:reference/pages/074
splat_sources:
- title: SRD 5.2.1
  locator: pp. 74
  url: https://www.dndbeyond.com/srd
---

# Pact of the Blade

## Source text

<!-- SRD page 74 -->

As a Bonus Action, you can conjure a pact weapon\
in your hand—a Simple or Martial Melee weapon of\
your choice with which you bond—or create a bond\
with a magic weapon you touch; you can’t bond with\
a magic weapon if someone else is attuned to it or\
another Warlock is bonded with it. Until the bond\
ends, you have proficiency with the weapon, and\
you can use it as a Spellcasting Focus.\
Whenever you attack with the bonded weapon,\
you can use your Charisma modifier for the attack\
and damage rolls instead of using Strength or\
Dexterity; and you can cause the weapon to deal\
Necrotic, Psychic, or Radiant damage or its normal\
damage type.\
Your bond with the weapon ends if you use this\
feature’s Bonus Action again, if the weapon is more\
than 5 feet away from you for 1 minute or more, or\
if you die. A conjured weapon disappears when the\
bond ends.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_pact_of_the_blade ` |
| ` implementation ` | ` dnd2024:rule_warlock_pact_of_the_blade ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` activation ` | ` bonus_action ` |
| ` conjured_weapon_disappears_when_bond_ends ` | ` true ` |

#### Options

<!-- splat:yaml path=splat_rule/parameters/options -->

```yaml
conjure:
  location: hand
  weapon_categories:
  - simple
  - martial
  weapon_kind: melee
  choice: warlock
bond_existing:
  weapon_magic: true
  touch_required: true
  prohibited_if:
  - attuned_to_someone_else
  - bonded_to_another_warlock
```

#### Bond benefits

<!-- splat:values path=splat_rule/parameters/bond_benefits -->

| Field | Value |
| --- | --- |
| ` weapon_proficiency ` | ` true ` |
| ` spellcasting_focus ` | ` true ` |
| ` optional_attack_and_damage_ability ` | ` charisma ` |
| ` damage_type_choice ` | ` ["normal", "necrotic", "psychic", "radiant"] ` |

#### Bond ends when

<!-- splat:yaml path=splat_rule/parameters/bond_ends_when -->

```yaml
- activate_feature_again
- warlock_dies
- distance_feet_greater_than: 5
  continuous_minutes: 1
```
