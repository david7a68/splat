---
splat_schema: 1
splat_id: b15ddebc-afe0-5607-bc5d-e8d4ce651a27
splat_kind: rule
splat_key: dnd2024:glossary/armor_class
splat_name: Armor Class
splat_origin:
  key: dnd2024:reference/glossary/armor_class
splat_sources:
- title: SRD 5.2.1
  locator: pp. 177
  url: https://www.dndbeyond.com/srd
---

# Armor Class

## Source text

<!-- SRD page 177 -->

An Armor Class (AC) is the target number for an\
attack roll. AC represents how difficult it is to hit a\
target.\
Your base AC calculation is 10 plus your Dexterity\
modifier. If a rule gives you another base AC calcula-\
tion, you choose which calculation to use; you can’t\
use more than one. See also “Attack Roll.”\

Original layout: [[../../srd-5.2.1/pages/177|page 177]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/armor_class ` |
| ` implementation ` | ` dnd2024:glossary_armor_class ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attack_target_number ` | ` armor_class ` |

#### Default base

<!-- splat:yaml path=splat_rule/parameters/default_base -->

```yaml
op: add
args:
- 10
- var: dexterity_modifier
```

#### Alternative base

<!-- splat:values path=splat_rule/parameters/alternative_base -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` subject ` |
| ` options ` | ` available_base_ac_calculations ` |
| ` select_count ` | ` 1 ` |
| ` stack ` | ` false ` |
