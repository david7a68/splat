---
splat_schema: 1
splat_id: 7dcd9ca9-ad7d-50da-87aa-c639841dd906
splat_kind: item
splat_key: dnd2024:items/luck_blade
splat_name: Luck Blade
splat_origin:
  key: dnd2024:reference/magic_item/luck_blade
splat_sources:
- title: SRD 5.2.1
  locator: pp. 229
  url: https://www.dndbeyond.com/srd
---

# Luck Blade

## Source text

<!-- SRD page 229 -->

Weapon (Glaive, Greatsword, Longsword, Rapier, Scimitar, Sickle, or Shortsword), Legendary (Requires Attunement)

You gain a +1 bonus to attack rolls and damage rolls made with this magic weapon. While the weapon is on your person, you also gain a +1 bonus to saving throws.

Luck. If the weapon is on your person, you can call on its luck (no action required) to reroll one failed D20 Test if you don’t have the Incapacitated condition. You must use the second roll. Once used, this property can’t be used again until the next dawn.

Wish. The weapon has 1d3 charges. While holding it, you can expend 1 charge and cast Wish from it. Once used, this property can’t be used again until the next dawn. The weapon loses this property if it has no charges.

Original layout: [[../../srd-5.2.1/pages/229|page 229]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` weapon ` |
| ` rarity ` | ` legendary ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/luck_blade ` |
| ` implementation ` | ` dnd2024:item_luck_blade ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attack_roll_bonus ` | ` 1 ` |
| ` damage_roll_bonus ` | ` 1 ` |
| ` scope ` | ` attacks_with_source_item ` |

#### On person

<!-- splat:yaml path=splat_rule/parameters/on_person -->

```yaml
saving_throw_bonus: 1
luck:
  action: none
  requires_not_condition: incapacitated
  trigger: failed_d20_test
  reroll: true
  must_use_second: true
  uses:
    maximum: 1
    initial: 1
    recovery:
      event: dawn
      amount:
        expression: '1'
        scope: use
      cap: maximum
```

#### Wish

<!-- splat:yaml path=splat_rule/parameters/wish -->

```yaml
equipment_state: held
charges:
  initial:
    expression: 1d3
    scope: use
  recovery: null
spell_grant:
  spell:
    key: dnd2024:spells/wish
  charge_cost: 1
  casting_time: spell_default
  method: item
  slot_cost: 0
  components: []
  level: lowest_possible
  range: spell_default
  duration: spell_default
  concentration: spell_default
uses_per_dawn: 1
on_zero_charges: lose_wish_property
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_weapons ` |
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
