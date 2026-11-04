---
splat_schema: 2
splat_id: ac741708-4e9f-5266-a8d4-3eb4c79342f2
splat_kind: rule
splat_key: dnd2024:glossary/curses
splat_name: Curses
splat_origin: dnd2024:reference/glossary/curses
splat_sources:
- SRD 5.2.1, pp. 179, https://www.dndbeyond.com/srd
---

# Curses

## Source text

<!-- SRD page 179 -->

Some game effects curse a creature or an object. The effect that confers a curse defines what the curse does. Curses can be removed by the Remove Curse and Greater Restoration spells or other magic that explicitly ends curses.

Original layout: [[../../srd-5.2.1/pages/179|page 179]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/curses ` |
| ` implementation ` | ` dnd2024:glossary_curses ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` target_types ` | ` ["creature", "object"] ` |
| ` behavior ` | ` granting_effect_defined ` |
| ` other_removal_requires_explicit_curse_ending ` | ` true ` |

#### Removal sources

<!-- splat:rows path=splat_rule/parameters/removal_sources -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/remove_curse ` |
| ` 1 ` | ` dnd2024:spells/greater_restoration ` |
