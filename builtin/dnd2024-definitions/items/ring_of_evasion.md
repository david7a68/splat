---
splat_schema: 2
splat_id: 1c31d842-028f-537a-8aa5-09a959077099
splat_kind: item
splat_key: dnd2024:items/ring_of_evasion
splat_name: Ring of Evasion
splat_origin: dnd2024:reference/magic_item/ring_of_evasion
splat_sources:
- SRD 5.2.1, pp. 238, https://www.dndbeyond.com/srd
---

# Ring of Evasion


## Item

| Field | Value |
| --- | --- |
| Type | Ring |
| Rarity | Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 238 -->

Ring, Rare (Requires Attunement)

This ring has 3 charges, and it regains 1d3 expended charges daily at dawn. When you fail a Dexterity saving throw while wearing the ring, you can take a Reaction to expend 1 charge to succeed on that save instead.

Original layout: [[../../srd-5.2.1/pages/238|page 238]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/ring_of_evasion ` |
| ` implementation ` | ` dnd2024:item_ring_of_evasion ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn_on_finger_or_similar_digit ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
maximum: 3
initial: 3
recovery:
  event: dawn
  amount:
    expression: 1d3
    scope: use
  cap: maximum
```

#### Reaction

<!-- splat:values path=splat_rule/parameters/reaction -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` failed_dexterity_saving_throw ` |
| ` charge_cost ` | ` 1 ` |
| ` replace_outcome ` | ` success ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_rings ` |
