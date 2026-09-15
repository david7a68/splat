---
splat_schema: 1
splat_id: becbde68-276f-5795-8d7d-f9bdba546ba7
splat_kind: rule
splat_key: dnd2024:rules/cursed_items
splat_name: Cursed Items
splat_origin:
  key: dnd2024:reference/pages/206
splat_sources:
- title: SRD 5.2.1
  locator: pp. 206
  url: https://www.dndbeyond.com/srd
---

# Cursed Items

## Source text

<!-- SRD page 206 -->

A magic item’s description specifies whether it\
bears a curse. Most methods of identifying items, in-\
cluding the Identify spell, fail to reveal such a curse.\
Attunement to a cursed item can’t be ended volun-\
tarily unless the curse is broken first, such as with a\
Remove Curse spell.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/cursed_items ` |
| ` implementation ` | ` dnd2024:rule_cursed_items ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` curse_presence_source ` | ` item_description ` |
| ` identification_reveals_curse_default ` | ` false ` |
| ` identify_spell_reveals_curse ` | ` false ` |
| ` voluntary_attunement_end_requires ` | ` curse_broken ` |

#### Curse removal example

<!-- splat:values path=splat_rule/parameters/curse_removal_example -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/remove_curse ` |
