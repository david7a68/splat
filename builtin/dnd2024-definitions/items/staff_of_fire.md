---
splat_schema: 1
splat_id: 5e744cdd-c403-5296-9197-622968899bba
splat_kind: item
splat_key: dnd2024:items/staff_of_fire
splat_name: Staff of Fire
splat_origin:
  key: dnd2024:reference/magic_item/staff_of_fire
splat_sources:
- title: SRD 5.2.1
  locator: pp. 245
  url: https://www.dndbeyond.com/srd
---

# Staff of Fire

## Source text

<!-- SRD page 245 -->

Staff, Very Rare (Requires Attunement by a Druid,\
Sorcerer, Warlock, or Wizard)\
You have Resistance to Fire damage while you hold\
this staff.\
Spells. The staff has 10 charges. While holding the\
staff, you can cast one of the spells on the following\
table from it, using your spell save DC. The table in-\
dicates how many charges you must expend to cast\
the spell.\
Charge Charge\
Spell Cost Spell Cost\
Burning Hands 1 Wall of Fire 4\
Fireball 3\
Regaining Charges. The staff regains 1d6 + 4\
expended charges daily at dawn. If you expend the\
last charge, roll 1d20. On a 1, the staff crumbles into\
cinders and is destroyed.\

Original layout: [[../../srd-5.2.1/pages/245|page 245]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` staff ` |
| ` rarity ` | ` very_rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |
| ` attunement_requirement_text ` | ` a Druid, Sorcerer, Warlock, or Wizard ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/staff_of_fire ` |
| ` implementation ` | ` dnd2024:item_staff_of_fire ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` held ` |
| ` damage_resistance ` | ` fire ` |
| ` spell_save_dc ` | ` user_spell_save_dc ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
maximum: 10
initial: 10
recovery:
  event: dawn
  amount:
    expression: 1d6+4
    scope: use
  cap: maximum
```

#### Spell grants

<!-- splat:table
path: splat_rule/parameters/spell_grants
key: Spell
order:
- burning_hands
- wall_of_fire
- fireball
columns:
  Spell:
    path: spell
    type: reference
  Charge cost:
    path: charge_cost
    type: integer
  Casting time:
    path: casting_time
    type: symbol
  Method:
    path: method
    type: symbol
  Slot cost:
    path: slot_cost
    type: integer
  Components:
    path: components
    type: yaml
  Level:
    path: level
    type: symbol
  Range:
    path: range
    type: symbol
  Duration:
    path: duration
    type: symbol
  Concentration:
    path: concentration
    type: symbol
-->

| Spell | Charge cost | Casting time | Method | Slot cost | Components | Level | Range | Duration | Concentration |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| [Burning Hands](<../spells/burning_hands.md> "dnd2024:spells/burning_hands") | ` 1 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Lowest possible ` | ` Spell default ` | ` Spell default ` | ` Spell default ` |
| [Wall of Fire](<../spells/wall_of_fire.md> "dnd2024:spells/wall_of_fire") | ` 4 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Lowest possible ` | ` Spell default ` | ` Spell default ` | ` Spell default ` |
| [Fireball](<../spells/fireball.md> "dnd2024:spells/fireball") | ` 3 ` | ` Spell default ` | ` Item ` | ` 0 ` | ` [] ` | ` Lowest possible ` | ` Spell default ` | ` Spell default ` | ` Spell default ` |

#### Last charge

<!-- splat:yaml path=splat_rule/parameters/last_charge -->

```yaml
roll:
  expression: 1d20
  scope: use
'on': 1
effect: destroyed
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
| ` eligible_classes ` | ` ["druid", "sorcerer", "warlock", "wizard"] ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_staffs ` |
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
