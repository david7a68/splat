---
splat_schema: 1
splat_id: 6e5c3914-970e-52ef-b80e-96ff9880e804
splat_kind: rule
splat_key: dnd2024:glossary/improvised_weapons
splat_name: Improvised Weapons
splat_origin:
  key: dnd2024:reference/glossary/improvised_weapons
splat_sources:
- title: SRD 5.2.1
  locator: pp. 183
  url: https://www.dndbeyond.com/srd
---

# Improvised Weapons

## Source text

<!-- SRD page 183 -->

An improvised weapon is an object wielded as a\
makeshift weapon, such as broken glass, a table leg,\
or a frying pan. A Simple or Martial weapon also\
counts as an improvised weapon if it’s wielded in\
a way contrary to its design; if you use a Ranged\
weapon to make a melee attack or throw a Me-\
lee weapon that lacks the Thrown property, the\
weapon counts as an improvised weapon. An impro-\
vised weapon follows the rules below.\
Proficiency. Don’t add your Proficiency Bonus to\
attack rolls with an improvised weapon.\
Damage. On a hit, the weapon deals 1d4 damage\
of a type the GM thinks is appropriate for the object.\
Range. If you throw the weapon, it has a normal\
range of 20 feet and a long range of 60 feet.\
Weapon Equivalents. If an improvised weapon\
resembles a Simple or Martial weapon, the GM may\
say it functions as that weapon and uses that weap-\
on’s rules. For example, the GM could treat a table\
leg as a Club.\

Original layout: [[../../srd-5.2.1/pages/183|page 183]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/improvised_weapons ` |
| ` implementation ` | ` dnd2024:glossary_improvised_weapons ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` qualifies_any ` | ` ["object_used_as_makeshift_weapon", "ranged_weapon_used_for_melee", "melee_weapon_without_thrown_property_thrown", "weapon_used_contrary_to_design"] ` |
| ` proficiency_bonus_added ` | ` false ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
dice: 1d4
type_choice:
  chooser: gm
  options:
  - acid
  - bludgeoning
  - cold
  - fire
  - force
  - lightning
  - necrotic
  - piercing
  - poison
  - psychic
  - radiant
  - slashing
  - thunder
  constraint: appropriate_for_object
```

#### Thrown range feet

<!-- splat:values path=splat_rule/parameters/thrown_range_feet -->

| Field | Value |
| --- | --- |
| ` normal ` | ` 20 ` |
| ` long ` | ` 60 ` |

#### Weapon equivalent

<!-- splat:values path=splat_rule/parameters/weapon_equivalent -->

| Field | Value |
| --- | --- |
| ` optional ` | ` true ` |
| ` chooser ` | ` gm ` |
| ` requires_resemblance ` | ` true ` |
| ` allowed_categories ` | ` ["simple", "martial"] ` |
| ` replacement_rules ` | ` selected_weapon_all_rules ` |
