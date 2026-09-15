---
splat_schema: 1
splat_id: 74a0b5bb-55c1-5610-9fe2-6e65ae5109b4
splat_kind: item
splat_key: dnd2024:items/ring_of_the_ram
splat_name: Ring of the Ram
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_the_ram
splat_sources:
- title: SRD 5.2.1
  locator: pp. 239
  url: https://www.dndbeyond.com/srd
---

# Ring of the Ram

## Source text

<!-- SRD page 239 -->

Ring, Rare (Requires Attunement)\
This ring has 3 charges and regains 1d3 expended\
charges daily at dawn. While wearing the ring, you\
can take a Magic action to expend 1 to 3 charges to\
make a ranged spell attack against one creature you\
can see within 60 feet of yourself. The ring produces\
a spectral ram’s head and makes its attack roll with\
a +7 bonus. On a hit, for each charge you spend, the\
target takes 2d10 Force damage and is pushed 5\
feet away from you.\
Alternatively, you can expend 1 to 3 of the ring’s\
charges as a Magic action to try to break a nonmag-\
ical object you can see within 60 feet of yourself\
that isn’t being worn or carried. The ring makes\
a Strength check with a +5 bonus for each charge\
you spend.\

Original layout: [[../../srd-5.2.1/pages/239|page 239]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` ring ` |
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
| ` mechanic ` | ` dnd2024:items/ring_of_the_ram ` |
| ` implementation ` | ` dnd2024:item_ring_of_the_ram ` |

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

#### Activation

<!-- splat:yaml path=splat_rule/parameters/activation -->

```yaml
action: magic
charge_choice:
  minimum: 1
  maximum: 3
options:
  creature_attack:
    target:
      kind: creature
      seen: true
      range_ft: 60
    attack:
      kind: ranged_spell
      bonus: 7
    on_hit:
      damage:
        dice_per_charge: 2
        sides: 10
        type: force
      push:
        ft_per_charge: 5
        direction: away_from_user
  object_break:
    target:
      kind: object
      nonmagical: true
      worn_or_carried: false
      seen: true
      range_ft: 60
    check:
      ability: strength
      bonus_per_charge: 5
      dc_from: object_break_dc
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
