---
splat_schema: 1
splat_id: e587bfe6-4f89-5d14-8c0c-6300ea328e86
splat_kind: rule
splat_key: dnd2024:rules/magic_item_categories_potions
splat_name: Potions
splat_origin:
  key: dnd2024:reference/pages/204
splat_sources:
- title: SRD 5.2.1
  locator: pp. 204
  url: https://www.dndbeyond.com/srd
---

# Potions

## Source text

<!-- SRD page 204 -->

An item in the Potion category might be a magical brew that must be imbibed or an oil that must be applied to a creature or an object. A typical potion consists of 1 ounce of liquid in a vial.

Using a Potion. Potions are consumable items. Drinking a potion or administering it to another creature requires a Bonus Action. Applying an oil might take longer as specified in its description. Once used, a potion takes effect immediately, and it is used up.

Mixing Potions. A character might drink one potion while still under the effects of another or pour several potions into a single container. The strange ingredients used in creating potions can result in unpredictable interactions.

When a character mixes two potions together, roll on the Potion Miscibility table. If more than two are combined, roll again for each subsequent potion, combining the results. Unless the effects are immediately obvious, reveal them only when they become evident.

**Potion Miscibility**

| 1d100 | Result |
| --- | --- |
| 01 | Both potions lose their effects, and the mixture creates a magical explosion in a 5-foot-radius Sphere centered on itself. Each creature in that area takes 4d10 Force damage. |
| 02–08 | Both potions lose their effects, and the mixture becomes an ingested poison of your choice (see “Poison” in “Gameplay Toolbox”). |
| 09–15 | Both potions lose their effects. |
| 16–25 | One potion loses its effect. |
| 26–35 | Both potions work, but with their numerical effects and durations halved. If a potion has no numerical effect and no duration, it instead loses its effect. |
| 36–90 | Both potions work normally. |
| 91–99 | Both potions work, but the numerical effects and duration of one potion are doubled. If neither potion has anything to double in this way, they work normally. |
| 00 | Only one potion works, but its effects are permanent. Choose the simplest effect to make permanent or the one that seems the most fun. For example, a Potion of Healing might increase the drinker’s Hit Point maximum by 2d4 + 2, or a Potion of Invisibility might give the drinker the Invisible condition indefinitely. At your discretion, a Dispel Magic spell or similar magic might end this lasting effect. |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magic_item_categories_potions ` |
| ` implementation ` | ` dnd2024:rules_magic_item_categories_potions ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` category ` | ` potion ` |
| ` typical_volume_ounces ` | ` 1 ` |
| ` consumable ` | ` true ` |

#### Activation

<!-- splat:values path=splat_rule/parameters/activation -->

| Field | Value |
| --- | --- |
| ` drink ` | ` bonus_action ` |
| ` administer_to_other ` | ` bonus_action ` |
| ` oil_application ` | ` item_specific_time ` |
| ` effect_starts ` | ` immediately ` |
| ` consumed_on_use ` | ` true ` |

#### Mixing

<!-- splat:yaml path=splat_rule/parameters/mixing -->

```yaml
triggers:
- combine_in_container
- drink_while_other_potion_effect_active
initial_potions: 2
extra_rolls: one_per_additional_potion
combine_results: true
reveal_hidden_results: when_evident
die:
  count: 1
  sides: 100
table:
- min: 1
  max: 1
  effects:
    both_potions_lose_effects: true
    explosion:
      magical: true
      shape: sphere
      radius_ft: 5
      origin: mixture
      damage:
        dice:
          count: 4
          sides: 10
        type: force
      save: none
- min: 2
  max: 8
  effects:
    both_potions_lose_effects: true
    replace_with_poison:
      route: ingested
      choice_authority: gm
- min: 9
  max: 15
  effects:
    both_potions_lose_effects: true
- min: 16
  max: 25
  effects:
    one_potion_loses_effect: true
    potion_selection: gm_choice
- min: 26
  max: 35
  effects:
    both_work: true
    numerical_effect_multiplier: 0.5
    duration_multiplier: 0.5
    if_no_numerical_effect_and_no_duration: potion_loses_effect
- min: 36
  max: 90
  effects:
    both_work_normally: true
- min: 91
  max: 99
  effects:
    both_work: true
    one_potion_numerical_effect_multiplier: 2
    same_potion_duration_multiplier: 2
    potion_selection: gm_choice
    if_neither_has_doublable_property: both_work_normally
- min: 100
  max: 100
  effects:
    working_potions: 1
    potion_selection: gm_choice
    selected_effect_duration: permanent
    effect_selection:
      authority: gm
      priorities:
      - simplest
      - most_fun
    examples:
    - potion: healing
      permanent_effect: hp_max_increase
      amount:
        dice:
          count: 2
          sides: 4
        add: 2
    - potion: invisibility
      permanent_effect: invisible_condition
    dispel_magic_or_similar_may_end:
      authority: gm
      choice: boolean
```
