---
splat_schema: 1
splat_id: 80a04f46-be96-5809-b662-9b2bdaeee518
splat_kind: item
splat_key: dnd2024:items/oil_of_slipperiness
splat_name: Oil of Slipperiness
splat_origin:
  key: dnd2024:reference/magic_item/oil_of_slipperiness
splat_sources:
- title: SRD 5.2.1
  locator: pp. 233, 234
  url: https://www.dndbeyond.com/srd
---

# Oil of Slipperiness

## Source text

<!-- SRD page 233 -->

Potion, Uncommon

One vial of this oil can cover one Medium or smaller creature, along with the equipment it’s wearing and carrying (one additional vial is required for each size category above Medium). Applying the oil takes 10 minutes. The affected creature then gains the effect of the Freedom of Movement spell for 8 hours.

Alternatively, the oil can be poured on the ground as a Magic action, where it covers a 10-foot square, duplicating the effect of the Grease spell in that area for 8 hours.

<!-- SRD page 234 -->

This sticky, black unguent is thick and heavy, but it flows quickly when poured.

Original layout: [[../../srd-5.2.1/pages/233|page 233]], [[../../srd-5.2.1/pages/234|page 234]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` potion ` |
| ` rarity ` | ` uncommon ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/oil_of_slipperiness ` |
| ` implementation ` | ` dnd2024:item_oil_of_slipperiness ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` consume ` | ` apply_or_pour ` |

#### Apply

<!-- splat:yaml path=splat_rule/parameters/apply -->

```yaml
duration:
  amount: 10
  unit: minute
target: creature_and_worn_carried_equipment
vials_by_size:
  tiny: 1
  small: 1
  medium: 1
  large: 2
  huge: 3
  gargantuan: 4
spell_effect:
  key: dnd2024:spells/freedom_of_movement
duration_effect:
  amount: 8
  unit: hour
```

#### Pour

<!-- splat:yaml path=splat_rule/parameters/pour -->

```yaml
action: magic
area:
  shape: square
  side_ft: 10
spell_effect:
  key: dnd2024:spells/grease
duration_effect:
  amount: 8
  unit: hour
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_potions ` |
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
