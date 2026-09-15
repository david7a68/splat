---
splat_schema: 1
splat_id: d8b984a1-d4d8-527a-a4d3-583135d21cc8
splat_kind: item
splat_key: dnd2024:items/scarab_of_protection
splat_name: Scarab of Protection
splat_origin:
  key: dnd2024:reference/magic_item/scarab_of_protection
splat_sources:
- title: SRD 5.2.1
  locator: pp. 243
  url: https://www.dndbeyond.com/srd
---

# Scarab of Protection

## Source text

<!-- SRD page 243 -->

Wondrous Item, Legendary (Requires Attunement)\
This beetle-shaped medallion provides three bene-\
fits while it is on your person.\
Defense. You gain a +1 bonus to Armor Class.\
Preservation. The scarab has 12 charges. If you\
fail a saving throw against a Necromancy spell or\
a harmful effect originating from an Undead, you\
can take a Reaction to expend 1 charge and turn the\
failed save into a successful one. The scarab crum-\
bles into powder and is destroyed when its last\
charge is expended.\
Spell Resistance. You have Advantage on saving\
throws against spells.\

Original layout: [[../../srd-5.2.1/pages/243|page 243]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
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
| ` mechanic ` | ` dnd2024:items/scarab_of_protection ` |
| ` implementation ` | ` dnd2024:item_scarab_of_protection ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` on_person ` |
| ` armor_class_bonus ` | ` 1 ` |
| ` saving_throws_against_spells ` | ` advantage ` |

#### Charges

<!-- splat:values path=splat_rule/parameters/charges -->

| Field | Value |
| --- | --- |
| ` initial ` | ` 12 ` |
| ` maximum ` | ` 12 ` |
| ` recovery ` | ` null ` |
| ` on_zero ` | ` destroy_item ` |

#### Preservation

<!-- splat:yaml path=splat_rule/parameters/preservation -->

```yaml
action: reaction
trigger: failed_saving_throw
eligible_effect:
  any_of:
  - spell_school: necromancy
  - origin_type: undead
    harmful: true
cost: 1
replace_result: success
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
