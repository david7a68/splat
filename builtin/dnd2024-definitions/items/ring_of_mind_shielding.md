---
splat_schema: 1
splat_id: 63bbcc54-2d6c-5efc-8b0b-1c27e0ce900a
splat_kind: item
splat_key: dnd2024:items/ring_of_mind_shielding
splat_name: Ring of Mind Shielding
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_mind_shielding
splat_sources:
- title: SRD 5.2.1
  locator: pp. 238
  url: https://www.dndbeyond.com/srd
---

# Ring of Mind Shielding

## Source text

<!-- SRD page 238 -->

Ring, Uncommon (Requires Attunement)\
While wearing this ring, you are immune to magic\
that allows other creatures to read your thoughts,\
determine whether you are lying, know your align-\
ment, or know your creature type. Creatures can\
telepathically communicate with you only if you\
allow it.\
You can take a Magic action to cause the ring to\
become imperceptible until you take another Magic\
action to make it perceptible, until you remove the\
ring, or until you die.\
If you die while wearing the ring, your soul enters\
it, unless it already houses a soul. You can remain in\
the ring or depart for the afterlife. As long as your\
soul is in the ring, you can telepathically commu-\
nicate with any creature wearing it. A wearer can’t\
prevent this telepathic communication.\

Original layout: [[../../srd-5.2.1/pages/238|page 238]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` ring ` |
| ` rarity ` | ` uncommon ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/ring_of_mind_shielding ` |
| ` implementation ` | ` dnd2024:item_ring_of_mind_shielding ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |
| ` magic_immunities ` | ` ["read_thoughts", "detect_lie", "know_alignment", "know_creature_type"] ` |

#### Incoming telepathy

<!-- splat:values path=splat_rule/parameters/incoming_telepathy -->

| Field | Value |
| --- | --- |
| ` requires_consent ` | ` true ` |
| ` exception ` | ` resident_ring_soul ` |

#### Imperceptibility

<!-- splat:values path=splat_rule/parameters/imperceptibility -->

| Field | Value |
| --- | --- |
| ` enable_action ` | ` magic ` |
| ` end_on ` | ` ["magic_action_make_perceptible", "ring_removed", "wearer_death"] ` |

#### Soul

<!-- splat:yaml path=splat_rule/parameters/soul -->

```yaml
trigger: wearer_dies
requires: ring_has_no_soul
effect: soul_enters_ring
choice:
- remain
- depart_afterlife
resident_telepathy:
  target: current_wearer
  wearer_cannot_block: true
capacity: 1
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
