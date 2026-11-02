---
splat_schema: 1
splat_id: f07a831d-667a-58d5-892d-a1f6f2ff5157
splat_kind: item
splat_key: dnd2024:items/robe_of_the_archmagi
splat_name: Robe of the Archmagi
splat_origin:
  key: dnd2024:reference/magic_item/robe_of_the_archmagi
splat_sources:
- title: SRD 5.2.1
  locator: pp. 240
  url: https://www.dndbeyond.com/srd
---

# Robe of the Archmagi

## Source text

<!-- SRD page 240 -->

Wondrous Item, Legendary (Requires Attunement by a Sorcerer, Warlock, or Wizard)

This elegant garment is made from exquisite cloth and adorned with runes.

You gain these benefits while wearing the robe.

Armor. If you aren’t wearing armor, your base Armor Class is 15 plus your Dexterity modifier.

Magic Resistance. You have Advantage on saving throws against spells and other magical effects.

War Mage. Your spell save DC and spell attack bonus each increase by 2.

Original layout: [[../../srd-5.2.1/pages/240|page 240]].

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
| ` attunement_requirement_text ` | ` a Sorcerer, Warlock, or Wizard ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/robe_of_the_archmagi ` |
| ` implementation ` | ` dnd2024:item_robe_of_the_archmagi ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` attunement_classes ` | ` ["sorcerer", "warlock", "wizard"] ` |
| ` spell_save_dc_bonus ` | ` 2 ` |
| ` spell_attack_bonus ` | ` 2 ` |
| ` attunement_required ` | ` true ` |

#### Armor class

<!-- splat:yaml path=splat_rule/parameters/armor_class -->

```yaml
requires_no_armor: true
base:
  add:
  - 15
  - dexterity_modifier
```

#### Saving throw advantage

<!-- splat:values path=splat_rule/parameters/saving_throw_advantage -->

| Field | Value |
| --- | --- |
| ` against ` | ` ["spells", "other_magical_effects"] ` |
