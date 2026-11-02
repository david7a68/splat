---
splat_schema: 1
splat_id: 327b0eb2-2a5c-5a52-ad34-23315690edb5
splat_kind: rule
splat_key: dnd2024:rules/magic_item_categories_scrolls
splat_name: Scrolls
splat_origin:
  key: dnd2024:reference/pages/205
splat_sources:
- title: SRD 5.2.1
  locator: pp. 205
  url: https://www.dndbeyond.com/srd
---

# Scrolls

## Source text

<!-- SRD page 205 -->

An item in the Scroll category is a roll of paper or parchment, sometimes attached to wooden rods and typically kept safe in a tube of ivory, jade, leather, metal, or wood. The most prevalent scroll is the Spell Scroll, a spell stored in written form.

Using a Scroll. Scrolls are consumable items. Unleashing the magic in a scroll requires the user to read the scroll. When its magic has been invoked, the scroll can’t be used again. Its words fade, or it crumbles into dust.

Any creature that can understand a written language can read a scroll and attempt to activate it unless its description notes otherwise.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magic_item_categories_scrolls ` |
| ` implementation ` | ` dnd2024:rules_magic_item_categories_scrolls ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` category ` | ` scroll ` |
| ` consumable ` | ` true ` |
| ` activation_requires ` | ` read_scroll ` |
| ` reader_eligibility ` | ` understands_any_written_language ` |
| ` item_specific_eligibility_overrides ` | ` true ` |

#### On magic invoked

<!-- splat:values path=splat_rule/parameters/on_magic_invoked -->

| Field | Value |
| --- | --- |
| ` reuse_prohibited ` | ` true ` |
| ` destruction_modes ` | ` ["words_fade", "crumble_to_dust"] ` |
