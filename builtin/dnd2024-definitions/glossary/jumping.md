---
splat_schema: 1
splat_id: cde6ea23-885b-5ef4-a0f4-848d6d0447bd
splat_kind: rule
splat_key: dnd2024:glossary/jumping
splat_name: Jumping
splat_origin:
  key: dnd2024:reference/glossary/jumping
splat_sources:
- title: SRD 5.2.1
  locator: pp. 184
  url: https://www.dndbeyond.com/srd
---

# Jumping

## Source text

<!-- SRD page 184 -->

When you jump, you make either a Long Jump (hor-\
izontal) or a High Jump (vertical). See also “Long\
Jump” and “High Jump.”\

Original layout: [[../../srd-5.2.1/pages/184|page 184]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/jumping ` |
| ` implementation ` | ` dnd2024:glossary_jumping ` |

### Parameters

#### Select one

<!-- splat:table
path: splat_rule/parameters/select_one
key: Rule
order:
- long_jump
- high_jump
columns:
  Rule:
    path: rule
    type: reference
  Direction:
    path: direction
    type: symbol
-->

| Rule | Direction |
| --- | --- |
| [Long Jump](<long_jump.md> "dnd2024:glossary/long_jump") | ` Horizontal ` |
| [High Jump](<high_jump.md> "dnd2024:glossary/high_jump") | ` Vertical ` |
