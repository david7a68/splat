---
splat_schema: 1
splat_id: 046fba86-2d45-5ef1-9c04-dc66e7ec1760
splat_kind: item
splat_key: dnd2024:items/helm_of_teleportation
splat_name: Helm of Teleportation
splat_origin:
  key: dnd2024:reference/magic_item/helm_of_teleportation
splat_sources:
- title: SRD 5.2.1
  locator: pp. 226
  url: https://www.dndbeyond.com/srd
---

# Helm of Teleportation

## Source text

<!-- SRD page 226 -->

Wondrous Item, Rare (Requires Attunement)\
This helm has 3 charges. While wearing it, you can\
expend 1 charge to cast Teleport from it. The helm\
regains 1d3 expended charges daily at dawn.\

Original layout: [[../../srd-5.2.1/pages/226|page 226]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/helm_of_teleportation ` |
| ` implementation ` | ` dnd2024:item_helm_of_teleportation ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

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

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | cost |
| --- | --- |
| ` 0 ` | ` 1 ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/teleport ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
