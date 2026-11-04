---
splat_schema: 2
splat_id: 8934f3f1-6941-500a-ac0d-1149bb97959b
splat_kind: item
splat_key: dnd2024:items/gem_of_seeing
splat_name: Gem of Seeing
splat_origin: dnd2024:reference/magic_item/gem_of_seeing
splat_sources:
- SRD 5.2.1, pp. 223, https://www.dndbeyond.com/srd
---

# Gem of Seeing


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 223 -->

Wondrous Item, Rare (Requires Attunement)

This gem has 3 charges. As a Magic action, you can expend 1 charge. For the next 10 minutes, you have Truesight out to 120 feet when you peer through the gem.

The gem regains 1d3 expended charges daily at dawn.

Original layout: [[../../srd-5.2.1/pages/223|page 223]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/gem_of_seeing ` |
| ` implementation ` | ` dnd2024:item_gem_of_seeing ` |

### Parameters

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
maximum: 3
initial: 3
recovery:
  event: dawn
  amount:
    dice: 1
    sides: 3
    bonus: 0
```

#### Activation

<!-- splat:yaml path=splat_rule/parameters/activation -->

```yaml
action: magic
cost: 1
duration:
  value: 10
  unit: minute
truesight_ft: 120
requires_during_effect: peer_through_gem
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
