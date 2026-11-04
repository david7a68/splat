---
splat_schema: 2
splat_id: 6e633ecb-2f5d-54de-9745-a598e3d5217b
splat_kind: rule
splat_key: dnd2024:rules/food_drink_and_lodging
splat_name: Food, Drink, and Lodging
splat_origin: dnd2024:reference/pages/101
splat_sources:
- SRD 5.2.1, pp. 101, 102, https://www.dndbeyond.com/srd
---

# Food, Drink, and Lodging

## Source text

<!-- SRD page 101 -->

The Food, Drink, and Lodging table gives prices for food and a single night’s lodging. Prices for daily lodging and meals are included in your lifestyle’s expenses.

**Food, Drink, and Lodging**

| Item | Cost | Item | Cost |
| --- | --- | --- | --- |
| Ale (mug) | 4 CP | Comfortable | 8 SP |
| Bread (loaf) | 2 CP | Wealthy | 2 GP |
| Cheese (wedge) | 1 SP | Aristocratic | 4 GP |
| Inn Stay per Day |  | Meal |  |
| Squalid | 7 CP | Squalid | 1 CP |
| Poor | 1 SP | Poor | 2 CP |
| Modest | 5 SP | Modest | 1 SP |

<!-- SRD page 102 -->

| Item | Cost | Item | Cost |
| --- | --- | --- | --- |
| Comfortable | 2 SP | Wine (bottle) |  |
| Wealthy | 3 SP | Common | 2 SP |
| Aristocratic | 6 SP | Fine | 10 GP |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/food_drink_and_lodging ` |
| ` implementation ` | ` dnd2024:rules_food_drink_and_lodging ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` daily_meals_and_lodging_included_in_lifestyle ` | ` true ` |

#### Food gp

<!-- splat:values path=splat_rule/parameters/food_gp -->

| Field | Value |
| --- | --- |
| ` ale_mug ` | ` 0.04 ` |
| ` bread_loaf ` | ` 0.02 ` |
| ` cheese_wedge ` | ` 0.1 ` |

#### Lodging gp per day

<!-- splat:values path=splat_rule/parameters/lodging_gp_per_day -->

| Field | Value |
| --- | --- |
| ` squalid ` | ` 0.07 ` |
| ` poor ` | ` 0.1 ` |
| ` modest ` | ` 0.5 ` |
| ` comfortable ` | ` 0.8 ` |
| ` wealthy ` | ` 2 ` |
| ` aristocratic ` | ` 4 ` |

#### Meal gp

<!-- splat:values path=splat_rule/parameters/meal_gp -->

| Field | Value |
| --- | --- |
| ` squalid ` | ` 0.01 ` |
| ` poor ` | ` 0.02 ` |
| ` modest ` | ` 0.1 ` |
| ` comfortable ` | ` 0.2 ` |
| ` wealthy ` | ` 0.3 ` |
| ` aristocratic ` | ` 0.6 ` |

#### Wine gp per bottle

<!-- splat:values path=splat_rule/parameters/wine_gp_per_bottle -->

| Field | Value |
| --- | --- |
| ` common ` | ` 0.2 ` |
| ` fine ` | ` 10 ` |
