---
splat_schema: 1
splat_id: cda544df-8444-5a1d-810a-e75823ae0f9f
splat_kind: item
splat_key: dnd2024:items/ring_of_spell_turning
splat_name: Ring of Spell Turning
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_spell_turning
splat_sources:
- title: SRD 5.2.1
  locator: pp. 239
  url: https://www.dndbeyond.com/srd
---

# Ring of Spell Turning

## Source text

<!-- SRD page 239 -->

Ring, Legendary (Requires Attunement)

While wearing this ring, you have Advantage on saving throws against spells. If you succeed on the save for a spell of level 7 or lower, the spell has no effect on you. If that spell targeted only you and didn’t create an area of effect, you can take a Reaction to deflect the spell back at the spell’s caster; the caster must make a saving throw against the spell using their own spell save DC.

Original layout: [[../../srd-5.2.1/pages/239|page 239]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` ring ` |
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
| ` mechanic ` | ` dnd2024:items/ring_of_spell_turning ` |
| ` implementation ` | ` dnd2024:item_ring_of_spell_turning ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |
| ` saving_throws_against_spells ` | ` advantage ` |

#### Successful save

<!-- splat:yaml path=splat_rule/parameters/successful_save -->

```yaml
maximum_spell_level: 7
spell_effect_on_user: none
reflection:
  action: reaction
  requires:
    only_target: user
    area_of_effect: false
  target: original_caster
  save_dc: original_caster_spell_save_dc
  save_against: same_spell
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
