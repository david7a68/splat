---
splat_schema: 2
splat_id: d46db1d1-82f1-5006-a208-795a00cf1d51
splat_kind: rule
splat_key: dnd2024:rules/character_backgrounds_equipment
splat_name: Equipment
splat_origin: dnd2024:reference/pages/083
splat_sources:
- SRD 5.2.1, pp. 83, https://www.dndbeyond.com/srd
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
