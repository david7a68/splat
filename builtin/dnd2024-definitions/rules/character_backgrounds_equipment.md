---
splat_schema: 1
splat_id: d46db1d1-82f1-5006-a208-795a00cf1d51
splat_kind: rule
splat_key: dnd2024:rules/character_backgrounds_equipment
splat_name: Equipment
splat_origin:
  key: dnd2024:reference/pages/083
splat_sources:
- title: SRD 5.2.1
  locator: pp. 83
  url: https://www.dndbeyond.com/srd
---

# Equipment

## Source text

<!-- SRD page 83 -->

Each background offers a choice between a package of equipment and 50 GP.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/character_backgrounds_equipment ` |
| ` implementation ` | ` dnd2024:rule_character_backgrounds_equipment ` |

### Parameters

#### Choose one

<!-- splat:yaml path=splat_rule/parameters/choose_one -->

```yaml
- background_equipment_package
- currency: gp
  amount: 50
```
