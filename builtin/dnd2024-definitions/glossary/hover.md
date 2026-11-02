---
splat_schema: 1
splat_id: e9563af3-b932-5670-9fdf-d4b1221e188f
splat_kind: rule
splat_key: dnd2024:glossary/hover
splat_name: Hover
splat_origin:
  key: dnd2024:reference/glossary/hover
splat_sources:
- title: SRD 5.2.1
  locator: pp. 183
  url: https://www.dndbeyond.com/srd
---

# Hover

## Source text

<!-- SRD page 183 -->

Some creatures can hover, as noted in their stat blocks, and some spells and other effects grant the ability to hover. Hovering while flying prevents you from falling in certain circumstances. See also “Flying.”

Original layout: [[../../srd-5.2.1/pages/183|page 183]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/hover ` |
| ` implementation ` | ` dnd2024:glossary_hover ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` grant_sources ` | ` ["stat_block", "spell", "other_effect"] ` |
| ` while_flying_prevents_fall_from ` | ` ["incapacitated", "prone", "fly_speed_zero"] ` |

#### Base rule

<!-- splat:values path=splat_rule/parameters/base_rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:glossary/flying ` |
