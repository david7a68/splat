---
splat_schema: 2
splat_id: c10b97e2-c908-52a4-96b5-95ad3403733b
splat_kind: item
splat_key: dnd2024:items/hat_of_disguise
splat_name: Hat of Disguise
splat_origin: dnd2024:reference/magic_item/hat_of_disguise
splat_sources:
- SRD 5.2.1, pp. 224, https://www.dndbeyond.com/srd
---

# Hat of Disguise


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Uncommon |
| Attunement | Yes |


## Source text

<!-- SRD page 224 -->

Wondrous Item, Uncommon (Requires Attunement)

While wearing this hat, you can cast the Disguise Self spell. The spell ends if the hat is removed.

Original layout: [[../../srd-5.2.1/pages/224|page 224]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/hat_of_disguise ` |
| ` implementation ` | ` dnd2024:item_hat_of_disguise ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | end_on |
| --- | --- |
| ` 0 ` | ` ["hat_removed"] ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/disguise_self ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
