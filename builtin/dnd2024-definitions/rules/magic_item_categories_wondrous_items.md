---
splat_schema: 2
splat_id: 28874c76-55dc-53ef-9452-3ac84e31d8b8
splat_kind: rule
splat_key: dnd2024:rules/magic_item_categories_wondrous_items
splat_name: Wondrous Items
splat_origin: dnd2024:reference/pages/205
splat_sources:
- SRD 5.2.1, pp. 205, https://www.dndbeyond.com/srd
---

# Wondrous Items

## Source text

<!-- SRD page 205 -->

Wondrous Items include wearable items such as boots, belts, capes, amulets, brooches, and circlets. Bags, carpets, figurines, horns, musical instruments, and more also fall into this category.

### Magic Item Rules

Rules for identifying, attuning to, and using magic items appear in “Equipment.” Additional rules are presented below.

Attunement Prerequisites. If a magic item has a class prerequisite, a creature must be a member of that class to attune to the item. If a creature must be a spellcaster to attune to an item, the creature qualifies if it can cast at least one spell using its traits or features, not by using a magic item or the like.

Items Made for Specific Creatures. Magic items that are meant to be worn tend to magically adjust themselves to the wearer. However, you can decide that a magic item doesn’t adjust its size to fit any wearer. For example, a particular armorer might make items usable only by folk who are sized and shaped like dwarves.

Unusual Anatomy. Use your discretion to decide whether a creature can wear an item not made for its anatomy. A ring placed on a tentacle might work, but a yuan-ti with a snakelike tail instead of legs can’t wear magic boots.

Paired Items. You can allow exceptions to the rule that paired items must both be worn. For example, a character with only one arm might be able to use a single Glove of Missile Snaring so long as the matching glove is on their person.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
| ` implementation ` | ` dnd2024:rule_magic_item_categories_wondrous_items ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |

#### Attunement prerequisites

<!-- splat:yaml path=splat_rule/parameters/attunement_prerequisites -->

```yaml
class: membership_in_required_class
spellcaster:
  minimum_spells: 1
  allowed_sources:
  - traits
  - features
  excluded_sources:
  - magic_items
  - similar_external_sources
```

#### Wearable size adjustment

<!-- splat:values path=splat_rule/parameters/wearable_size_adjustment -->

| Field | Value |
| --- | --- |
| ` default ` | ` true ` |
| ` gm_can_disable ` | ` true ` |
| ` gm_fit_restrictions ` | ` ["body_size", "body_shape"] ` |

#### Unusual anatomy fit

<!-- splat:values path=splat_rule/parameters/unusual_anatomy_fit -->

| Field | Value |
| --- | --- |
| ` actor ` | ` gm ` |
| ` scope ` | ` item_not_made_for_anatomy ` |
| ` must_have_required_body_parts ` | ` true ` |

#### Paired items

<!-- splat:yaml path=splat_rule/parameters/paired_items -->

```yaml
default: both_worn
gm_exception:
  scope: missing_body_part
  other_item_remains_on_person: true
```
