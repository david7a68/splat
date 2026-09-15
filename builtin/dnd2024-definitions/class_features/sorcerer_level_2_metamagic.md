---
splat_schema: 1
splat_id: df3c746f-db36-5799-8b04-e777c124986e
splat_kind: rule
splat_key: dnd2024:class_features/sorcerer_level_2_metamagic
splat_name: 'Level 2: Metamagic'
splat_origin:
  key: dnd2024:reference/pages/066
splat_sources:
- title: SRD 5.2.1
  locator: pp. 66
  url: https://www.dndbeyond.com/srd
---

# Level 2: Metamagic

## Source text

<!-- SRD page 66 -->

Because your magic flows from within, you can\
alter your spells to suit your needs; you gain two\
Metamagic options of your choice from “Meta magic\
Options” later in this class’s description. You use\
the chosen options to temporarily modify spells you\
cast. To use an option, you must spend the number\
of Sorcery Points that it costs.\
You can use only one Metamagic option on a spell\
when you cast it unless otherwise noted in one of\
those options.\
Whenever you gain a Sorcerer level, you can re-\
place one of your Metamagic options with one you\
don’t know. You gain two more options at Sorcerer\
level 10 and two more at Sorcerer level 17.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/sorcerer_level_2_metamagic ` |
| ` implementation ` | ` dnd2024:class_features_sorcerer_level_2_metamagic ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` pay_option_sorcery_point_cost ` | ` true ` |
| ` maximum_options_per_cast ` | ` 1 ` |
| ` option_specific_stacking_exception_allowed ` | ` true ` |

#### Metamagic choices

<!-- splat:yaml path=splat_rule/parameters/metamagic_choices -->

```yaml
count_by_sorcerer_level:
  '2': 2
  '10': 4
  '17': 6
source:
  key: dnd2024:rules/sorcerer_metamagic_options
replace:
  event: character.sorcerer_level_gained
  maximum: 1
  new_option_must_be_unknown: true
```
