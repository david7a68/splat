---
splat_schema: 1
splat_id: 8de03d4f-772d-597c-a05a-098df13a53a9
splat_kind: rule
splat_key: dnd2024:rules/magic_items_wearing_and_wielding_items
splat_name: Wearing and Wielding Items
splat_origin:
  key: dnd2024:reference/pages/103
splat_sources:
- title: SRD 5.2.1
  locator: pp. 103
  url: https://www.dndbeyond.com/srd
---

# Wearing and Wielding Items

## Source text

<!-- SRD page 103 -->

Using a magic item’s properties might mean wear-\
ing or wielding it. A magic item meant to be worn\
must be donned in the intended fashion: boots go on\
feet, gloves on hands, hats and helmets on a head,\
and rings on a finger. Magic armor must be donned,\
a Shield strapped to the arm, a cloak fastened about\
the shoulders. A weapon must be held.\
In most cases, a magic item that’s meant to be\
worn can fit a creature regardless of size or build.\
Magic garments are made to be easily adjustable, or\
they magically adjust themselves to the wearer.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magic_items_wearing_and_wielding_items ` |
| ` implementation ` | ` dnd2024:rules_magic_items_wearing_and_wielding_items ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` magic_garment_fit_default ` | ` any_size_or_build ` |
| ` item_specific_overrides ` | ` true ` |

#### Required equipment positions

<!-- splat:values path=splat_rule/parameters/required_equipment_positions -->

| Field | Value |
| --- | --- |
| ` boots ` | ` feet ` |
| ` gloves ` | ` hands ` |
| ` hat ` | ` head ` |
| ` helmet ` | ` head ` |
| ` ring ` | ` finger ` |
| ` armor ` | ` donned ` |
| ` shield ` | ` strapped_to_arm ` |
| ` cloak ` | ` shoulders ` |
| ` weapon ` | ` held ` |
