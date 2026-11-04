---
splat_schema: 2
splat_id: 8ac511d2-540a-5027-8272-27e55c9c2a8d
splat_kind: item
splat_key: dnd2024:items/decanter_of_endless_water
splat_name: Decanter of Endless Water
splat_origin: dnd2024:reference/magic_item/decanter_of_endless_water
splat_sources:
- SRD 5.2.1, pp. 217, https://www.dndbeyond.com/srd
---

# Decanter of Endless Water


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Uncommon |
| Attunement | No |


## Source text

<!-- SRD page 217 -->

Wondrous Item, Uncommon

This stoppered flask sloshes when shaken, as if it contains water. The decanter weighs 2 pounds.

You can take a Magic action to remove the stopper and issue one of three command words, whereupon an amount of fresh water or salt water (your choice) pours out of the flask. The water stops pouring out at the start of your next turn. Choose from the following command words:

Splash. The decanter produces 1 gallon of water.

Fountain. The decanter produces 5 gallons of water.

Geyser. The decanter produces 30 gallons of water that gushes forth in a Line 30 feet long and 1 foot wide. If you’re holding the decanter, you can aim the geyser in one direction (no action required). One creature of your choice in the Line must succeed on a DC 13 Strength saving throw or take 1d4 Bludgeoning damage and have the Prone condition. Instead of a creature, you can target one object in the Line that isn’t being worn or carried and that weighs no more than 200 pounds. The object is knocked over by the geyser.

Original layout: [[../../srd-5.2.1/pages/217|page 217]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/decanter_of_endless_water ` |
| ` implementation ` | ` dnd2024:item_decanter_of_endless_water ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weight_lb ` | ` 2 ` |

#### Activation

<!-- splat:yaml path=splat_rule/parameters/activation -->

```yaml
action: magic
remove_stopper: true
command_word: true
water_type_choice:
- fresh
- salt
end: self_next_turn_start
options:
  splash:
    gallons: 1
  fountain:
    gallons: 5
  geyser:
    gallons: 30
    shape: line
    length_ft: 30
    width_ft: 1
    aim:
      requires: holding
      action: none
      directions: 1
    target:
      choice:
      - one_creature_in_line
      - one_object_in_line
    creature:
      save:
        ability: strength
        dc: 13
      failure:
        damage:
          roll:
            dice: 1
            sides: 4
            bonus: 0
          type: bludgeoning
        condition: prone
    object:
      worn_or_carried: false
      maximum_weight_lb: 200
      effect: knocked_over
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
