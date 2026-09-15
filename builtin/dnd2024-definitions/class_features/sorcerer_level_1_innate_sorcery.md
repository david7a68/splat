---
splat_schema: 1
splat_id: 82874b69-d75c-52b7-8ba9-ee18236be272
splat_kind: rule
splat_key: dnd2024:class_features/sorcerer_level_1_innate_sorcery
splat_name: 'Level 1: Innate Sorcery'
splat_origin:
  key: dnd2024:reference/pages/065
splat_sources:
- title: SRD 5.2.1
  locator: pp. 65, 66
  url: https://www.dndbeyond.com/srd
---

# Level 1: Innate Sorcery

## Source text

<!-- SRD page 65 -->

An event in your past left an indelible mark on you,\
infusing you with simmering magic. As a Bonus\
Action, you can unleash that magic for 1 minute,\
during which you gain the following benefits:\

<!-- SRD page 66 -->

•  The spell save DC of your Sorcerer spells\
increases by 1.\
•  You have Advantage on the attack rolls of\
Sorcerer spells you cast.\
You can use this feature twice, and you regain all\
expended uses of it when you finish a Long Rest.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/sorcerer_level_1_innate_sorcery ` |
| ` implementation ` | ` dnd2024:class_features_sorcerer_level_1_innate_sorcery ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` bonus_action ` |

#### Duration

<!-- splat:values path=splat_rule/parameters/duration -->

| Field | Value |
| --- | --- |
| ` minutes ` | ` 1 ` |

#### Uses

<!-- splat:yaml path=splat_rule/parameters/uses -->

```yaml
maximum: 2
recover:
  event: rest.long_rest_completed
  amount: all
```

#### Benefits

<!-- splat:values path=splat_rule/parameters/benefits -->

| Field | Value |
| --- | --- |
| ` sorcerer_spell_save_dc_bonus ` | ` 1 ` |
| ` sorcerer_spell_attack_advantage ` | ` true ` |
