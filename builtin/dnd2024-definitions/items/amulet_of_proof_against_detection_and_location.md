---
splat_schema: 2
splat_id: 95cf6ee6-059f-5661-8aeb-127435d11c94
splat_kind: item
splat_key: dnd2024:items/amulet_of_proof_against_detection_and_location
splat_name: Amulet of Proof against Detection and Location
splat_origin: dnd2024:reference/magic_item/amulet_of_proof_against_detection_and_location
splat_sources:
- SRD 5.2.1, pp. 209, https://www.dndbeyond.com/srd
---

# Amulet of Proof against Detection and Location


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Uncommon |
| Attunement | Yes |


## Source text

<!-- SRD page 209 -->

Wondrous Item, Uncommon (Requires Attunement)

While wearing this amulet, you can’t be targeted by Divination spells or perceived through magical scrying sensors unless you allow it.

Original layout: [[../../srd-5.2.1/pages/209|page 209]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/amulet_of_proof_against_detection_and_location ` |
| ` implementation ` | ` dnd2024:item_amulet_of_proof_against_detection_and_location ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

#### Prevent

<!-- splat:yaml path=splat_rule/parameters/prevent -->

```yaml
spell_targeting:
  school: divination
perception_by: magical_scrying_sensors
```

#### Exception

<!-- splat:values path=splat_rule/parameters/exception -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` wearer ` |
| ` allow ` | ` true ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
