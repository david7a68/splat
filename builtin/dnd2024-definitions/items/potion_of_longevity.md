---
splat_schema: 1
splat_id: aa54d0e4-b2a7-5377-ba9c-76deb3551e2b
splat_kind: item
splat_key: dnd2024:items/potion_of_longevity
splat_name: Potion of Longevity
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_longevity
splat_sources:
- title: SRD 5.2.1
  locator: pp. 236
  url: https://www.dndbeyond.com/srd
---

# Potion of Longevity

## Source text

<!-- SRD page 236 -->

Potion, Very Rare

When you drink this potion, your physical age is reduced by 1d6 + 6 years, to a minimum of 13 years. Each time you subsequently drink a Potion of Longevity, there is 10 percent cumulative chance that you instead age by 1d6 + 6 years.

Suspended in this amber liquid is a tiny heart that, against all reason, is still beating. These ingredients vanish when the potion is opened.

Original layout: [[../../srd-5.2.1/pages/236|page 236]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` potion ` |
| ` rarity ` | ` very_rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/potion_of_longevity ` |
| ` implementation ` | ` dnd2024:item_potion_of_longevity ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` consume ` | ` drink ` |
| ` default_direction ` | ` decrease ` |
| ` minimum_age_years ` | ` 13 ` |
| ` consume_action ` | ` bonus_action ` |
| ` administer_action ` | ` bonus_action ` |
| ` effect_target ` | ` drinker ` |
| ` consumed_quantity ` | ` 1 ` |
| ` effect_starts ` | ` immediately_after_consumption ` |

#### Physical age change years

<!-- splat:values path=splat_rule/parameters/physical_age_change_years -->

| Field | Value |
| --- | --- |
| ` expression ` | ` 1d6+6 ` |
| ` scope ` | ` use ` |

#### Subsequent use

<!-- splat:yaml path=splat_rule/parameters/subsequent_use -->

```yaml
track: lifetime_prior_potions_of_longevity_consumed
aging_probability_percent:
  op: min
  args:
  - 100
  - op: multiply
    args:
    - 10
    - binding: prior_consumptions
on_probability_success: increase_age_instead
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_potions ` |
