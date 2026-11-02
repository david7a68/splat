---
splat_schema: 1
splat_id: f0c38a4c-3aae-574b-834e-fe869289d1e4
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_roll_two_d20s
splat_name: Roll Two D20s
splat_origin:
  key: dnd2024:reference/pages/008
splat_sources:
- title: SRD 5.2.1
  locator: pp. 8
  url: https://www.dndbeyond.com/srd
---

# Roll Two D20s

## Source text

<!-- SRD page 8 -->

When a roll has either Advantage or Disadvantage, roll a second d20 when you make the roll. Use the higher of the two rolls if you have Advantage, and use the lower roll if you have Disadvantage. For example, if you have Disadvantage and roll an 18 and a 3, use the 3. If you instead have Advantage and roll those numbers, use the 18.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/d20_tests_roll_two_d20s ` |
| ` implementation ` | ` dnd2024:rules_d20_tests_roll_two_d20s ` |

### Parameters

#### Roll

<!-- splat:values path=splat_rule/parameters/roll -->

| Field | Value |
| --- | --- |
| ` count ` | ` 2 ` |
| ` sides ` | ` 20 ` |
| ` add ` | ` 0 ` |

#### Keep

<!-- splat:values path=splat_rule/parameters/keep -->

| Field | Value |
| --- | --- |
| ` advantage ` | ` highest ` |
| ` disadvantage ` | ` lowest ` |
