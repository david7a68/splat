---
splat_schema: 1
splat_id: 3073ecdd-0616-53e2-bf40-6218a391d1f3
splat_kind: item
splat_key: dnd2024:items/wand_of_secrets
splat_name: Wand of Secrets
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_secrets
splat_sources:
- title: SRD 5.2.1
  locator: pp. 251
  url: https://www.dndbeyond.com/srd
---

# Wand of Secrets

## Source text

<!-- SRD page 251 -->

Wand, Uncommon

This wand has 3 charges and regains 1d3 expended charges daily at dawn. While holding it, you can take a Magic action to expend 1 charge, and if a secret door or trap is within 60 feet of you, the wand pulses and points at the one nearest to you.

Original layout: [[../../srd-5.2.1/pages/251|page 251]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wand ` |
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
| ` mechanic ` | ` dnd2024:items/wand_of_secrets ` |
| ` implementation ` | ` dnd2024:item_wand_of_secrets ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` held ` |
| ` action ` | ` magic ` |
| ` charge_cost ` | ` 1 ` |
| ` attunement_required ` | ` false ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
initial: 3
maximum: 3
recovery:
  event: time.dawn
  amount:
    roll:
      count: 1
      sides: 3
  cap_at_maximum: true
```

#### Detection

<!-- splat:values path=splat_rule/parameters/detection -->

| Field | Value |
| --- | --- |
| ` types ` | ` ["secret_door", "trap"] ` |
| ` range_ft ` | ` 60 ` |
| ` target_selection ` | ` nearest ` |
| ` indication ` | ` ["pulse", "point_to_target"] ` |
| ` none_in_range ` | ` no_indication ` |
