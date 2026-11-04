---
splat_schema: 2
splat_id: 49f81995-bc49-58d8-b972-06c8e6b43f24
splat_kind: item
splat_key: dnd2024:items/ring_of_regeneration
splat_name: Ring of Regeneration
splat_origin: dnd2024:reference/magic_item/ring_of_regeneration
splat_sources:
- SRD 5.2.1, pp. 238, https://www.dndbeyond.com/srd
---

# Ring of Regeneration


## Item

| Field | Value |
| --- | --- |
| Type | Ring |
| Rarity | Very Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 238 -->

Ring, Very Rare (Requires Attunement)

While wearing this ring, you regain 1d6 Hit Points every 10 minutes if you have at least 1 Hit Point. If you lose a body part, the ring causes the missing part to regrow and return to full functionality after 1d6 + 1 days if you have at least 1 Hit Point the whole time.

Original layout: [[../../srd-5.2.1/pages/238|page 238]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/ring_of_regeneration ` |
| ` implementation ` | ` dnd2024:item_ring_of_regeneration ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` attunement_required ` | ` true ` |

#### Healing

<!-- splat:yaml path=splat_rule/parameters/healing -->

```yaml
amount:
  roll:
    count: 1
    sides: 6
interval:
  amount: 10
  unit: minutes
requires_hp_minimum: 1
```

#### Regrow part

<!-- splat:yaml path=splat_rule/parameters/regrow_part -->

```yaml
trigger: body_part_lost
duration:
  amount:
    roll:
      count: 1
      sides: 6
      add: 1
  unit: days
requires_hp_minimum_throughout: 1
full_functionality: true
```
